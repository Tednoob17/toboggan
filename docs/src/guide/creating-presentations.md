# Creating Presentations

Presentations in Toboggan are written in **TOML** format. You can also write
slides in **Markdown** and convert them with the CLI.

## Anatomy of a presentation

```toml
[presentation]
title = "My Talk"
author = "Your Name"
description = "A short description of the talk"

[[slides]]
kind = "Part"
title = "Introduction"

[[slides]]
title = "Welcome!"
body = '''
This is a simple slide with text content.
'''
```

## Slide kinds

| Kind | Description |
|------|-------------|
| `Part` | Section divider, shows part number |
| `Slide` | Regular content slide |

## Content types

Each slide has a `title`, `body`, and optionally `notes` (speaker notes).

```toml
[[slides]]
title = "Code example"
body = '''
```rust
fn main() {
    println!("Hello, Toboggan!");
}
```
'''

[slides.notes]
body = "Explain this code step by step"
```

## Adding style

```toml
[[slides]]
title = "Styled slide"

[slides.style]
background_color = "#1a1a2e"
color = "#e94560"
font_size = 32
```

## Converting Markdown to TOML

```bash
toboggan-cli slides/ -f toml -o presentation.toml
```

This reads a folder of Markdown files and produces a TOML presentation.
