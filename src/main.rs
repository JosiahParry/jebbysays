pub mod auth;
pub mod http;
pub mod portfolio;
pub mod prompts;
pub mod resources;
pub mod tools;
pub mod types;

use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use axum::{Json, Router, middleware, response::Html, routing::get};
use clap::{Parser, Subcommand};
use portfolio::Portfolio;
use rmcp::{ServiceExt, transport::stdio};
use serde_json::json;
use tokio_util::sync::CancellationToken;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

use auth::{OAuthConfig, jwks::JwksCache, middleware::auth_middleware};
use http::McpState;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;

async fn landing() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn oauth_authorization_server(
    axum::extract::State(state): axum::extract::State<Arc<McpState>>,
) -> Json<serde_json::Value> {
    let mut doc = json!({
        "issuer": state.oauth.issuer,
        "authorization_endpoint": state.oauth.authorization_endpoint,
        "token_endpoint": state.oauth.token_endpoint,
        "response_types_supported": ["code"],
        "code_challenge_methods_supported": ["S256"],
    });

    if let Some(reg) = &state.oauth.registration_endpoint {
        doc["registration_endpoint"] = json!(reg);
    }

    Json(doc)
}

async fn oauth_protected_resource(
    axum::extract::State(state): axum::extract::State<Arc<McpState>>,
) -> Json<serde_json::Value> {
    Json(json!({
        "resource": state.oauth.audience,
        "authorization_servers": [state.oauth.audience],
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
            .unwrap_or_else(|_| "debug,h2=off".to_string().into()),
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

            let ct = CancellationToken::new();
            let oauth = Arc::new(OAuthConfig::from_env().await?);
            let jwks = Arc::new(JwksCache::new(oauth.clone()).await?);
            let state = Arc::new(McpState {
                db: portfolio.db,
                ct: ct.child_token(),
                oauth,
                sessions: Arc::new(LocalSessionManager::default()),
            });

            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            let router = Router::new()
                .route("/mcp", axum::routing::any(http::mcp_handler))
                .route_layer(middleware::from_fn_with_state(jwks, auth_middleware))
                .with_state(state.clone())
                .route("/", get(landing))
                .nest_service("/imgs", ServeDir::new("imgs"))
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
                .with_state(state)
                .layer(cors);

            let addr = format!("127.0.0.1:{port}");
            let tcp_listener = tokio::net::TcpListener::bind(&addr).await?;
            tracing::info!("Listening on {addr}");
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
