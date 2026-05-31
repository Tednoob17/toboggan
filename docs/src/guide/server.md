# Server Usage

`toboggan-server` loads a TOML talk file, serves the browser UI, and keeps all connected clients in sync over WebSocket.

## Starting the server

```bash
# Basic usage — TOML file
toboggan-server talk.toml

# Basic usage — Markdown folder (v0.1.1-beta.2+)
toboggan-server ./slides/

# Custom host and port
toboggan-server --host 0.0.0.0 --port 9090 talk.toml

# Serve presentation images from a local folder
toboggan-server --public-dir ./public talk.toml

# Enable watch mode
toboggan-server --watch talk.toml
```

The `--public-dir` flag serves static files (images, videos, etc.) at the `/public/` URL path.
This is useful for embedding images in your slide HTML with `<img src="/public/my-image.jpg">`.<｜end▁of▁thinking｜>

## Connecting clients

Once the server is running, open any client and point it at the server URL:

| Client | Command / URL |
|--------|---------------|
| **Web** | `http://localhost:8080` |
| **TUI** | `toboggan-tui --host localhost --port 8080` |
| **Mobile/Desktop** | Configure the server URL in the app |

> **Web UI note**: The browser frontend requires building `toboggan-web`
> (Node.js + wasm-pack). See [Web Client build instructions](web.md#building-the-web-frontend).
> Without the frontend, the server shows a placeholder page.
> All API endpoints (`/api/talk`, `/api/ws`) work regardless.

## HTTP and WebSocket endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Browser UI assets |
| `/api/health` | GET | Health check |
| `/api/ws` | GET | WebSocket upgrade endpoint |
| `/api/presentation` | GET | Presentation metadata |

## Protocol flow

1. A client connects to `/api/ws`.
2. The client sends `Register` with a display name.
3. The server replies with `Registered` and the initial `State` notification.
4. Navigation commands (`NextSlide`, `PreviousSlide`, `GoTo`, `First`, `Last`, `NextStep`, `PreviousStep`) update the shared state.
5. The server broadcasts notifications to all connected clients.

## Systemd service (Linux)

```ini
[Unit]
Description=Toboggan Presentation Server
After=network.target

[Service]
ExecStart=/usr/local/bin/toboggan-server /path/to/talk.toml
Restart=on-failure
User=youruser

[Install]
WantedBy=multi-user.target
```
