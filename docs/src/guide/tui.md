# TUI Client

The terminal UI client (`toboggan-tui`) allows you to view and control
presentations from the terminal, using [Ratatui](https://ratatui.rs/).

## Running

```bash
# Connect to a running server
toboggan-tui http://localhost:8080
```

## Controls

| Key | Action |
|-----|--------|
| `→` / `n` | Next slide |
| `←` / `p` | Previous slide |
| `q` / `Ctrl+C` | Quit |
| `g` | Go to slide (type number) |
| `r` | Refresh / reconnect |
| `?` | Toggle help |

## Features

- Real-time slide display with syntax highlighting
- Speaker notes view (with presenter mode)
- Connection status indicator
- Works over SSH — present remotely from any terminal

## Example

```bash
# On the presentation machine
toboggan-server talk.toml

# On any machine connected to the same network
toboggan-tui http://192.168.1.42:8080
```
