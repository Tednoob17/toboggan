# Configuration

## Server configuration

The server accepts command-line arguments:

```bash
toboggan-server [OPTIONS] <PRESENTATION>

Options:
  -p, --port <PORT>        Server port [default: 8080]
  -w, --watch              Watch for file changes
      --watch-delay <MS>   Watch debounce delay [default: 200]
      --access-token <TOKEN>  Require token for API access
      --heartbeat <SECS>   Heartbeat interval [default: 30]
      --cleanup <SECS>     Cleanup interval [default: 30]
      --shutdown <SECS>    Shutdown timeout [default: 10]
  -h, --help               Print help
  -V, --version            Print version
```

## Environment variables

| Variable | Description |
|----------|-------------|
| `TOBOGGAN_SKIP_WEB_CHECK=1` | Skip web dist check at build time |
| `RUST_LOG=toboggan_server=debug` | Enable debug logging |

## Retry configuration

The client reconnection behavior can be configured in the presentation
file via the `[client]` section:

```toml
[client]
max_retries = 10
initial_retry_delay = "1s"
max_retry_delay = "30s"
backoff_factor = 2.0
```

## Logging

Toboggan uses the `env_logger` crate. Set `RUST_LOG` to control verbosity:

```bash
RUST_LOG=info toboggan-server talk.toml
RUST_LOG=toboggan_server=debug toboggan-server talk.toml
RUST_LOG=toboggan_client=trace toboggan-tui http://localhost:8080
```
