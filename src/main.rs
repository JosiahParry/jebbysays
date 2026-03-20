pub mod auth;
pub mod http;
pub mod portfolio;
pub mod prompts;
pub mod resources;
pub mod tools;
pub mod types;

use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use axum::{Json, Router, middleware, routing::get};
use clap::{Parser, Subcommand};
use portfolio::Portfolio;
use rmcp::{ServiceExt, transport::stdio};
use serde_json::json;
use tokio_util::sync::CancellationToken;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

use auth::{OAuthConfig, jwks::JwksCache, middleware::auth_middleware};
use http::McpState;

#[derive(Parser)]
#[command(name = "chief", about = "Your personal chief of staff")]
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
        .map(|h| h.join("chief").join("chief.sqlite3"))
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
            .unwrap_or_else(|_| "debug".to_string().into()),
    );

    let cli = Cli::parse();
    let path = resolve_path(cli.path)?;
    let portfolio = Portfolio::new(path).await?;

    match cli.command {
        Command::Serve { port } => {
            sub.with(tracing_subscriber::fmt::layer()).init();

            let ct = CancellationToken::new();
            let jwks = Arc::new(JwksCache::new(Arc::new(OAuthConfig::from_env())).await?);
            let state = Arc::new(McpState {
                db: portfolio.db,
                ct: ct.child_token(),
            });

            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            let router = Router::new()
                .route("/mcp", axum::routing::any(http::mcp_handler))
                .route_layer(middleware::from_fn_with_state(jwks, auth_middleware))
                .with_state(state)
                .route("/.well-known/oauth-authorization-server", get(|| async {
                    let config = OAuthConfig::from_env();
                    Json(json!({
                        "issuer": config.issuer,
                        "authorization_endpoint": format!("{}/authorize", config.issuer.trim_end_matches('/')),
                        "token_endpoint": format!("{}/oauth/token", config.issuer.trim_end_matches('/')),
                        "response_types_supported": ["code"],
                        "code_challenge_methods_supported": ["S256"],
                    }))
                }))
                .route("/.well-known/oauth-protected-resource", get(|| async {
                    let config = OAuthConfig::from_env();
                    Json(json!({
                        "resource": config.audience,
                        "authorization_servers": [config.issuer],
                    }))
                }))
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
