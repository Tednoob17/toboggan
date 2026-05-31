# Troubleshooting

This page collects common issues and how to fix them.

## Markdown → TOML conversion

### Paths and quoting

- If your input path contains spaces, always quote it:

```bash
toboggan-cli "My Slides Folder/" -o presentation.toml
```

- The CLI expects a **directory** as input. If you pass a single `.toml` file, the CLI will error with `NotADirectory` — you don't need to convert a TOML file with the CLI.

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

- Is the input a directory? (not a `.toml` file)
- Are paths quoted if they contain spaces?
- Are frontmatter blocks `+++` TOML or removed?
- Are files named with `.md`/`.html` and not hidden?

### Output file errors

- If the CLI fails when writing the output file, make sure the parent directory exists. The CLI uses `File::create(path)` which will error if the directory does not exist. Create the target directory first or write to an existing folder:

```bash
mkdir -p /tmp/toboggan-output
toboggan-cli "path/to/slides/" -o /tmp/toboggan-output/presentation.toml
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

## Server

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
# Check the talk file is valid TOML
cargo run -p toboggan-cli -- --input my-talk.toml -o /dev/null

# Common issues:
# - Missing required fields (title, slides)
# - Invalid slide kind (use: Cover, Standard, Section, Break)
# - Content type must be "Text" or "Html" (not raw strings)
```
