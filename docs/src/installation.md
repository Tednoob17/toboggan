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

Download individual `.deb` packages:

```bash
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-cli_0.1.0-1_amd64.deb -o toboggan-cli.deb
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-server_0.1.0-1_amd64.deb -o toboggan-server.deb
curl -sSfL https://github.com/Tednoob17/toboggan/releases/latest/download/toboggan-tui_0.1.0-1_amd64.deb -o toboggan-tui.deb
sudo dpkg -i toboggan-cli.deb toboggan-server.deb toboggan-tui.deb
sudo apt-get install -f
```

## Build from source

### Prerequisites

| Component | Requires | Notes |
|-----------|----------|-------|
| CLI + server + TUI | [Rust](https://rustup.rs) (stable) | Main workspace |
| Desktop app | Rust + GPU drivers | Separate workspace |
| Web frontend | Rust + Node.js + wasm-pack + WASM target | Optional, server works without it |

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### 2. Install wasm-pack

> Do not use `cargo install wasm-pack` (very slow). Use the pre-built binary instead.

**Linux (x86_64):**
```bash
curl -sSfL https://github.com/rustwasm/wasm-pack/releases/download/v0.15.0/wasm-pack-v0.15.0-x86_64-unknown-linux-musl.tar.gz \
  -o /tmp/wasm-pack.tar.gz
tar -xzf /tmp/wasm-pack.tar.gz -C /tmp/
cp /tmp/wasm-pack-v0.15.0-x86_64-unknown-linux-musl/wasm-pack ~/.cargo/bin/
wasm-pack --version
```

**macOS (Intel):**
```bash
curl -sSfL https://github.com/rustwasm/wasm-pack/releases/download/v0.15.0/wasm-pack-v0.15.0-x86_64-apple-darwin.tar.gz \
  -o /tmp/wasm-pack.tar.gz
tar -xzf /tmp/wasm-pack.tar.gz -C /tmp/
cp /tmp/wasm-pack-v0.15.0-x86_64-apple-darwin/wasm-pack ~/.cargo/bin/
wasm-pack --version
```

**macOS (Apple Silicon):**
```bash
curl -sSfL https://github.com/rustwasm/wasm-pack/releases/download/v0.15.0/wasm-pack-v0.15.0-aarch64-apple-darwin.tar.gz \
  -o /tmp/wasm-pack.tar.gz
tar -xzf /tmp/wasm-pack.tar.gz -C /tmp/
cp /tmp/wasm-pack-v0.15.0-aarch64-apple-darwin/wasm-pack ~/.cargo/bin/
wasm-pack --version
```

**Windows:** Download the `.exe` from the [releases page](https://github.com/rustwasm/wasm-pack/releases/tag/v0.15.0) and add it to your PATH.

### 3. Install Node.js

Required for building the web frontend. Install via [nvm](https://github.com/nvm-sh/nvm) or from your package manager:

```bash
node --version  # needs 18+
npm --version
```

### 4. Build everything

> **What does `--release` mean?** Rust has two build profiles:
> - **Debug** (`cargo build`): compiles fast, runs slow — good for development
> - **Release** (`cargo build --release`): compiles slow (minutes), runs fast — good for final use
>
> We use `--release` below for the final binaries. For quick testing, you can omit it.

```bash
# Clone
git clone https://github.com/Tednoob17/toboggan
cd toboggan

# Build CLI + server + TUI (main workspace)
cargo build --release

# Build desktop app (separate workspace)
cargo build --release --manifest-path toboggan-desktop/Cargo.toml

# Build web frontend (optional — server works without it)
cd toboggan-web/toboggan-wasm
wasm-pack build --target web --release
cd ..
npm install
npm run build
cd ..

# Rebuild server with web UI embedded (required after web frontend build)
cargo build --release -p toboggan-server
```

> **Tip**: Add `--release` for optimized builds. Without it, you get debug builds
> (faster to compile but slower to run). Binaries are in `target/release/`
> (with `--release`) or `target/debug/` (without).

### Or use the build script

```bash
./scripts/build-web.sh
```

This automates all the steps above and checks that prerequisites are installed.

### Troubleshooting

For common issues (wasm-opt errors, missing dependencies, server placeholder, TUI connection problems),
see the [Troubleshooting page](guide/cli-troubleshooting.md).
