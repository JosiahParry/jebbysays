use anyhow::anyhow;
use app::{shell, App};
use axum::{extract::State, middleware, routing::get, Json, Router};
use clap::{Parser, Subcommand};
use jebbysays_core::auth::jwks::JwksCache;
use jebbysays_core::auth::middleware::auth_middleware;
use jebbysays_core::auth::web::{callback_handler, login_handler, logout_handler, WebAuthState};
use jebbysays_core::{McpState, OAuthConfig, Portfolio};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::{transport::stdio, ServiceExt};
use serde_json::json;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tower_http::cors::{Any, CorsLayer};
use tower_sessions::{cookie::SameSite, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

#[derive(Clone, axum::extract::FromRef)]
struct AppState {
    mcp: Arc<McpState>,
    jwks: Arc<JwksCache>,
    leptos: LeptosOptions,
}

impl axum::extract::FromRef<AppState> for WebAuthState {
    fn from_ref(state: &AppState) -> Self {
        Self {
            oauth: state.mcp.oauth.clone(),
            jwks: state.jwks.clone(),
        }
    }
}

async fn oauth_authorization_server(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut doc = json!({
        "issuer": state.mcp.oauth.issuer,
        "authorization_endpoint": state.mcp.oauth.authorization_endpoint,
        "token_endpoint": state.mcp.oauth.token_endpoint,
        "response_types_supported": ["code"],
        "code_challenge_methods_supported": ["S256"],
    });

    if let Some(reg) = &state.mcp.oauth.registration_endpoint {
        doc["registration_endpoint"] = json!(reg);
    }

    Json(doc)
}

async fn oauth_protected_resource(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "resource": state.mcp.oauth.audience,
        "authorization_servers": [state.mcp.oauth.audience],
    }))
}

#[derive(Parser)]
#[command(name = "jebbysays", about = "Your personal chief of staff")]
struct Cli {
    /// Path to the SQLite database
    #[arg(long, global = true)]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the HTTP MCP server
    Serve {
        /// Port to listen on
        #[arg(long, default_value_t = 24433)]
        port: u16,
    },
    /// Start the stdio MCP server
    Stdio,
}

fn resolve_path(path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    if let Some(p) = path {
        return Ok(p);
    }

    dirs::data_dir()
        .map(|h| h.join("jebbysays").join("jebbysays.sqlite3"))
        .ok_or_else(|| {
            anyhow!(
                "Could not determine data directory. Use --path to specify the database location."
            )
        })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sub = tracing_subscriber::registry().with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "debug,h2=off,hyper_util=off".to_string().into()),
    );

    let cli = Cli::parse();
    let path = resolve_path(cli.path)?;
    let portfolio = Portfolio::new(path).await?;

    match cli.command {
        Command::Serve { port } => {
            sub.with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_line_number(true),
            )
            .init();

            let conf = get_configuration(Some("Cargo.toml"))?;
            let leptos_options = LeptosOptions::builder()
                .output_name("jebbysays")
                .site_addr(format!("127.0.0.1:{port}").parse::<SocketAddr>()?)
                .site_root(conf.leptos_options.site_root)
                .site_pkg_dir(conf.leptos_options.site_pkg_dir)
                .build();

            let routes = generate_route_list(App);

            let ct = CancellationToken::new();
            let oauth = Arc::new(OAuthConfig::from_env().await?);
            let jwks = Arc::new(JwksCache::new(oauth.clone()).await?);

            let session_store = SqliteStore::new(portfolio.db.clone());
            session_store.migrate().await?;
            let session_layer = SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_same_site(SameSite::Lax);

            let mcp_state = Arc::new(McpState {
                db: portfolio.db,
                ct: ct.child_token(),
                oauth,
                sessions: Arc::new(LocalSessionManager::default()),
            });

            let app_state = AppState {
                mcp: mcp_state,
                jwks,
                leptos: leptos_options,
            };

            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            // MCP sub-router with auth middleware scoped to /mcp only
            let mcp_router = Router::new()
                .route("/mcp", axum::routing::any(mcp_handler_with_state))
                .route_layer(middleware::from_fn_with_state(
                    app_state.jwks.clone(),
                    auth_middleware,
                ));

            let router = Router::new()
                .merge(mcp_router)
                .route("/auth/login", get(login_handler))
                .route("/auth/callback", get(callback_handler))
                .route("/auth/logout", get(logout_handler))
                .route(
                    "/.well-known/oauth-authorization-server",
                    get(oauth_authorization_server),
                )
                .route(
                    "/.well-known/oauth-protected-resource",
                    get(oauth_protected_resource),
                )
                .route(
                    "/.well-known/oauth-protected-resource/mcp",
                    get(oauth_protected_resource),
                )
                .leptos_routes_with_context(
                    &app_state,
                    routes,
                    {
                        let mcp = app_state.mcp.clone();
                        move || provide_context(mcp.clone())
                    },
                    {
                        let leptos_options = app_state.leptos.clone();
                        move || shell(leptos_options.clone())
                    },
                )
                .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
                .with_state(app_state)
                .layer(session_layer)
                .layer(cors);

            let addr = format!("127.0.0.1:{port}");
            let tcp_listener = tokio::net::TcpListener::bind(&addr).await?;
            tracing::info!("Listening on http://{addr}");
            let _ = axum::serve(tcp_listener, router)
                .with_graceful_shutdown(async move {
                    tokio::signal::ctrl_c().await.unwrap();
                    ct.cancel();
                })
                .await;
        }
        Command::Stdio => {
            sub.with(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_ansi(false),
            )
            .init();

            let service = portfolio.serve(stdio()).await?;
            service.waiting().await?;
        }
    }

    Ok(())
}

/// MCP handler that extracts state from AppState
async fn mcp_handler_with_state(
    State(app_state): State<AppState>,
    axum::extract::Extension(user_id): axum::extract::Extension<String>,
    request: axum::http::Request<axum::body::Body>,
) -> axum::response::Response {
    jebbysays_core::http::mcp_handler_inner(app_state.mcp.clone(), user_id, request).await
}
