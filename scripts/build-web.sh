#!/bin/bash
set -euo pipefail

echo "=== Building Toboggan Web Frontend ==="

# Step 1: Check prerequisites
echo ""
echo "--- Checking prerequisites ---"

if ! command -v cargo &>/dev/null; then
  echo "ERROR: rust/cargo not found. Install from https://rustup.rs"
  exit 1
fi

if ! rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
  echo "Adding wasm32-unknown-unknown target..."
  rustup target add wasm32-unknown-unknown
fi

if ! command -v wasm-pack &>/dev/null; then
  echo "ERROR: wasm-pack not found."
  echo "Install it from: https://rustwasm.github.io/wasm-pack/installer/"
  echo "Or download a pre-built binary:"
  echo "  curl -sSfL https://github.com/rustwasm/wasm-pack/releases/download/v0.15.0/wasm-pack-v0.15.0-x86_64-unknown-linux-musl.tar.gz \\"
  echo "    -o /tmp/wasm-pack.tar.gz"
  echo "  tar -xzf /tmp/wasm-pack.tar.gz -C /tmp/"
  echo "  cp /tmp/wasm-pack-v0.15.0-x86_64-unknown-linux-musl/wasm-pack ~/.cargo/bin/"
  exit 1
fi
echo "  ✓ wasm-pack: $(wasm-pack --version)"

if ! command -v node &>/dev/null; then
  echo "ERROR: node not found. Install from https://nodejs.org (version 18+)"
  exit 1
fi
echo "  ✓ node: $(node --version)"
echo "  ✓ npm:  $(npm --version)"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Step 2: Build WASM crate
echo ""
echo "--- Step 1/3: Building WASM crate ---"
cd "$PROJECT_DIR/toboggan-web/toboggan-wasm"
wasm-pack build --target web --release
echo "  ✓ WASM crate built"

# Step 3: Install npm deps (if needed)
echo ""
echo "--- Step 2/3: Building TypeScript frontend ---"
cd "$PROJECT_DIR/toboggan-web"
if [ ! -d node_modules ]; then
  echo "  Installing npm dependencies..."
  npm install
fi
npm run build
echo "  ✓ Frontend built"

# Step 4: Rebuild server
echo ""
echo "--- Step 3/3: Rebuilding server ---"
cd "$PROJECT_DIR"
cargo build -p toboggan-server
echo "  ✓ Server rebuilt with embedded UI"

echo ""
echo "=== Done! ==="
echo "Run the server:"
echo "  ./target/debug/toboggan-server --host 0.0.0.0 --port 8080 path/to/talk.toml"
echo "Then open: http://localhost:8080"
