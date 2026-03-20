pub mod jwks;
pub mod middleware;

use serde::Deserialize;

#[derive(Deserialize)]
struct OidcDiscovery {
    authorization_endpoint: String,
    token_endpoint: String,
    jwks_uri: String,
}

pub struct OAuthConfig {
    pub audience: String,
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
}

impl OAuthConfig {
    pub async fn from_env() -> anyhow::Result<Self> {
        let audience = std::env::var("OAUTH_AUDIENCE")
            .map_err(|_| anyhow::anyhow!("OAUTH_AUDIENCE must be set"))?;
        let issuer = std::env::var("OAUTH_ISSUER")
            .map_err(|_| anyhow::anyhow!("OAUTH_ISSUER must be set"))?;

        let discovery_url = format!(
            "{}/.well-known/openid-configuration",
            issuer.trim_end_matches('/')
        );
        let discovery = reqwest::get(&discovery_url)
            .await?
            .json::<OidcDiscovery>()
            .await?;

        Ok(Self {
            audience,
            issuer,
            authorization_endpoint: discovery.authorization_endpoint,
            token_endpoint: discovery.token_endpoint,
            jwks_uri: discovery.jwks_uri,
        })
    }
}
