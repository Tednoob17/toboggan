# Server Usage

`toboggan-server` loads a TOML talk file, serves the browser UI, and keeps all connected clients in sync over WebSocket.

## Starting the server

```bash
# Basic usage
toboggan-server talk.toml

# Custom host and port
toboggan-server --host 0.0.0.0 --port 9090 talk.toml

# Enable watch mode
toboggan-server --watch talk.toml
```

## Connecting clients

Once the server is running, open any client and point it at the server URL:

| Client | Command / URL |
|--------|---------------|
| **Web** | `http://localhost:8080` |
| **TUI** | `cargo run -p toboggan-tui -- http://localhost:8080` |
| **Mobile/Desktop** | Configure the server URL in the app |

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
