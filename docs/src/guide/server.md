# Server Usage

The server (`toboggan-server`) hosts your presentation and synchronizes
all connected clients in real time via WebSocket.

## Starting the server

```bash
# Basic usage
toboggan-server presentation.toml

# Custom port
toboggan-server --port 9090 presentation.toml

# With hot reload (watch for file changes)
toboggan-server --watch presentation.toml
```

## Connecting clients

Once the server is running, open any client and point it to the server URL:

| Client | Command / URL |
|--------|---------------|
| **Web** | `http://localhost:8080` |
| **TUI** | `toboggan-tui http://localhost:8080` |
| **Desktop** | Launches and auto-connects |

## API endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/talk` | GET | Get current talk state |
| `/api/command` | POST | Send a command (`NextSlide`, `PrevSlide`, `GotoSlide`, etc.) |
| `/api/commands` | POST | Send multiple commands at once |
| `/ws` | WebSocket | Real-time sync and notifications |
| `/doc` | GET | Interactive API documentation (Scalar UI) |

## Available commands

```json
{ "command": "NextSlide" }
{ "command": "PrevSlide" }
{ "command": "GotoSlide", "params": { "index": 0 } }
```

## Systemd service (Linux)

```ini
[Unit]
Description=Toboggan Presentation Server
After=network.target

[Service]
ExecStart=/usr/local/bin/toboggan-server /path/to/presentation.toml
Restart=on-failure
User=youruser

[Install]
WantedBy=multi-user.target
```
