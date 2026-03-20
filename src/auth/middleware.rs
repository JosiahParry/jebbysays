use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::jwks::JwksCache;

pub async fn auth_middleware(
    State(jwks): State<Arc<JwksCache>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sub = jwks.validate_token(token).await.map_err(|e| {
        tracing::debug!("token validation failed: {e}");
        StatusCode::UNAUTHORIZED
    })?;

    tracing::debug!("authenticated user: {sub}");
    request.extensions_mut().insert(sub);
    Ok(next.run(request).await)
}
