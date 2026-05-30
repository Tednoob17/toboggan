# Presentation Format

Toboggan serializes talks as TOML using the `Talk`, `Slide`, `Content`, and `TerminalConfig` types from `toboggan-core`.

## Canonical shape

```toml
title = "My Talk"
date = "2026-05-30"
footer = "Optional footer"
head = "Optional HTML head fragment"

[[slides]]
kind = "Part"

[slides.title]
type = "Text"
text = "Part 1: Introduction"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Welcome"

[slides.body]
type = "Text"
text = "Welcome to this presentation!"
```

## Talk fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String | Yes | Presentation title |
| `date` | Date | Yes | Presentation date (`YYYY-MM-DD`) |
| `footer` | String | No | Optional global footer |
| `head` | String | No | Optional HTML inserted into the page head |
| `slides` | Array | Yes | Ordered slide list |

## Slide fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String | No | `Cover`, `Part`, or `Standard` |
| `title` | `Content` | No | Slide title content |
| `body` | `Content` | No | Slide body content |
| `notes` | `Content` | No | Speaker notes |
| `style` | Style | No | CSS classes or inline style |
| `terminals` | Array | No | Embedded terminal configurations |

## Content variants

`Content` is serialized as a tagged enum:

| Variant | Shape | Use |
|---------|-------|-----|
| `Empty` | omitted/default | No content |
| `Text` | `{ "type": "Text", "text": "..." }` | Plain text or Markdown text |
| `Html` | `{ "type": "Html", "raw": "...", "alt": "..." }` | Rich HTML with optional accessibility fallback |

## Style fields

| Field | Type | Description |
|-------|------|-------------|
| `classes` | Array<String> | CSS classes |
| `style` | String | Inline CSS |

## Embedded terminals

Slides can embed one or more terminal panes via `TerminalConfig`.

| Field | Type | Description |
|-------|------|-------------|
| `cwd` | String | Working directory |
| `theme` | `dark` / `light` | Terminal theme |
| `cmd` | String | Optional command to run |

The markdown parser recognizes terminal blocks like `<!-- term: path/to/cwd -->` and variants with `:light` or `| command`.

## Time values

- Presentation dates use `YYYY-MM-DD`.
- Durations in other config structures serialize as human-readable strings such as `30s`, `2m`, or `1m 30s`.
