# CLI Usage

`toboggan-cli` converts a folder of Markdown/HTML slides into a Toboggan talk file and prints statistics.

## Basic usage

```bash
# Convert a source folder to TOML
toboggan-cli slides/ -o presentation.toml

# Emit JSON instead
toboggan-cli slides/ -f json -o presentation.json

# List available syntax highlighting themes
toboggan-cli --list-themes
```

See [CLI Troubleshooting](guide/cli-troubleshooting.md) for common issues and fixes.

## Arguments and options

| Command | Description |
|---------|-------------|
| `slides/` | Input directory containing your presentation source files |
| `-o, --output` | Output file path (default: stdout) |
| `-t, --title` | Override the talk title |
| `-d, --date` | Override the talk date (`YYYY-MM-DD`) |
| `-f, --format` | Output format: `toml`, `json`, `yaml`, or `html` |
| `--theme` | Syntax highlighting theme |
| `--list-themes` | Print the available themes and exit |
| `--no-counter` | Disable automatic part/slide numbering |
| `--no-stats` | Skip the stats summary |
| `--wpm` | Override the speaking rate used for duration estimates |
| `--exclude-notes-from-duration` | Remove speaker notes from duration estimates |

## Input layout

The CLI expects a source folder with markdown or HTML files:

```text
slides/
├── _cover.md
├── _footer.html
├── _head.html
├── 01-introduction/
│   ├── _part.md
│   └── 01-welcome.md
└── 02-deep-dive/
    └── 01-details.md
```

- `_cover.md` sets the title/date metadata.
- `_footer.html` provides a custom HTML footer (repeated on every slide).
- `_head.html` provides custom HTML to inject into the `<head>` of the output.
- `_part.md` creates a section divider.
- Files are processed in alphabetical order.
- Hidden files (names starting with `.`) are ignored.

## Frontmatter

Source slides can include TOML frontmatter delimited by `+++`.

```markdown
+++
title = "Slide Title"
duration = "5m"
+++

# Slide content
```

For richer examples and failure modes, see the troubleshooting page.
