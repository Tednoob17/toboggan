# TUI Client

`toboggan-tui` is the terminal client built with `ratatui` and `crossterm`.

## Requirements

- A real TTY (terminal). Does **not** work in CI sub-shells or non-interactive contexts.
- Works over SSH, on any Linux/macOS/Windows terminal.

## Running

```bash
# If installed via pre-built binary or cargo install
toboggan-tui --host localhost --port 8080

# If built from source (binary is in target/debug/ or target/release/)
./target/debug/toboggan-tui --host localhost --port 8080
```

## Controls

| Key | Action |
|-----|--------|
| `→` / `n` | Next slide |
| `←` / `p` | Previous slide |
| `q` / `Ctrl+C` | Quit |
| `g` | Go to slide |
| `r` | Refresh / reconnect |
| `?` | Toggle help |

## What it’s good for

- Presenting from SSH or a local terminal.
- Controlling a presentation without opening a browser.
- Reusing the same server protocol as every other client.
