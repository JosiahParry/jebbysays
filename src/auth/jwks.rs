use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::OAuthConfig;

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    aud: serde_json::Value,
    exp: u64,
}

#[derive(Clone)]
pub struct JwksCache {
    keys: Arc<RwLock<HashMap<String, DecodingKey>>>,
    config: Arc<OAuthConfig>,
}

impl JwksCache {
    pub async fn new(config: Arc<OAuthConfig>) -> anyhow::Result<Self> {
        let cache = Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            config,
        };
        cache.refresh().await?;
        Ok(cache)
    }

    async fn refresh(&self) -> anyhow::Result<()> {
        let resp = reqwest::get(self.config.jwks_uri())
            .await?
            .json::<JwksResponse>()
            .await?;

        let mut keys = self.keys.write().await;
        keys.clear();
        for jwk in resp.keys {
            let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
            keys.insert(jwk.kid, key);
        }
        Ok(())
    }

    pub async fn validate_token(&self, token: &str) -> anyhow::Result<String> {
        let header = jsonwebtoken::decode_header(token)?;
        let kid = header.kid.ok_or_else(|| anyhow!("token missing kid"))?;

        let keys = self.keys.read().await;
        let key = keys
            .get(&kid)
            .ok_or_else(|| anyhow!("unknown kid: {kid}"))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.config.audience]);

        let data = decode::<Claims>(token, key, &validation)?;
        Ok(data.claims.sub)
    }
}
