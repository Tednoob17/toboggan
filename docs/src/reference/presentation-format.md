# Presentation Format

The TOML presentation format is the native format for Toboggan.

## Full example

```toml
[presentation]
title = "My Talk"
author = "Jane Doe"
description = "An example presentation"
version = "0.1.0"

[[slides]]
kind = "Part"
title = "Part 1: Introduction"

[[slides]]
title = "Welcome"
body = '''
Welcome to this presentation!

This is a multi-line text block.
'''

[slides.style]
background_color = "#2d3436"
color = "#dfe6e9"

[[slides.notes]]
body = "Speaker notes go here"

[[slides]]
title = "Code Example"
body = '''
Here is some Rust code:

```rust
fn hello() {
    println!("Hello!");
}
```
'''

[[slides]]
kind = "Part"
title = "Part 2: Deep Dive"

[[slides]]
title = "Key Concepts"
body = '''
- First concept
- Second concept
- Third concept
'''
```

## Field reference

### Presentation

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String | Yes | Presentation title |
| `author` | String | No | Author name |
| `description` | String | No | Short description |

### Slide

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String | No | `"Slide"` (default) or `"Part"` |
| `title` | String | Yes | Slide title |
| `body` | String | No | Slide body content (Markdown) |
| `duration` | Duration | No | Auto-advance timer |

### Style

| Field | Type | Description |
|-------|------|-------------|
| `background_color` | String | CSS color |
| `color` | String | Text color |
| `font_size` | Integer | Font size in pixels |

## Duration format

Durations can be specified as:

| Format | Example | Result |
|--------|---------|--------|
| Seconds | `30s` | 30 seconds |
| Minutes | `2m` | 2 minutes |
| Hours | `1h` | 1 hour |
| Combined | `1m 30s` | 1 minute 30 seconds |
