# Developer Overview

This document summarizes Toboggan's architecture, main crates, core types, runtime model, and how to build and extend the project.

## Project at a glance

- Multi-platform presentation system written in Rust
- Real-time synchronization via WebSockets
- Organized as a main workspace (fast build) and a separate `toboggan-desktop` workspace (heavier GPU deps)

## Workspace layout

- `toboggan-core` — Core domain types: `Talk`, `Slide`, `Content`, `State`, `Command`, `Notification`.
- `toboggan-cli` — Folder-to-presentation converter and presentation statistics tooling.
- `toboggan-server` — Axum-based WebSocket + REST server, state manager, client registry.
- `toboggan-client` — Shared WebSocket client utilities used by various clients.
- `toboggan-tui` — Terminal client based on `ratatui` and `crossterm`.
- `toboggan-web` / `toboggan-wasm` — TypeScript frontend and optional WASM client.
- `toboggan-mobile` / `TobogganApp` — iOS/Android integration via UniFFI and SwiftUI frontend.
- `toboggan-desktop` — Separate workspace using `iced` + `wgpu` (compiles independently).

## Core concepts and types

### `Talk`
- Container for a presentation: `title`, `date`, optional `head`/`footer`, and `slides: Vec<Slide>`.
- `Talk::new`, builder-style `with_date`, `with_footer`, `add_slide`.

### `Slide` and `SlideId`
- `Slide` contains `kind`, `style`, `title`, `body`, `notes`, and optional `terminals`.
- `SlideId` is a typed wrapper around `usize` with helpers: `FIRST`, `index()`, `prev()`, `display_number()`.

### `Content`
- Enum representing slide content: `Empty`, `Text`, `Html { raw, alt, style }`.
- Helper constructors: `Content::text`, `Content::html`, `Content::html_with_alt`.

### Runtime `State`
- States: `Init`, `Running { current, current_step }`, `Done { current, current_step }`.
- Helpers: `current()`, `current_step()`, `next()`, `previous()`, `is_first_slide()`, `is_last_slide()`, `update_slide()`.

### Protocol: `Command` and `Notification`
- `Command` (client→server): `Register`, `Unregister`, `Ping`, navigation (`First`, `Last`, `GoTo`, `NextSlide`, `PreviousSlide`), `NextStep`/`PreviousStep`, effects like `Blink`.
- `Notification` (server→clients): `State`, `Error`, `Pong`, `Blink`, `TalkChange`, `Registered`, `ClientConnected`, `ClientDisconnected`.

## Server runtime model

- `TobogganState` composes `TalkService` and `ClientService` and coordinates commands and broadcasts.
- WebSocket flow: clients `Register` → server sends initial `State` → client exchanges `Command`s → server `handle_command` → broadcasts `Notification`s.
- Concurrency: async tasks per connection (watcher, sender, receiver, heartbeat). Shared state uses thread-safe services and channels.

## CLI behavior highlights

- `toboggan-cli` parses folders into slides, supports frontmatter TOML, progressive reveals via `<!-- pause -->`, speaker notes, and many output formats (TOML, JSON, YAML, CBOR, MessagePack, bincode).
- Provides presentation statistics and optional numbering of parts/slides.

## Build & run (developer quick commands)

```bash
# Build main workspace (fast)
cargo build --release

# Build server and run with example presentation
cargo run -p toboggan-server -- examples/riir-flat-output.toml

# Convert a folder to a talk
cargo run -p toboggan-cli -- examples/riir-flat.md -o /tmp/my-talk.toml

# Run terminal client
cargo run -p toboggan-tui
```

## Tests, formatting and linting

- Run tests: `cargo test` or `cargo nextest run` for parallel execution.
- Format: `cargo fmt` — Lints: `cargo clippy`.

## Where to look in code

- Core types: `toboggan-core/src/` (`talk.rs`, `slide.rs`, `content.rs`, `state.rs`, `command.rs`, `notification.rs`).
- CLI: `toboggan-cli/src/` (`main.rs`, `lib.rs`, `parser/`, `output/`).
- Server: `toboggan-server/src/` (`router/`, `services/`, `state.rs`, `watcher.rs`).

## Next steps and contribution ideas

- Add more examples for the WASM and mobile clients.
- Expand OpenAPI coverage (feature `openapi`) to document HTTP endpoints.
- Add more integration tests that run server + headless client to validate protocol.

---

If you want, I can:

- expand this into separate per-crate developer pages (APIs, examples),
- generate a sequence diagram for the WebSocket protocol,
- or open a PR with the new docs and README updates.

Tell me which next step you prefer.
