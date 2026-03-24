pub mod jwks;
pub mod middleware;
pub mod web;

use serde::Deserialize;

#[derive(Deserialize)]
struct AuthServerMetadata {
    authorization_endpoint: String,
    token_endpoint: String,
    jwks_uri: String,
    registration_endpoint: Option<String>,
}

pub struct OAuthConfig {
    pub audience: String,
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    pub registration_endpoint: Option<String>,
    /// Only needed for the web UI login flow. Not required for MCP-only deployments.
    pub client_id: Option<String>,
    /// Derived from `audience` if `client_id` is set.
    pub redirect_uri: Option<String>,
}

impl OAuthConfig {
    pub async fn from_env() -> anyhow::Result<Self> {
        let audience = std::env::var("MCP_SERVER_URL")
            .map_err(|_| anyhow::anyhow!("MCP_SERVER_URL must be set"))?;
        let issuer = std::env::var("OAUTH_ISSUER")
            .map_err(|_| anyhow::anyhow!("OAUTH_ISSUER must be set"))?;
        let client_id = std::env::var("OAUTH_CLIENT_ID").ok();
        let redirect_uri = client_id
            .as_ref()
            .map(|_| format!("{}/auth/callback", audience.trim_end_matches('/')));

        let discovery_url = format!(
            "{}/.well-known/oauth-authorization-server",
            issuer.trim_end_matches('/')
        );
        let metadata = reqwest::get(&discovery_url)
            .await?
            .json::<AuthServerMetadata>()
            .await?;

        Ok(Self {
            audience,
            issuer,
            authorization_endpoint: metadata.authorization_endpoint,
            token_endpoint: metadata.token_endpoint,
            jwks_uri: metadata.jwks_uri,
            registration_endpoint: metadata.registration_endpoint,
            client_id,
            redirect_uri,
        })
    }
}
