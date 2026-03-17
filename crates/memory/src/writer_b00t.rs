//! `B00tSoulWriter` — implements moltis `MemoryWriter` via b00t soul serve HTTP API.
//!
//! Delegates all memory writes to `b00t soul serve` (`/v1/memory/write`) instead
//! of writing to the local filesystem.  Falls back to direct filesystem writes
//! when b00t soul serve is unreachable (`fallback_on_error = true`, default).
//!
//! # Architecture (b00t is the soul backend)
//! ```text
//! moltis MemoryWriter::write_memory()
//!     └── B00tSoulWriter
//!             ├── POST /v1/memory/write  → b00t soul serve
//!             └── fallback: local ._b00t_/ filesystem
//! ```
//!
//! # b00t soul serve endpoints used
//! - `POST /v1/memory/write`  body: `{"file":"MEMORY.md","content":"...","append":true}`
//!   → returns `{"location":"...","bytes_written":N}`
//!
//! # b00t:map v1
//! # summary: B00tSoulWriter — moltis MemoryWriter → b00t soul /v1/memory/write
//! # tags: soul, memory, writer, b00t, shim, http
//! # tier: ch0nky

use std::path::PathBuf;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use moltis_agents::memory_writer::{MemoryWriteResult, MemoryWriter};

// ─── b00t soul memory write API client ───────────────────────────────────────

const DEFAULT_SOUL_URL: &str = "http://127.0.0.1:7700";

/// Maximum allowed memory write size (bytes).
/// Mirrors the 50KB cap used by `MemoryManager` to keep behavior consistent.
const MAX_CONTENT_BYTES: usize = 50 * 1024;

#[derive(Serialize)]
struct WriteRequest<'a> {
    file: &'a str,
    content: &'a str,
    append: bool,
}

#[derive(Deserialize)]
struct WriteResponse {
    location: String,
    bytes_written: usize,
}

struct B00tSoulHttpClient {
    base_url: String,
    http: reqwest::Client,
}

impl B00tSoulHttpClient {
    fn new(base_url: impl Into<String>) -> Self {
        let timeout = std::time::Duration::from_secs(5);
        let http = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_else(|err| {
                warn!(
                    "failed to build reqwest client for B00tSoulHttpClient with timeout {:?}: {err}",
                    timeout
                );
                reqwest::Client::builder()
                    .timeout(timeout)
                    .build()
                    .expect("failed to build fallback reqwest client for B00tSoulHttpClient")
            });

        Self {
            base_url: base_url.into(),
            http,
        }
    }

    async fn write_memory(&self, file: &str, content: &str, append: bool) -> Result<MemoryWriteResult> {
        let url = format!("{}/v1/memory/write", self.base_url);
        let resp = self
            .http
            .post(&url)
            .json(&WriteRequest { file, content, append })
            .send()
            .await
            .context("b00t soul write request")?
            .error_for_status()
            .context("b00t soul write response")?;
        let wr: WriteResponse = resp.json().await.context("decode write response")?;
        Ok(MemoryWriteResult {
            location: wr.location,
            bytes_written: wr.bytes_written,
        })
    }
}

// ─── Filesystem fallback ──────────────────────────────────────────────────────

/// Validate memory file paths for the local fallback writer.
///
/// This mirrors the stricter contract used by the primary `MemoryWriter`:
/// - allow exactly: `MEMORY.md` or `memory.md`
/// - or: `memory/<name>.md`
///   where `<name>`:
///     - is a single path segment (no `/` or `\`)
///     - contains no whitespace
///     - ends with `.md`
/// Additionally, all backslashes are rejected so Windows-style paths
/// cannot bypass checks.
fn validate_memory_path_fallback(file: &str) -> Result<()> {
    // Reject any use of Windows-style separators.
    if file.contains('\\') {
        anyhow::bail!("backslashes not allowed in memory file path: {file}");
    }

    // Reject leading/trailing whitespace and empty paths.
    if file.trim().is_empty() || file.trim() != file {
        anyhow::bail!("whitespace not allowed in memory file path: {file}");
    }

    match file {
        "MEMORY.md" | "memory.md" => Ok(()),
        _ => {
            const PREFIX: &str = "memory/";
            if !file.starts_with(PREFIX) {
                anyhow::bail!("invalid memory file path (must be MEMORY.md, memory.md, or memory/<name>.md): {file}");
            }
            let rest = &file[PREFIX.len()..];
            if rest.is_empty() {
                anyhow::bail!("memory/<name>.md must include a non-empty <name>: {file}");
            }
            // `<name>` must be a single segment with no nested separators.
            if rest.contains('/') {
                anyhow::bail!("nested paths not allowed in memory file name: {file}");
            }
            // No whitespace inside the name, and must end with `.md`.
            if rest.chars().any(|c| c.is_whitespace()) {
                anyhow::bail!("whitespace not allowed in memory file name: {file}");
            }
            if !rest.ends_with(".md") {
                anyhow::bail!("memory file name must end with .md: {file}");
            }
            Ok(())
        }
    }
}

struct LocalFallbackWriter {
    base: PathBuf,
}

impl LocalFallbackWriter {
    fn detect() -> Self {
        let local = std::env::current_dir()
            .ok()
            .map(|d| d.join("._b00t_"))
            .filter(|p| p.is_dir());
        let base = local.unwrap_or_else(|| {
            dirs_next::home_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("._b00t_")
        });
        Self { base }
    }

    async fn write_memory(&self, file: &str, content: &str, append: bool) -> Result<MemoryWriteResult> {
        // Enforce the same memory path contract as the primary MemoryWriter.
        validate_memory_path_fallback(file)?;

        let file_path = std::path::Path::new(file);
        for component in file_path.components() {
            use std::path::Component;
            match component {
                Component::ParentDir => anyhow::bail!("path traversal in memory file: {file}"),
                Component::RootDir | Component::Prefix(_) => {
                    anyhow::bail!("absolute path not allowed: {file}")
                }
                _ => {}
            }
        }
        let resolved = self.base.join(file_path);
        if let Some(parent) = resolved.parent() {
            tokio::fs::create_dir_all(parent).await.context("create memory dir")?;
        }
        let final_content = if append && resolved.exists() {
            let existing = tokio::fs::read_to_string(&resolved).await.context("read existing")?;
            format!("{}\n\n{}", existing.trim_end(), content)
        } else {
            content.to_owned()
        };
        let bytes = final_content.len();
        tokio::fs::write(&resolved, &final_content).await.context("write memory file")?;
        Ok(MemoryWriteResult {
            location: resolved.display().to_string(),
            bytes_written: bytes,
        })
    }
}

// ─── B00tSoulWriter ───────────────────────────────────────────────────────────

/// `B00tSoulWriter` — `MemoryWriter` impl that delegates to b00t soul serve.
///
/// `fallback_on_error = true` (default): b00t soul unreachable → local `._b00t_/` writes.
pub struct B00tSoulWriter {
    client: B00tSoulHttpClient,
    fallback: LocalFallbackWriter,
    fallback_on_error: bool,
}

impl B00tSoulWriter {
    pub fn new() -> Self {
        Self::with_soul_url(DEFAULT_SOUL_URL)
    }

    pub fn with_soul_url(soul_url: impl Into<String>) -> Self {
        Self {
            client: B00tSoulHttpClient::new(soul_url),
            fallback: LocalFallbackWriter::detect(),
            fallback_on_error: true,
        }
    }

    #[must_use]
    pub fn strict(mut self) -> Self {
        self.fallback_on_error = false;
        self
    }
}

impl Default for B00tSoulWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MemoryWriter for B00tSoulWriter {
    async fn write_memory(
        &self,
        file: &str,
        content: &str,
        append: bool,
    ) -> Result<MemoryWriteResult> {
        // Enforce a maximum content size to mirror `MemoryManager`'s 50KB cap and
        // avoid sending or writing excessively large payloads.
        if content.len() > MAX_CONTENT_BYTES {
            anyhow::bail!(
                "memory write exceeds maximum allowed size of {} bytes (got {} bytes)",
                MAX_CONTENT_BYTES,
                content.len()
            );
        }

        match self.client.write_memory(file, content, append).await {
            Ok(result) => {
                debug!("soul write via b00t soul serve → {}", result.location);
                Ok(result)
            }
            // 🤓 debug! not warn! — fallback is expected when soul serve not running
            //    mirrors peer review fix in store_b00t.rs (PR #2, PR #3)
            Err(e) if self.fallback_on_error => {
                debug!("b00t soul write failed, using local fallback: {e:#}");
                self.fallback.write_memory(file, content, append).await
            }
            Err(e) => Err(e),
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn writer_fallback_on_unreachable_soul() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("._b00t_")).unwrap();
        let writer = B00tSoulWriter {
            client: B00tSoulHttpClient::new("http://127.0.0.1:19999"),
            fallback: LocalFallbackWriter { base: dir.path().join("._b00t_") },
            fallback_on_error: true,
        };
        let result = writer.write_memory("MEMORY.md", "# Test", false).await.unwrap();
        assert!(result.bytes_written > 0);
        let content = std::fs::read_to_string(&result.location).unwrap();
        assert_eq!(content, "# Test");
    }

    #[tokio::test]
    async fn writer_strict_errors_on_unreachable_soul() {
        let writer = B00tSoulWriter::with_soul_url("http://127.0.0.1:19999").strict();
        assert!(writer.write_memory("MEMORY.md", "test", false).await.is_err());
    }

    #[tokio::test]
    async fn fallback_writer_append_semantics() {
        let dir = tempfile::tempdir().unwrap();
        let w = LocalFallbackWriter { base: dir.path().to_path_buf() };
        w.write_memory("mem.md", "first", false).await.unwrap();
        w.write_memory("mem.md", "second", true).await.unwrap();
        let content = std::fs::read_to_string(dir.path().join("mem.md")).unwrap();
        assert!(content.contains("first") && content.contains("second"));
    }

    #[tokio::test]
    async fn fallback_writer_rejects_path_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let w = LocalFallbackWriter { base: dir.path().to_path_buf() };
        assert!(w.write_memory("../evil.md", "x", false).await.is_err());
    }
}
