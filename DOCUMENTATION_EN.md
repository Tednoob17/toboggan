# Toboggan — Documentation (English)

This document consolidates the main project information, quick-start instructions, architecture overview, and per-component summaries. It is derived from the repository README and the existing `docs/` book located at `docs/src/`.

## Overview

Toboggan is a modern, multi-platform presentation system written in Rust. Create slides in Markdown or TOML, serve them with a WebSocket-enabled server, and present from any client — web browser, terminal, desktop, or mobile.

Key features:
- Simple content creation (Markdown or TOML)
- Real-time synchronization via WebSocket
- Multi-platform clients: Web, Terminal, Desktop, iOS, embedded
- Modular Rust workspace with focus on safety and portability

## Quick Start

1. Build the main workspace (CLI, server, TUI):

```bash
git clone https://github.com/Tednoob17/toboggan
cd toboggan
cargo build --release
```

2. Convert a Markdown presentation (optional):

```bash
cargo run -p toboggan-cli -- examples/riir-flat.md -o my-talk.toml
```

3. Run the server with a presentation:

```bash
cargo run -p toboggan-server -- my-talk.toml
# then open http://localhost:8080
```

4. Use the terminal client to connect:

```bash
cargo run -p toboggan-tui -- http://localhost:8080
```

## Installation and Releases

Prebuilt binaries are available in Releases for Linux, macOS, and Windows. Each release typically includes `toboggan-cli`, `toboggan-server`, and `toboggan-tui`.

On Linux/macOS you can download the tarball, extract and copy the binaries to `~/.local/bin` or `/usr/local/bin`.

There are also `.deb` packages for Debian/Ubuntu on some releases.

## Building (full guide)

Requirements:
- Rust (recommended recent stable version matching Cargo manifests)
- Node.js (for the web frontend build)

Build commands:

```bash
# Main workspace (fast)
cargo build --release

# Desktop (heavy, separate workspace)
cargo build --release --manifest-path toboggan-desktop/Cargo.toml

# Web frontend (from /toboggan-web)
cd toboggan-web && npm install && npm run build && cd -
```

## Architecture (high level)

The project is split into a main workspace and a separate desktop workspace:

- Main workspace (fast build): `toboggan-core`, `toboggan-stats`, `toboggan-server`, `toboggan-cli`, `toboggan-client`, `toboggan-tui`, `toboggan-mobile`, `toboggan-web`
- Desktop workspace (separate): `toboggan-desktop` (iced + wgpu heavy deps)

Design principles:
- WebSocket-based JSON protocol for real-time sync
- No unsafe code (workspace lints deny unsafe)
- Modular core types in `toboggan-core` designed for portability (no_std/alloc/wasm)
- Shared `toboggan-client` library for WebSocket client logic

There is an architecture diagram in the docs (`docs/src/reference/architecture.{svg,png}`) and a narrative in `docs/src/reference/architecture.md`.

## WebSocket Protocol (summary)

Clients send JSON commands to the server (examples):

- Navigation: `{ "type": "Next" }`, `{ "type": "Previous" }`, `{ "type": "Goto", "slide": 5 }`
- Controls: `{ "type": "Play" }`, `{ "type": "Pause" }`, `{ "type": "Resume" }`
- Client management: `{ "type": "Register", "client_id": "id" }`

Server emits state updates and notifications:

- `State` messages containing current slide and runtime state (Init/Paused/Running/Done)
- `Error { message }` for invalid input
- `Pong` in response to `Ping`

See `docs/src/reference/presentation-format.md` for presentation serialization and `docs/src/reference/configuration.md` for server configuration options.

## Clients — quick summaries

- Web (`toboggan-web`): TypeScript frontend that talks to the server and can be built to static assets. A WASM client exists in `toboggan-web/toboggan-wasm`.
- Terminal UI (`toboggan-tui`): TUI presenter/viewer built with `ratatui` and `crossterm`.
- Desktop (`toboggan-desktop`): Native GUI using `iced` and GPU libs.
- Mobile (`toboggan-mobile` / `TobogganApp`): iOS/Android integration using UniFFI for Rust bindings.
- Embedded (`toboggan-esp32`): (excluded from main workspace) experimental ESP32 client.

## Key crates and short descriptions

- `toboggan-core` — Core domain models and types (Talk, Slide, Content, State). Designed for `no_std`/`alloc` and wasm usage.
- `toboggan-cli` — Convert Markdown folders to Toboggan presentation formats (TOML/JSON/YAML/etc.), includes statistics and progressive reveal parsing.
- `toboggan-server` — Axum-based WebSocket + REST server that serves the web UI and broadcasts presentation state.
- `toboggan-client` — Shared WebSocket client helper utilities used by clients.
- `toboggan-tui` — Terminal UI presenter/viewer.
- `toboggan-web` — Web frontend and optional WASM client.
- `toboggan-mobile` — UniFFI bindings and mobile-specific glue code.

For API-level documentation, consult the crate docs on docs.rs or inspect the `src/` directories of each crate.

## Examples

The repository contains several example presentations in `examples/` and example configurations. A simple flow to try locally:

```bash
# convert example md to toml
cargo run -p toboggan-cli -- examples/riir-flat.md -o demo.toml

# serve it
cargo run -p toboggan-server -- demo.toml

# open the UI at http://localhost:8080
```

## CLI Usage (summary)

`toboggan-cli` supports folder-based input, frontmatter in TOML, progressive reveals with `<!-- pause -->`, speaker notes, and multiple output formats. Use `toboggan-cli --help` or see `docs/src/guide/cli.md` for full CLI options and examples.

## Server Usage (summary)

`toboggan-server` exposes:
- Static web UI at `/`
- WebSocket upgrade at `/api/ws`
- Health and metadata endpoints under `/api/*`

Run `toboggan-server --help` or consult `docs/src/guide/server.md` for environment variables, CLI flags, and deployment tips (systemd, nginx reverse proxy, Dockerfile example included in docs).

## Contribution & Development

Development guidelines are described in the main README: format with `cargo fmt`, lint with `cargo clippy`, and run tests with `cargo test`. The workspace enforces `no unsafe` via lints.

If you plan to work on the desktop app, build it in its separate workspace: `cargo build --release --manifest-path toboggan-desktop/Cargo.toml`.

## Where to look next (docs files)

The repository contains a documentation book under `docs/src/`. Relevant pages:

- `docs/src/introduction.md` — project intro and quick commands
- `docs/src/installation.md` — installation details
- `docs/src/guide/*` — user guides for creating presentations and each client
- `docs/src/reference/*` — format, configuration, architecture

## License

Licensed under Apache-2.0 or MIT (your choice). See `LICENSE-APACHE` and `LICENSE-MIT` at project root.

---

This consolidated document is generated automatically from the repository README and the `docs/` book. For authoritative, navigable documentation, use the mdBook HTML produced from `docs/` (see `docs/book.toml`).
