# Creating Presentations

Presentations in Toboggan are written in **TOML** format. You can also write
slides in **Markdown** and convert them with the CLI.

## Step-by-step: create your first presentation

### 1. Create a new file

```bash
touch my-talk.toml
```

### 2. Write the presentation header

```toml
[presentation]
title = "My First Talk"
author = "Your Name"
description = "A demo presentation for Toboggan"
```

This section defines the overall presentation metadata.

### 3. Add slides

Each slide is a `[[slides]]` entry. Slides appear in the order you write them.

```toml
[[slides]]
kind = "Part"
title = "Introduction"
```

A `Part` slide acts as a section divider.

```toml
[[slides]]
title = "Welcome!"
body = '''
This is my first slide.

Toboggan supports **bold**, *italic*, and `code` in Markdown.
'''
```

### 4. Add more slides

```toml
[[slides]]
title = "Key Points"
body = '''
- First point
- Second point
- Third point
'''

[[slides]]
title = "Code Demo"
body = '''
Here is some Rust code:

```rust
fn hello() {
    println!("Hello Toboggan!");
}
```
'''

[[slides]]
title = "Thanks!"
body = "Thank you for watching!"
```

### 5. Run the server

```bash
toboggan-server my-talk.toml
```

Open `http://localhost:8080` in your browser to see your presentation.

## Full example

```toml
[presentation]
title = "My Talk"
author = "Your Name"
description = "A short description"

[[slides]]
kind = "Part"
title = "Part 1"

[[slides]]
title = "Welcome!"
body = "Hello world!"

[[slides]]
title = "Styled Slide"

[slides.style]
background_color = "#2d3436"
color = "#dfe6e9"

[[slides]]
kind = "Part"
title = "Part 2"

[[slides]]
title = "With Notes"

body = "Slide content here"

[slides.notes]
body = "Speaker notes — only visible in presenter mode"
```

## Slide fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | No | `"Slide"` (default) or `"Part"` |
| `title` | string | Yes | Slide title |
| `body` | string | No | Markdown content |
| `duration` | duration | No | Auto-advance time (e.g. `"30s"`, `"2m"`) |

### Style fields

| Field | Type | Description |
|-------|------|-------------|
| `background_color` | string | CSS color for background |
| `color` | string | CSS color for text |
| `font_size` | integer | Font size in pixels |

### Notes

```toml
[slides.notes]
body = "Your speaker notes here"
```

Notes are visible in presenter mode and are not shown to the audience.

## Converting Markdown to TOML

If you prefer writing in Markdown:

```bash
# Convert a folder of markdown files to TOML
toboggan-cli slides/ -f toml -o presentation.toml
```

Your folder structure:

```
slides/
├── 01-intro.md
├── 02-details.md
└── 03-end.md
```

Each Markdown file becomes a slide. The frontmatter of each file sets
the slide metadata:

```markdown
---
title: "My Slide"
duration = "1m"
---

Slide content here...
```
