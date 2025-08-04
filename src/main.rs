mod tools;
mod common;

use tools::agent_mcp::AgentMcpServer;

use anyhow::Result;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use tracing_subscriber::{self, EnvFilter};

const BIND_ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
                .add_directive(tracing::Level::DEBUG.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server with tool groups");
    tracing::info!("Available tool groups:");
    tracing::info!("eth_tools: balance, send");

    let config = SseServerConfig {
        bind: BIND_ADDRESS.parse()?,
        sse_path: String::from("/sse"),
        post_path: String::from("/message"),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    let (sse_server, router) = SseServer::new(config);
    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;
    let ct = sse_server.config.ct.child_token();

    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        ct.cancelled().await;
        tracing::info!("sse server cancelled");
    });
    tokio::spawn(async move {
        if let Err(e) = server.await {
            tracing::error!(error = %e, "sse server shutdown with error");
        }
    });

    let ct = sse_server.with_service(AgentMcpServer::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
