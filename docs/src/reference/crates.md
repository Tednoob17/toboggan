# Crate Reference

This page gives a compact, code-oriented summary of the main crates in the workspace.

## `toboggan-core`

Shared domain model used everywhere else.

- `Talk`: presentation container with `title`, `date`, optional `footer`/`head`, and `slides`.
- `Slide`: slide body, notes, style, terminals, and kind (`Cover`, `Part`, `Standard`).
- `Content`: text/HTML content with accessibility-friendly alternatives.
- `State`: runtime presentation state (`Init`, `Running`, `Done`).
- `Command` / `Notification`: JSON protocol enums exchanged over WebSocket.
- `ClientId`, `ClientInfo`, `TalkResponse`, `SlidesResponse`: transport-facing types.
- `Date`, `Duration`, `Timestamp`: time helpers with serde support.

## `toboggan-cli`

Converts a folder of Markdown/HTML sources into a serialized talk.

- Entry point: `cargo run -p toboggan-cli -- <slides-folder> -o output.toml`
- Supports output formats: `toml`, `json`, `yaml`, `html`.
- Reads `_cover.md`, `_part.md`, `<!-- pause -->`, speaker notes, and terminal blocks.
- Computes presentation statistics and prints slide titles plus duration estimates.

## `toboggan-server`

Hosts the talk and coordinates connected clients.

- Entry point: `cargo run -p toboggan-server -- <talk.toml>`
- Validates host/port, WebSocket heartbeat, cleanup timers, and optional public assets.
- Keeps presentation state in a shared `TobogganState` facade.
- Accepts `Register` before handling other commands and broadcasts notifications to all clients.

## `toboggan-client`

Reusable async client layer for the different frontends.

- Builds `api_url` and `websocket_url` from a host/port pair.
- Provides retry behavior with exponential backoff and optional jitter.
- Reuses the same wire types from `toboggan-core`.

## `toboggan-stats`

Presentation metrics and duration calculations.

- Counts slides, parts, words, images, and notes.
- Estimates talk length with configurable speaking rate.
- Used by the CLI when it prints summary statistics.

## `toboggan-tui`

Terminal user interface for live presenting.

- Uses `ratatui`, `crossterm`, and `toboggan-client`.
- Sends navigation commands and renders the current state.
- Useful for SSH-based or terminal-only presentations.

## `toboggan-web`

Browser frontend built with Vite and TypeScript.

- Dev scripts: `dev`, `build`, `preview`, `serve`, `lint`, `format`, `check`.
- Serves the presenter-facing UI in the browser.
- Talks to the same server protocol as the other clients.

## `toboggan-mobile`

UniFFI bridge for Swift and Kotlin consumers.

- Produces `lib`, `cdylib`, and `staticlib` artifacts.
- Includes helper binaries for UniFFI code generation.
- Bridges the Rust client/core logic to mobile apps.

## `toboggan-desktop`

Separate workspace for the native desktop app.

- Uses `iced` and `wgpu`.
- Kept separate to avoid pulling heavy GPU dependencies into the main workspace build.