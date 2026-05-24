# Installation

## Download pre-built binaries

Get the latest release from [github.com/Tednoob17/toboggan/releases](https://github.com/Tednoob17/toboggan/releases).

Each file is named by platform:

| File | Platform |
|------|----------|
| `toboggan-cli-linux-amd64` | Linux x86_64 |
| `toboggan-server-linux-amd64` | Linux x86_64 |
| `toboggan-tui-linux-amd64` | Linux x86_64 |
| `toboggan-desktop-linux-amd64` | Linux x86_64 (desktop GUI) |
| `*macos-amd64` | macOS (Intel) |
| `*macos-arm64` | macOS (Apple Silicon) |
| `*windows-amd64.exe` | Windows x86_64 |
| `*.deb` | Debian / Ubuntu packages |

### Linux

```bash
# Download
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-cli-linux-amd64 -o toboggan-cli

# Make executable and install
chmod +x toboggan-cli
sudo mv toboggan-cli /usr/local/bin/
```

Repeat for `toboggan-server`, `toboggan-tui`, and `toboggan-desktop` as needed.

### macOS

```bash
# Intel Mac
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-cli-macos-amd64 -o toboggan-cli

# Apple Silicon Mac
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-cli-macos-arm64 -o toboggan-cli

chmod +x toboggan-cli
sudo mv toboggan-cli /usr/local/bin/
```

### Windows (PowerShell)

```powershell
curl.exe -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-cli-windows-amd64.exe -o toboggan-cli.exe
# Move to a directory in your PATH
```

### Debian / Ubuntu

```bash
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-cli-linux-amd64.deb -o toboggan-cli.deb
sudo dpkg -i toboggan-cli.deb
```

## Build from source

Requires [Rust](https://rustup.rs) (stable).

```bash
# Clone
git clone https://github.com/Tednoob17/toboggan
cd toboggan

# Build main workspace (CLI + server + TUI)
cargo build --release

# Build desktop app (separate workspace)
cargo build --release --manifest-path toboggan-desktop/Cargo.toml
```

Binaries are in `target/release/`.
