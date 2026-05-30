# Configuration

## Server configuration

`toboggan-server` accepts the following CLI options:

```bash
toboggan-server [OPTIONS] <TALK>

Options:
  --host <IP>                  Host to bind to [default: 127.0.0.1]
  --port <PORT>                Port to bind to [default: 8080]
  --max-clients <N>            Maximum number of WebSocket clients [default: 100]
  --heartbeat-interval-secs <N> Heartbeat interval [default: 30]
  --shutdown-timeout-secs <N>   Graceful shutdown timeout [default: 30]
  --cleanup-interval-secs <N>   Client cleanup interval [default: 60]
  --allowed-origins <LIST>      Comma-separated CORS origins
  --public-dir <DIR>            Static assets directory served at `/public/`
  --watch                      Reload the talk when the file changes
```

These options also read from environment variables:

| Variable | Description |
|----------|-------------|
| `TOBOGGAN_HOST` | Server bind host |
| `TOBOGGAN_PORT` | Server bind port |
| `TOBOGGAN_MAX_CLIENTS` | Maximum number of clients |
| `TOBOGGAN_HEARTBEAT_INTERVAL` | Heartbeat interval in seconds |
| `TOBOGGAN_SHUTDOWN_TIMEOUT` | Shutdown timeout in seconds |
| `TOBOGGAN_CLEANUP_INTERVAL` | Cleanup interval in seconds |
| `TOBOGGAN_CORS_ORIGINS` | Comma-separated list of allowed origins |
| `TOBOGGAN_PUBLIC_DIR` | Optional public assets directory |
| `TOBOGGAN_WATCH` | Enable watch mode |

## Client configuration

`toboggan-core` exposes a reusable client config helper:

```toml
[client]
api_url = "http://localhost:8080"
websocket_url = "ws://localhost:8080/api/ws"

[client.retry]
max_retries = 10
initial_retry_delay = "1s"
max_retry_delay = "30s"
backoff_factor = 2.0
use_jitter = true
```

## Logging

The server and clients use `tracing`. Set `RUST_LOG` to control verbosity:

```bash
RUST_LOG=info toboggan-server talk.toml
RUST_LOG=toboggan_server=debug toboggan-server talk.toml
RUST_LOG=toboggan_client=trace toboggan-tui http://localhost:8080
```
