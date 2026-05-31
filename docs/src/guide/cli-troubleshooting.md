# Troubleshooting

This page collects common issues and how to fix them.

## Markdown → TOML conversion

### Paths and quoting

- If your input path contains spaces, always quote it:

```bash
toboggan-cli "My Slides Folder/" -o presentation.toml
```

- The CLI expects a **directory** as input. If you pass a single `.toml` file or a file path, the CLI will error with `NotADirectory` (if it's a file) or produce unexpected output — you don't need to convert a TOML file with the CLI.

### Frontmatter format

- The CLI parses **TOML** frontmatter delimited with `+++` (three pluses). Example:

```markdown
+++
title = "Slide Title"
duration = "5m"
css = "background: #fff; color: #333;"
+++

Slide content here...
```

- Malformed TOML frontmatter will be reported in debug logs. If you use YAML frontmatter (`---`), convert it to TOML or remove it.

### When no slides are processed

- Ensure files have supported extensions (`.md`, `.html`) and are not hidden (names starting with `.`).
- Use `_cover.md` in the root of the presentation folder to provide `title`/`date` metadata, or pass `--title` and `--date` on the CLI.

### Debugging and logs

- For detailed parsing errors and diagnostics, run the CLI with debug logging:

```bash
RUST_LOG=debug cargo run -p toboggan-cli -- "path/to/slides/" -o out.toml
```

- If the CLI exits without writing output, check stderr for messages about skipped slides or parsing errors.

### Quick checklist

- Is the input a directory? (not a single `.toml` or `.md` file)
- Are paths quoted if they contain spaces?
- Are frontmatter blocks `+++` TOML or removed?
- Are files named with `.md`/`.html` and not hidden?

### Output file errors

- If the CLI fails when writing the output file, make sure the parent directory exists. The CLI uses `File::create(path)` which will error if the directory does not exist. Create the target directory first or write to an existing folder:

```bash
mkdir -p /tmp/toboggan-output
toboggan-cli "path/to/slides/" -o /tmp/toboggan-output/presentation.toml
```

## Server

### Server not accessible from other devices

**Symptom**: The server starts but other devices on the network can't connect (connection refused or timeout).

**Cause**: By default, `toboggan-server` binds to `127.0.0.1` (localhost only). This is not reachable from other machines.

**Fix**: Bind to all interfaces with `--host 0.0.0.0`:

```bash
toboggan-server --host 0.0.0.0 --port 8080 my-talk.toml
```

### "address already in use"

**Symptom**: The server fails to start on the default port.

**Fix**: Use a different port or kill the existing process:

```bash
# Use a custom port
toboggan-server --port 9090 my-talk.toml

# Or find and kill the process using port 8080
lsof -i :8080
kill <PID>
```

### Talk file errors

**Symptom**: Server exits immediately with a `Talk` parse error.

**Fix**: Validate the talk file:

```bash
# Check the talk file loads correctly
toboggan-server my-talk.toml

# Common issues:
# - Missing required fields (title, date)
# - Invalid slide kind (use: Cover, Part, Standard)
# - Content type must be "Text" or "Html" (not bare strings)
```

### Images not showing in slides

**Symptom**: Slides with `<img>` tags show broken images.

**Cause**: The server needs to know where to serve static files from.

**Fix**: Use `--public-dir` to point to the folder containing your images:

```bash
toboggan-server --public-dir ./public my-talk.toml
```

Then reference images as `/public/my-image.jpg` in your slide HTML.

### Hot-reload (watch mode) not detecting changes

**Symptom**: The server's `--watch` flag doesn't reload when you edit the talk file.

**Fix**: The watch mode uses filesystem polling. Make sure:
- The file is saved to disk (not just in an editor buffer)
- You're watching the correct file (the one passed to the server)
- Some editors (vim with `set nobackup`) may trigger more reliably than others

```bash
toboggan-server --watch my-talk.toml
```

### CORS errors in the browser

**Symptom**: The browser console shows CORS errors when the web UI connects to the server.

**Fix**: If the web UI is served from a different origin than the server, use `--allowed-origins`:

```bash
toboggan-server --allowed-origins "https://my-cdn.com" my-talk.toml
```

Or allow all origins for development:

```bash
toboggan-server --allowed-origins "*" my-talk.toml
```

### Verify the server is running

```bash
# Health check endpoint
curl http://localhost:8080/api/health

# Should return: OK
# If connection refused, the server is not running or bound to a different host/port.
```

## Presentation format

### Footer not showing or unwanted footer appears

- The Toboggan UI has a built-in footer bar (`.toboggan-footer`). To hide it, inject CSS via the `head` field:

```toml
head = """
<style>
  .toboggan-footer { display: none; }
</style>
"""
```

- The optional `footer` field in the talk header sets a text string in the presentation metadata, not a visual footer.

### Slide kind is invalid

**Symptom**: The server rejects the talk with "unknown variant" for `kind`.

**Fix**: Only these slide kinds are valid: `Cover`, `Part`, `Standard`.

```toml
# Correct
kind = "Part"

# Wrong — "Section" and "Break" don't exist
```

### Content type must be tagged

Each content block is a typed enum. You can't set `body` to a raw string:

```toml
# Wrong — this will not parse
[slides.body]
text = "Hello"

# Correct — use "type" field
[slides.body]
type = "Text"
text = "Hello"

# Or for HTML content
[slides.body]
type = "Html"
raw = "<h1>Hello</h1>"
```

## Web frontend build

### wasm-pack fails with bulk memory errors

**Symptom**: `wasm-pack build` fails with:
```
memory.copy operations require bulk memory operations [--enable-bulk-memory-opt]
```

**Cause**: The Rust compiler generates WASM with bulk memory instructions (`memory.copy`, `memory.fill`),
but the `wasm-opt` tool (from binaryen) requires `--enable-bulk-memory` to process them.

**Fix**: Replace the `wasm-opt` binary with a wrapper script that enables the needed WASM features:

```bash
# Locate the wasm-opt binary (the * wildcard will match the version folder)
cd ~/.cache/.wasm-pack/wasm-opt-*/bin/
mv wasm-opt wasm-opt.real

# Create a wrapper that injects the required flags
cat > wasm-opt << 'EOF'
#!/bin/bash
exec "$(dirname "$0")/wasm-opt.real" --enable-bulk-memory-opt --enable-nontrapping-float-to-int "$@"
EOF
chmod +x wasm-opt
```

Then re-run:
```bash
wasm-pack build --target web --release
```

### wasm-pack not found

**Symptom**: `command not found: wasm-pack`

**Fix**: Download the pre-built binary (faster than `cargo install`):

```bash
# Linux x86_64
curl -sSfL https://github.com/rustwasm/wasm-pack/releases/download/v0.15.0/wasm-pack-v0.15.0-x86_64-unknown-linux-musl.tar.gz \
  -o /tmp/wasm-pack.tar.gz
tar -xzf /tmp/wasm-pack.tar.gz -C /tmp/
cp /tmp/wasm-pack-v0.15.0-x86_64-unknown-linux-musl/wasm-pack ~/.cargo/bin/
```

### Server shows placeholder instead of the web UI

**Symptom**: Opening `http://localhost:8080` shows a plain page saying "Web UI not built".

**Cause**: The server embeds the frontend at compile time. If `toboggan-web/dist/`
doesn't exist when you build the server, it embeds a placeholder.

**Fix**: Build the web frontend, then rebuild the server:

```bash
cd toboggan-web/toboggan-wasm
wasm-pack build --target web --release
cd ..
npm install
npm run build
cd ..
cargo build -p toboggan-server
```

Or use the provided script:
```bash
./scripts/build-web.sh
```

### WebSocket connection retries exhausted

**Symptom**: The web UI shows "reconnecting..." and eventually gives up.

**Fix**: If the server is behind a proxy or on a slow network, the frontend's WebSocket retry settings can be adjusted in `toboggan-web/.env`:

```env
VITE_WS_MAX_RETRIES=10
VITE_WS_INITIAL_RETRY_DELAY=2000
VITE_WS_MAX_RETRY_DELAY=60000
```

Then rebuild the frontend and server.

## TUI client

### TUI says "not a terminal"

**Symptom**: The TUI exits immediately with a "not a terminal" error.

**Cause**: The TUI uses `crossterm` which requires a real TTY. This happens
when running in a CI pipeline, a tool sub-shell (e.g. `$(...)`), or a
detached process.

**Fix**: Run the TUI in a real terminal (SSH works). If testing, use the
web client instead.

### TUI can't connect

**Symptom**: The TUI starts but shows "Connection refused" or "No route to host".

**Fix**: Make sure the server is running and listening on the right address:

```bash
# Check the server is up
curl http://localhost:8080/api/health

# Run TUI with matching host/port
toboggan-tui --host localhost --port 8080
```

If the server is bound to `0.0.0.0`, use the machine's LAN IP to connect from another device:
```bash
toboggan-tui --host 192.168.1.42 --port 8080
```

### TUI keyboard shortcuts not working

**Symptom**: Keys like `→`, `←`, or `g` don't respond.

**Fix**: Make sure the terminal is in focus and supports the required key sequences.
Some terminal multiplexers (tmux, screen) may intercept arrow keys. Try:
- Running the TUI outside tmux/screen
- Using alternate shortcuts (n = next, p = previous, q = quit)
