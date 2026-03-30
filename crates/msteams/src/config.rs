use {
    moltis_channels::{
        config_view::ChannelConfigView,
        gating::{DmPolicy, GroupPolicy, MentionMode},
    },
    moltis_common::secret_serde,
    secrecy::Secret,
    serde::{Deserialize, Serialize, ser::SerializeStruct},
};

/// Configuration for a single Microsoft Teams bot account.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MsTeamsAccountConfig {
    /// Microsoft App ID (bot registration client ID).
    pub app_id: String,

    /// Microsoft App Password (client secret).
    #[serde(serialize_with = "secret_serde::serialize_secret")]
    pub app_password: Secret<String>,

    /// OAuth tenant segment for Bot Framework token issuance.
    pub oauth_tenant: String,

    /// OAuth scope for Bot Framework connector API.
    pub oauth_scope: String,

    /// DM access policy.
    pub dm_policy: DmPolicy,

    /// Group access policy.
    pub group_policy: GroupPolicy,

    /// Mention activation mode for group chats.
    pub mention_mode: MentionMode,

    /// User allowlist (AAD object IDs or channel user IDs).
    pub allowlist: Vec<String>,

    /// Group/team allowlist.
    pub group_allowlist: Vec<String>,

    /// Optional shared secret validated against `?secret=...` on webhook calls.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "secret_serde::serialize_option_secret"
    )]
    pub webhook_secret: Option<Secret<String>>,

    /// Default model ID for this channel account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Provider name associated with `model`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_provider: Option<String>,
}

impl std::fmt::Debug for MsTeamsAccountConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MsTeamsAccountConfig")
            .field("app_id", &self.app_id)
            .field("app_password", &"[REDACTED]")
            .field("oauth_tenant", &self.oauth_tenant)
            .field("oauth_scope", &self.oauth_scope)
            .field("dm_policy", &self.dm_policy)
            .field("group_policy", &self.group_policy)
            .field("mention_mode", &self.mention_mode)
            .field("allowlist", &self.allowlist)
            .field("group_allowlist", &self.group_allowlist)
            .field(
                "webhook_secret",
                &self.webhook_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("model", &self.model)
            .field("model_provider", &self.model_provider)
            .finish()
    }
}

/// Wrapper that serializes secret fields as `"[REDACTED]"` for API responses.
pub struct RedactedConfig<'a>(pub &'a MsTeamsAccountConfig);

impl Serialize for RedactedConfig<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let c = self.0;
        let mut count = 9; // always-present fields
        count += c.webhook_secret.is_some() as usize;
        count += c.model.is_some() as usize;
        count += c.model_provider.is_some() as usize;
        let mut s = serializer.serialize_struct("MsTeamsAccountConfig", count)?;
        s.serialize_field("app_id", &c.app_id)?;
        s.serialize_field("app_password", secret_serde::REDACTED)?;
        s.serialize_field("oauth_tenant", &c.oauth_tenant)?;
        s.serialize_field("oauth_scope", &c.oauth_scope)?;
        s.serialize_field("dm_policy", &c.dm_policy)?;
        s.serialize_field("group_policy", &c.group_policy)?;
        s.serialize_field("mention_mode", &c.mention_mode)?;
        s.serialize_field("allowlist", &c.allowlist)?;
        s.serialize_field("group_allowlist", &c.group_allowlist)?;
        if c.webhook_secret.is_some() {
            s.serialize_field("webhook_secret", secret_serde::REDACTED)?;
        }
        if c.model.is_some() {
            s.serialize_field("model", &c.model)?;
        }
        if c.model_provider.is_some() {
            s.serialize_field("model_provider", &c.model_provider)?;
        }
        s.end()
    }
}

impl ChannelConfigView for MsTeamsAccountConfig {
    fn allowlist(&self) -> &[String] {
        &self.allowlist
    }

    fn group_allowlist(&self) -> &[String] {
        &self.group_allowlist
    }

    fn dm_policy(&self) -> DmPolicy {
        self.dm_policy.clone()
    }

    fn group_policy(&self) -> GroupPolicy {
        self.group_policy.clone()
    }

    fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    fn model_provider(&self) -> Option<&str> {
        self.model_provider.as_deref()
    }
}

impl Default for MsTeamsAccountConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_password: Secret::new(String::new()),
            oauth_tenant: "botframework.com".into(),
            oauth_scope: "https://api.botframework.com/.default".into(),
            dm_policy: DmPolicy::Allowlist,
            group_policy: GroupPolicy::Open,
            mention_mode: MentionMode::Mention,
            allowlist: Vec::new(),
            group_allowlist: Vec::new(),
            webhook_secret: None,
            model: None,
            model_provider: None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn redacted_hides_secrets() {
        let cfg = MsTeamsAccountConfig {
            app_id: "my-app-id".into(),
            app_password: Secret::new("super-secret-pw".into()),
            webhook_secret: Some(Secret::new("webhook-sec".into())),
            model: Some("gpt-4o".into()),
            ..Default::default()
        };
        let redacted = serde_json::to_value(RedactedConfig(&cfg)).unwrap();
        assert_eq!(redacted["app_password"], "[REDACTED]");
        assert_eq!(redacted["webhook_secret"], "[REDACTED]");
        // Non-secret fields preserved
        assert_eq!(redacted["app_id"], "my-app-id");
        assert_eq!(redacted["model"], "gpt-4o");

        // Storage path still exposes secrets
        let storage = serde_json::to_value(&cfg).unwrap();
        assert_eq!(storage["app_password"], "super-secret-pw");
        assert_eq!(storage["webhook_secret"], "webhook-sec");
    }

    #[test]
    fn redacted_omits_none_webhook_secret() {
        let cfg = MsTeamsAccountConfig::default();
        let redacted = serde_json::to_value(RedactedConfig(&cfg)).unwrap();
        assert!(redacted.get("webhook_secret").is_none());
    }

    #[test]
    fn config_round_trip() {
        let json = serde_json::json!({
            "app_id": "test-id",
            "app_password": "test-pw",
            "dm_policy": "open",
        });
        let cfg: MsTeamsAccountConfig = serde_json::from_value(json).unwrap();
        assert_eq!(cfg.app_id, "test-id");
        assert_eq!(cfg.dm_policy, DmPolicy::Open);
        let value = serde_json::to_value(&cfg).unwrap();
        let _: MsTeamsAccountConfig = serde_json::from_value(value).unwrap();
    }
}
