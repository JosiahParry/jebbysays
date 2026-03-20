pub mod jwks;
pub mod middleware;

pub struct OAuthConfig {
    pub audience: &'static str,
    pub issuer: &'static str,
}

impl OAuthConfig {
    pub fn from_env() -> Self {
        Self {
            audience: env!("OAUTH_AUDIENCE"),
            issuer: env!("OAUTH_ISSUER"),
        }
    }

    pub fn jwks_uri(&self) -> String {
        format!(
            "{}/.well-known/jwks.json",
            self.issuer.trim_end_matches('/')
        )
    }
}
