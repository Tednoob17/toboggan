# Creating Presentations

Toboggan talks are stored as TOML and can be generated from Markdown/HTML source folders with `toboggan-cli`.

## Step-by-step: create your first presentation

### 1. Create a new file

```bash
touch my-talk.toml
```

### 2. Write the presentation header

```toml
title = "My First Talk"
date = "2026-05-30"
footer = "Demo footer"
```

These are the core `Talk` fields serialized by `toboggan-core`.

### 3. Add slides

Each slide is a `[[slides]]` entry. Slides appear in the order you write them.

```toml
[[slides]]
kind = "Part"

[slides.title]
type = "Text"
text = "Introduction"
```

A `Part` slide acts as a section divider.

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Welcome!"

[slides.body]
type = "Text"
text = "This is my first slide. Toboggan supports Markdown-like text content."
```

### 4. Add more slides

````toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Key Points"

[slides.body]
type = "Text"
text = "- First point\n- Second point\n- Third point"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Code Demo"

[slides.body]
type = "Html"
raw = "<pre><code class=\"language-rust\">fn hello() { println!(\"Hello Toboggan!\"); }</code></pre>"
alt = "Rust code example"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Thanks!"

[slides.body]
type = "Text"
text = "Thank you for watching!"
````

### 5. Run the server

```bash
toboggan-server my-talk.toml
```

Open `http://localhost:8080` in your browser to see your presentation.

## Full example

```toml
title = "My Talk"
date = "2026-05-30"

[[slides]]
kind = "Part"

[slides.title]
type = "Text"
text = "Part 1"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Welcome!"

[slides.body]
type = "Text"
text = "Hello world!"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Styled Slide"

[slides.style]
classes = ["centered"]
style = "background: #2d3436; color: #dfe6e9;"

[[slides]]
kind = "Part"

[slides.title]
type = "Text"
text = "Part 2"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "With Notes"

[slides.body]
type = "Text"
text = "Slide content here"

[slides.notes]
type = "Text"
text = "Speaker notes — only visible in presenter mode"
```

## Slide fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | No | `Cover`, `Part`, or `Standard` |
| `title` | content | No | Slide title content |
| `body` | content | No | Slide body content |
| `notes` | content | No | Speaker notes |
| `terminals` | array | No | Embedded terminal panes |
| `style` | style | No | CSS classes and inline style |

### Style fields

| Field | Type | Description |
|-------|------|-------------|
| `classes` | Array<String> | CSS classes applied to the slide |
| `style` | String | Inline CSS |

### Notes

```toml
[slides.notes]
type = "Text"
text = "Your speaker notes here"
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
+++
title = "My Slide"
duration = "1m"
+++

Slide content here...
```
