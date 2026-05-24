# CLI Usage

The CLI (`toboggan-cli`) converts Markdown presentations to TOML format.

## Basic usage

```bash
# Convert a markdown presentation to TOML
toboggan-cli slides/ -f toml -o presentation.toml

# Convert to HTML (standalone)
toboggan-cli slides/ -f html -o presentation.html

# Show statistics about a presentation
toboggan-cli slides/ --stats
```

## Commands

| Command | Description |
|---------|-------------|
| `slides/` | Path to a file or directory of markdown slides |
| `-f, --format` | Output format: `toml`, `html`, `stat` |
| `-o, --output` | Output file path |
| `--stats` | Show slide statistics |
| `--export-pdf` | Export to PDF (requires headless Chrome) |

## Folder structure

```
slides/
├── 01-introduction.md
├── 02-concepts.md
│   └── 02-subtopic.md
└── 03-conclusion.md
```

Each markdown file becomes a slide. Folders can be used for nesting.

## Markdown frontmatter

```markdown
---
title: "Slide Title"
duration = "5m"
style = { background_color = "#fff", color = "#333" }
---

Slide content here...
```
