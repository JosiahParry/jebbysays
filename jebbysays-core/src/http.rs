use std::sync::Arc;

use axum::{
    Extension,
    body::Body,
    extract::State,
    http::Request,
    response::{IntoResponse, Response},
};
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use sqlx::SqlitePool;
use tokio_util::sync::CancellationToken;
use tower::Service;

use crate::portfolio::Portfolio;

#[derive(Clone)]
pub struct McpState {
    pub db: SqlitePool,
    pub ct: CancellationToken,
    pub oauth: Arc<crate::auth::OAuthConfig>,
    pub sessions: Arc<LocalSessionManager>,
}

#[tracing::instrument(skip_all, fields(user_id = %user_id, method = %request.method(), uri = %request.uri()))]
pub async fn mcp_handler(
    State(state): State<Arc<McpState>>,
    Extension(user_id): Extension<String>,
    request: Request<Body>,
) -> Response {
    mcp_handler_inner(state, user_id, request).await
}

/// Inner handler that can be called directly with McpState
#[tracing::instrument(skip_all, fields(user_id = %user_id, method = %request.method(), uri = %request.uri()))]
pub async fn mcp_handler_inner(
    state: Arc<McpState>,
    user_id: String,
    request: Request<Body>,
) -> Response {
    tracing::debug!(headers = ?request.headers(), "incoming request headers");
    let portfolio = Portfolio::with_user(state.db.clone(), user_id);
    let mut service = StreamableHttpService::new(
        move || Ok(portfolio.clone()),
        state.sessions.clone(),
        StreamableHttpServerConfig {
            cancellation_token: state.ct.clone(),
            ..Default::default()
        },
    );
    service.call(request).await.into_response()
}
