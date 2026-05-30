# CLI Troubleshooting

This page collects common issues encountered when using `toboggan-cli` and how to fix them.

## Paths and quoting

- If your input path contains spaces, always quote it:

```bash
toboggan-cli "My Slides Folder/" -o presentation.toml
```

- The CLI expects a **directory** as input. If you pass a single `.toml` file, the CLI will error with `NotADirectory` — you don't need to convert a TOML file with the CLI.

## Frontmatter format

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

## When no slides are processed

- Ensure files have supported extensions (`.md`, `.html`) and are not hidden (names starting with `.`).
- Use `_cover.md` in the root of the presentation folder to provide `title`/`date` metadata, or pass `--title` and `--date` on the CLI.

## Debugging and logs

- For detailed parsing errors and diagnostics, run the CLI with debug logging:

```bash
RUST_LOG=debug cargo run -p toboggan-cli -- "path/to/slides/" -o out.toml
```

- If the CLI exits without writing output, check stderr for messages about skipped slides or parsing errors.

## Quick checklist

- Is the input a directory? (not a `.toml` file)
- Are paths quoted if they contain spaces?
- Are frontmatter blocks `+++` TOML or removed?
- Are files named with `.md`/`.html` and not hidden?

If you'd like, paste the exact command you ran and I will reproduce the run locally and explain the specific error messages.

## Output file errors

- If the CLI fails when writing the output file, make sure the parent directory exists. The CLI uses `File::create(path)` which will error if the directory does not exist. Create the target directory first or write to an existing folder:

```bash
mkdir -p /tmp/toboggan-output
toboggan-cli "path/to/slides/" -o /tmp/toboggan-output/presentation.toml
```
