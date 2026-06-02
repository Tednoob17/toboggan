use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use toboggan_cli::parser::FolderParser;
use toboggan_core::Talk;
use tracing::{info, instrument, warn};
use utoipa::openapi::OpenApi;

use crate::{
    ClientService, Settings, TalkService, TobogganState, routes_with_cors, start_watch_task,
};

#[doc(hidden)]
#[instrument]
pub async fn launch(settings: Settings) -> anyhow::Result<()> {
    info!(?settings, "launching server...");
    let Settings {
        host,
        port,
        ref talk,
        max_clients,
        ..
    } = settings;

    let talk = load_talk(talk).await.context("Loading talk")?;

    let addr = SocketAddr::from((host, port));
    info!(?addr, "Using address");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Connecting to {addr} ..."))?;

    let talk_service = TalkService::new(talk).context("build talk service")?;
    let client_service = ClientService::new(max_clients);
    let cleanup_service = client_service.clone();
    let state = TobogganState::new(talk_service, client_service);

    let cleanup_interval = settings.cleanup_interval();
    tokio::spawn(async move {
        cleanup_service.cleanup_clients_task(cleanup_interval).await;
        info!("Cleanup task completed");
    });

    if settings.watch {
        let watch_state = state.clone();
        let watch_path = settings.talk.clone();
        start_watch_task(watch_path, watch_state).context("Starting file watcher")?;
    }

    let openapi = create_openapi()?;

    let router = routes_with_cors(
        settings.allowed_origins.as_deref(),
        settings.public_dir.clone(),
        openapi,
    )
    .with_state(state);
    let shutdown_signal = setup_shutdown_signal(settings.shutdown_timeout());

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await
    .context("Axum server")?;

    info!("Server shutdown complete");

    Ok(())
}

#[instrument]
fn load_talk_from_folder(path: &Path) -> anyhow::Result<Talk> {
    let parser = FolderParser::new(path.to_path_buf(), "base16-ocean.light".to_owned())
        .with_context(|| format!("Parsing markdown folder {}", path.display()))?;
    let parse_result = parser
        .parse(None, None)
        .with_context(|| format!("Processing slides from {}", path.display()))?;
    let talk = parse_result.to_talk();
    info!(
        "Loaded {} slides from markdown folder {}",
        talk.slides.len(),
        path.display()
    );
    Ok(talk)
}

#[instrument]
async fn load_talk(path: &Path) -> anyhow::Result<Talk> {
    if path.is_dir() {
        return load_talk_from_folder(path);
    }
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Reading talk file {}", path.display()))?;
    let result = toml::from_str(&content).context("Parsing talk")?;
    Ok(result)
}

async fn setup_shutdown_signal(timeout: Duration) {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            warn!("Failed to install Ctrl+C handler: {err}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(err) => {
                warn!("Failed to install SIGTERM handler: {err}");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            info!("Received Ctrl+C, initiating graceful shutdown...");
        }
        () = terminate => {
            info!("Received SIGTERM, initiating graceful shutdown...");
        }
    }

    info!(
        "Waiting up to {} seconds for graceful shutdown",
        timeout.as_secs()
    );

    info!("Shutdown signal processed, server will now terminate gracefully");
}

fn create_openapi() -> anyhow::Result<OpenApi> {
    let json_content = include_str!("../openapi.json");
    let openapi = serde_json::from_str(json_content).context("reading openapi.json file")?;
    Ok(openapi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::expect_used)]
    fn test_create_openapi() {
        let result = create_openapi();

        assert!(result.is_ok(), "create_openapi should succeed: {result:?}");

        let openapi = result.expect("should have OpenApi");

        // Check that paths are present (from the generated openapi.yml)
        assert!(
            !openapi.paths.paths.is_empty(),
            "should have API paths from openapi.yml"
        );

        // Check that schemas are present
        assert!(
            openapi.components.is_some(),
            "should have component schemas"
        );
    }
}
