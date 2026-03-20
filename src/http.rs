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
pub(crate) struct McpState {
    pub(crate) db: SqlitePool,
    pub(crate) ct: CancellationToken,
    pub(crate) oauth: Arc<crate::auth::OAuthConfig>,
    pub(crate) sessions: Arc<LocalSessionManager>,
}

#[tracing::instrument(skip_all, fields(user_id = %user_id, method = %request.method(), uri = %request.uri()))]
pub(crate) async fn mcp_handler(
    State(state): State<Arc<McpState>>,
    Extension(user_id): Extension<String>,
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
