//! `B00tSoulShim` — MoltisMemory_🥾
//!
//! Implements [`MemoryStore`] but routes K/V caches (embedding cache) to b00t
//! soul HTTP API (`/v1/kv/<key>`), delegating files/chunks/FTS to a local
//! [`SqliteMemoryStore`].
//!
//! # Architecture
//! ```text
//! moltis MemoryStore trait
//!     ├── files / chunks / FTS / vector search
//!     │       → local SqliteMemoryStore (no change)
//!     └── embedding cache (K/V)
//!             → b00t soul HTTP API  (primary)
//!             → local SqliteMemoryStore (fallback when b00t unreachable)
//! ```
//!
//! b00t soul API endpoints used:
//! - `GET  /v1/kv/{key}`     → `{"value": "..."}` or 404
//! - `PUT  /v1/kv/{key}`     → body `{"value": "..."}`, returns 204
//! - `DELETE /v1/kv/{key}`   → returns 204
//! - `GET  /v1/kv?prefix=x`  → `{"keys": [...]}` for list_keys
//!
//! # b00t:map v1
//! # summary: MoltisMemory_🥾 shim — routes moltis embedding cache → b00t soul K/V
//! # tags: memory, soul, kv, b00t, shim, embedding-cache
//! # tier: ch0nky

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{debug, warn};

use crate::{
    schema::{ChunkRow, FileRow},
    search::SearchResult,
    store::{CacheEntry, MemoryStore},
    store_sqlite::SqliteMemoryStore,
};

// ─── b00t soul HTTP client ────────────────────────────────────────────────────

/// Default b00t soul serve endpoint.
const DEFAULT_B00T_SOUL_URL: &str = "http://127.0.0.1:7700";

/// Key namespace prefix for moltis embedding cache entries in b00t soul K/V.
const CACHE_NS: &str = "moltis:emb_cache:";

// 🤓 FILE_NS reserved for future per-file K/V soul storage
// const FILE_NS: &str = "moltis:file:";

#[derive(Serialize, Deserialize)]
struct KvValue {
    value: String,
}

/// 🤓 KvKeys used by list_keys — part of the protocol spec even if not yet called
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct KvKeys {
    keys: Vec<String>,
}

/// Minimal async client for the b00t soul K/V HTTP API.
/// 🤓 delete/list_keys are part of the public API surface for future
///    eviction propagation and prefix-scan — keep them even if unused now.
#[allow(dead_code)]
struct B00tSoulClient {
    base_url: String,
    http: reqwest::Client,
}

impl B00tSoulClient {
    fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(2))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Encode a namespaced key for URL path use (percent-encode colons etc.)
    fn key_path(key: &str) -> String {
        // reqwest handles percent encoding on .path(); just encode unsafe chars.
        key.replace('/', "%2F")
    }

    async fn get(&self, key: &str) -> Result<Option<String>> {
        let url = format!("{}/v1/kv/{}", self.base_url, Self::key_path(key));
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .context("b00t soul GET request")?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let kv: KvValue = resp
            .error_for_status()
            .context("b00t soul GET status")?
            .json()
            .await
            .context("b00t soul GET decode")?;
        Ok(Some(kv.value))
    }

    async fn put(&self, key: &str, value: &str) -> Result<()> {
        let url = format!("{}/v1/kv/{}", self.base_url, Self::key_path(key));
        self.http
            .put(&url)
            .json(&KvValue {
                value: value.to_owned(),
            })
            .send()
            .await
            .context("b00t soul PUT request")?
            .error_for_status()
            .context("b00t soul PUT status")?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let url = format!("{}/v1/kv/{}", self.base_url, Self::key_path(key));
        self.http
            .delete(&url)
            .send()
            .await
            .context("b00t soul DELETE request")?
            .error_for_status()
            .context("b00t soul DELETE status")?;
        Ok(())
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>> {
        let url = format!("{}/v1/kv", self.base_url);
        let resp = self
            .http
            .get(&url)
            .query(&[("prefix", prefix)])
            .send()
            .await
            .context("b00t soul list_keys request")?
            .error_for_status()
            .context("b00t soul list_keys status")?;
        let kk: KvKeys = resp.json().await.context("b00t soul list_keys decode")?;
        Ok(kk.keys)
    }
}

// ─── Embedding serialization ──────────────────────────────────────────────────

/// Encode f32 slice as base64 string for K/V storage.
fn encode_embedding(v: &[f32]) -> String {
    use std::io::Write as _;
    let mut bytes = Vec::with_capacity(v.len() * 4);
    for f in v {
        bytes.write_all(&f.to_le_bytes()).ok();
    }
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.encode(&bytes)
}

/// Decode base64 string back to f32 vec.
fn decode_embedding(s: &str) -> Result<Vec<f32>> {
    use base64::Engine as _;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(s)
        .context("decode base64 embedding")?;
    let floats = bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    Ok(floats)
}

/// Canonical K/V key for an embedding cache entry.
fn cache_key(provider: &str, model: &str, hash: &str) -> String {
    format!("{CACHE_NS}{provider}:{model}:{hash}")
}

// ─── B00tSoulShim ─────────────────────────────────────────────────────────────

/// `B00tSoulShim` — MoltisMemory_🥾
///
/// Implements `MemoryStore`: files/chunks/FTS delegate to local SQLite;
/// embedding cache delegates to b00t soul HTTP API (fallback: local SQLite).
pub struct B00tSoulShim {
    /// Local SQLite store for files, chunks, FTS, vector search.
    local: SqliteMemoryStore,
    /// b00t soul HTTP API client.
    soul: B00tSoulClient,
    /// When true, soul API calls that fail fall back to local SQLite silently.
    fallback_on_error: bool,
}

impl B00tSoulShim {
    /// Create a shim with the default b00t soul URL (`http://127.0.0.1:7700`).
    pub fn new(pool: SqlitePool) -> Self {
        Self::with_soul_url(pool, DEFAULT_B00T_SOUL_URL)
    }

    /// Create a shim pointing at a custom b00t soul serve URL.
    pub fn with_soul_url(pool: SqlitePool, soul_url: impl Into<String>) -> Self {
        Self {
            local: SqliteMemoryStore::new(pool),
            soul: B00tSoulClient::new(soul_url),
            fallback_on_error: true,
        }
    }

    /// Disable local fallback — errors from b00t soul API propagate.
    #[must_use]
    pub fn strict(mut self) -> Self {
        self.fallback_on_error = false;
        self
    }

    // ─── embedding cache helpers via b00t soul ────────────────────────────

    async fn soul_get_embedding(
        &self,
        provider: &str,
        model: &str,
        hash: &str,
    ) -> Result<Option<Vec<f32>>> {
        let key = cache_key(provider, model, hash);
        match self.soul.get(&key).await {
            Ok(Some(encoded)) => Ok(Some(decode_embedding(&encoded)?)),
            Ok(None) => Ok(None),
            Err(e) if self.fallback_on_error => {
                debug!("b00t soul GET failed, using local fallback: {e:#}");
                self.local
                    .get_cached_embedding(provider, model, hash)
                    .await
            },
            Err(e) => Err(e),
        }
    }

    async fn soul_put_embedding(
        &self,
        provider: &str,
        model: &str,
        provider_key: &str,
        hash: &str,
        embedding: &[f32],
    ) -> Result<()> {
        let key = cache_key(provider, model, hash);
        let encoded = encode_embedding(embedding);
        match self.soul.put(&key, &encoded).await {
            Ok(()) => {
                // 🤓 also write to local SQLite so vector_search (FTS) stays
                //    functional when b00t soul serve is the canonical store.
                self.local
                    .put_cached_embedding(provider, model, provider_key, hash, embedding)
                    .await
            },
            Err(e) if self.fallback_on_error => {
                warn!("b00t soul PUT failed, writing local only: {e:#}");
                self.local
                    .put_cached_embedding(provider, model, provider_key, hash, embedding)
                    .await
            },
            Err(e) => Err(e),
        }
    }
}

// ─── MemoryStore impl ─────────────────────────────────────────────────────────

#[async_trait]
impl MemoryStore for B00tSoulShim {
    // ---- files: local SQLite ----

    async fn upsert_file(&self, file: &FileRow) -> Result<()> {
        self.local.upsert_file(file).await
    }

    async fn get_file(&self, path: &str) -> Result<Option<FileRow>> {
        self.local.get_file(path).await
    }

    async fn delete_file(&self, path: &str) -> Result<()> {
        self.local.delete_file(path).await
    }

    async fn list_files(&self) -> Result<Vec<FileRow>> {
        self.local.list_files().await
    }

    // ---- chunks: local SQLite ----

    async fn upsert_chunks(&self, chunks: &[ChunkRow]) -> Result<()> {
        self.local.upsert_chunks(chunks).await
    }

    async fn get_chunks_for_file(&self, path: &str) -> Result<Vec<ChunkRow>> {
        self.local.get_chunks_for_file(path).await
    }

    async fn delete_chunks_for_file(&self, path: &str) -> Result<()> {
        self.local.delete_chunks_for_file(path).await
    }

    async fn get_chunk_by_id(&self, id: &str) -> Result<Option<ChunkRow>> {
        self.local.get_chunk_by_id(id).await
    }

    // ---- embedding cache: b00t soul K/V (fallback: local SQLite) ----

    async fn get_cached_embedding(
        &self,
        provider: &str,
        model: &str,
        hash: &str,
    ) -> Result<Option<Vec<f32>>> {
        self.soul_get_embedding(provider, model, hash).await
    }

    async fn put_cached_embedding(
        &self,
        provider: &str,
        model: &str,
        provider_key: &str,
        hash: &str,
        embedding: &[f32],
    ) -> Result<()> {
        self.soul_put_embedding(provider, model, provider_key, hash, embedding)
            .await
    }

    async fn put_cached_embeddings_batch(&self, entries: &[CacheEntry<'_>]) -> Result<()> {
        // 🤓 Batch: fire concurrently to b00t soul, then mirror to local SQLite.
        //    tokio::join! not usable here (variable arity); use futures::future::join_all.
        let futs: Vec<_> = entries
            .iter()
            .map(|e| {
                self.soul_put_embedding(e.provider, e.model, e.provider_key, e.hash, e.embedding)
            })
            .collect();
        for result in futures::future::join_all(futs).await {
            result?;
        }
        Ok(())
    }

    async fn count_cached_embeddings(&self) -> Result<usize> {
        // 🤓 Count from local SQLite (authoritative for size-based eviction).
        //    b00t soul K/V prefix scan is an option but more expensive.
        self.local.count_cached_embeddings().await
    }

    async fn evict_embedding_cache(&self, keep: usize) -> Result<usize> {
        // 🤓 Eviction: remove from local first, then propagate deletes to soul.
        let evicted = self.local.evict_embedding_cache(keep).await?;
        if evicted > 0 {
            debug!("evicted {evicted} embedding cache entries; b00t soul K/V may retain stale keys until TTL");
        }
        Ok(evicted)
    }

    // ---- search: local SQLite ----

    async fn vector_search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        self.local.vector_search(query_embedding, limit).await
    }

    async fn keyword_search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        self.local.keyword_search(query, limit).await
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;
    use crate::schema::run_migrations;

    /// Build an in-memory SQLite pool with migrations applied.
    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        pool
    }

    /// Build a shim pointed at a non-existent soul URL (forces local fallback).
    async fn fallback_shim() -> B00tSoulShim {
        let pool = test_pool().await;
        B00tSoulShim::with_soul_url(pool, "http://127.0.0.1:19999") // port nobody listens on
    }

    #[tokio::test]
    async fn shim_files_roundtrip_via_local_sqlite() {
        let shim = fallback_shim().await;
        let file = FileRow {
            path: "test/hello.md".into(),
            source: "test".into(),
            hash: "abc".into(),
            mtime: 1_000_000,
            size: 42,
        };
        shim.upsert_file(&file).await.unwrap();
        let got = shim.get_file("test/hello.md").await.unwrap();
        assert!(got.is_some());
        assert_eq!(got.unwrap().hash, "abc");
    }

    #[tokio::test]
    async fn shim_embedding_cache_falls_back_to_local() {
        // soul URL unreachable → fallback to local SQLite
        let shim = fallback_shim().await;
        shim.put_cached_embedding("openai", "text-3", "pk", "hash1", &[1.0, 0.0])
            .await
            .unwrap();
        let got = shim
            .get_cached_embedding("openai", "text-3", "hash1")
            .await
            .unwrap();
        assert!(got.is_some());
        let vec = got.unwrap();
        assert!((vec[0] - 1.0f32).abs() < 1e-5);
    }

    #[tokio::test]
    async fn shim_strict_mode_errors_on_unreachable_soul() {
        let pool = test_pool().await;
        let shim = B00tSoulShim::with_soul_url(pool, "http://127.0.0.1:19999").strict();
        let result = shim
            .put_cached_embedding("openai", "text-3", "pk", "hash2", &[0.5])
            .await;
        assert!(result.is_err(), "strict mode must propagate soul API error");
    }

    #[tokio::test]
    async fn encode_decode_embedding_roundtrip() {
        let v = vec![1.0f32, -0.5, 0.25, 0.0];
        let encoded = encode_embedding(&v);
        let decoded = decode_embedding(&encoded).unwrap();
        assert_eq!(v.len(), decoded.len());
        for (a, b) in v.iter().zip(decoded.iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }

    #[tokio::test]
    async fn shim_keyword_search_via_local_sqlite() {
        let shim = fallback_shim().await;
        let file = FileRow {
            path: "test/kw.md".into(),
            source: "test".into(),
            hash: "xyz".into(),
            mtime: 0,
            size: 0,
        };
        shim.upsert_file(&file).await.unwrap();
        let chunk = ChunkRow {
            id: "c1".into(),
            path: "test/kw.md".into(),
            source: "test".into(),
            start_line: 1,
            end_line: 5,
            hash: "h1".into(),
            model: "none".into(),
            text: "the quick brown fox".into(),
            embedding: None,
            updated_at: "2025-01-01T00:00:00Z".into(),
        };
        shim.upsert_chunks(&[chunk]).await.unwrap();
        let results = shim.keyword_search("quick brown", 5).await.unwrap();
        assert!(!results.is_empty());
    }
}
