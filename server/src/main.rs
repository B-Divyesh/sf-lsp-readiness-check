use std::{net::SocketAddr, path::PathBuf};

use anyhow::Context;
use lsp_readiness_api::{Database, ServiceConfig, build_state, router};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        Some("backup") => {
            let destination = arguments
                .next()
                .context("usage: lsp-readiness-api backup <destination>")?;
            let config = ServiceConfig::from_env()?;
            Database::open(config.database_path)?.backup(&PathBuf::from(destination))?;
            println!("Backup completed.");
            Ok(())
        }
        Some("restore") => {
            let source = arguments
                .next()
                .context("usage: lsp-readiness-api restore <source>")?;
            let config = ServiceConfig::from_env()?;
            Database::restore(&PathBuf::from(source), &config.database_path)?;
            println!("Restore completed.");
            Ok(())
        }
        Some("migrate") => {
            let config = ServiceConfig::from_env()?;
            Database::open(config.database_path)?;
            println!("Migrations completed.");
            Ok(())
        }
        Some("serve") | None => serve().await,
        Some(command) => {
            anyhow::bail!("unknown command {command}; use serve, migrate, backup, or restore")
        }
    }
}

async fn serve() -> anyhow::Result<()> {
    let config = ServiceConfig::from_env()?;
    let public_origin = config.public_origin.clone();
    let state = build_state(config)?;
    let app = router(state, &public_origin)?;
    let port: u16 = std::env::var("PORT")
        .ok()
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(8080);
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "lsp_readiness_api_started");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut signal) =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            signal.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
