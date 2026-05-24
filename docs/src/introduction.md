# Toboggan

**Toboggan** is a modern, multi-platform presentation system built in Rust.
Write your slides in Markdown or TOML, serve them via a WebSocket-enabled server,
and present from any client — web browser, terminal, desktop app, or mobile device.

## Inspiration

The name *Toboggan* comes from the French word for *slides* (as in a
slide deck). It's a project originally started by **Guillaume Lagrange**
(you-know-who), which I took over to advance and evolve into the
multi-platform system it is today.

Inspired by the Italian animated series *Huntik*, Toboggan lets you become a
**Seeker** of knowledge. Your presentations are your **Titans** — powerful,
dynamic, and under your control.

## Features

- **Simple content creation** — Write presentations in Markdown or TOML
- **Real-time synchronization** — Multi-client sync via WebSocket
- **Multi-platform clients** — Web, terminal, desktop, iOS, embedded
- **Zero external dependencies** — Just a single binary per component

## Quick start

```bash
# 1. Start the server with a presentation
toboggan-server examples/demo.toml

# 2. Open the web UI
#    → http://localhost:8080

# 3. Control from the terminal
toboggan-tui http://localhost:8080
```
