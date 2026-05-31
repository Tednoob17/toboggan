use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher as _};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::TobogganState;

const DEBOUNCE_DURATION: Duration = Duration::from_millis(300);

pub fn start_watch_task(talk_path: PathBuf, state: TobogganState) -> anyhow::Result<()> {
    info!(path = %talk_path.display(), "Starting file watcher");

    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>(100);

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
        if tx.blocking_send(res).is_err() {
            error!("Failed to send file watcher event - channel closed");
        }
    })
    .context("Failed to create file watcher")?;

    let recursive = if talk_path.is_dir() {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    watcher
        .watch(&talk_path, recursive)
        .with_context(|| format!("Failed to watch: {}", talk_path.display()))?;

    tokio::spawn(watch_loop(watcher, rx, talk_path, state));

    Ok(())
}

async fn watch_loop(
    watcher: RecommendedWatcher,
    mut rx: mpsc::Receiver<Result<Event, notify::Error>>,
    talk_path: PathBuf,
    state: TobogganState,
) {
    let mut last_reload = tokio::time::Instant::now();
    let _watcher = watcher; // Keep watcher alive

    while let Some(event_result) = rx.recv().await {
        match event_result {
            Ok(event) => {
                if should_reload(&event)
                    && let Some(reload_time) = handle_reload(&talk_path, &state, last_reload).await
                {
                    last_reload = reload_time;
                }
            }
            Err(error) => {
                warn!(?error, "File watcher error");
            }
        }
    }

    info!("File watcher task stopped");
}

async fn handle_reload(
    talk_path: &Path,
    state: &TobogganState,
    last_reload: tokio::time::Instant,
) -> Option<tokio::time::Instant> {
    let now = tokio::time::Instant::now();
    let elapsed = now.duration_since(last_reload);

    if elapsed >= DEBOUNCE_DURATION {
        info!("File change detected, reloading talk");
        if let Err(err) = reload_talk(talk_path, state).await {
            error!("Failed to reload talk: {err:?}");
        } else {
            return Some(now);
        }
    }
    None
}

fn should_reload(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
    )
}

async fn reload_talk(path: &Path, state: &TobogganState) -> anyhow::Result<()> {
    let new_talk = if path.is_dir() {
        let parser = toboggan_cli::parser::FolderParser::new(
            path.to_path_buf(),
            "base16-ocean.light".to_owned(),
        )
        .context("Parsing markdown folder")?;
        let parse_result = parser.parse(None, None).context("Processing slides")?;
        parse_result.to_talk()
    } else {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Reading talk file {}", path.display()))?;
        toml::from_str(&content).context("Parsing talk TOML")?
    };

    state.reload_talk(new_talk).await?;

    Ok(())
}
