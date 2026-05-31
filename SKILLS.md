# Skill: PDF → Toboggan TOML Conversion

Extract content from an existing PDF presentation and rebuild it as a Toboggan `.toml` slideset, pixel-perfect.

## Required Tools

```bash
# Python PDF extraction
pip install pymupdf  # provides `fitz` / `import fitz`
```

## Workflow

### 1. Extract Images from PDF

```bash
python3 << 'PYEOF'
import fitz
doc = fitz.open("path/to/talk.pdf")
seen = {}
for pg_num in range(len(doc)):
    page = doc[pg_num]
    for img in page.get_images():
        xref = img[0]
        if xref not in seen:
            seen[xref] = True
            pix = fitz.Pixmap(doc, xref)
            pix.save(f"public/pdf_img_{xref}.jpg")
PYEOF
```

Serve images with `--public-dir public/`.

### 2. Extract Text & Layout (position, color, size, font)

```python
doc = fitz.open("path/to/talk.pdf")
for pg_num in range(len(doc)):
    page = doc[pg_num]
    blocks = page.get_text("dict")["blocks"]
    for b in blocks:
        if b.get("type") == 0:
            for line in b.get("lines", []):
                for span in line.get("spans", []):
                    text = span.get("text", "").strip()
                    if not text: continue
                    x, y = span["origin"]
                    size = span.get("size", 0)
                    col = span.get("color", 0)
                    # Convert int color to hex
                    r = (int(col) >> 16) & 0xFF
                    g = (int(col) >> 8) & 0xFF
                    b2 = int(col) & 0xFF
                    color_hex = f"#{r:02x}{g:02x}{b2:02x}"
                    font = span.get("font", "")
                    print(f"({x:.0f},{y:.0f}) {color_hex} sz={size:.1f} {font} '{text}'")
```

### 3. Map PDF Pages → TOML Slides

Create one `[[slides]]` block per PDF page (51 pages → 51 slides).

| TOML `kind` | When |
|---|---|
| `Cover` | First slide (full background image) |
| `Part` | Section divider (centered text, no body) |
| `Standard` | Everything else (title + body content) |

### 4. Build Every Slide with Inline Styles

Toboggan renders each slide inside a **shadow DOM**, so CSS classes from the page won't apply. **All styles must be inline:**

```html
<div style="display: flex; flex-direction: column; min-height: 40vh;">
  <div style="flex: 1; padding: 0.5em 0;">
    <!-- content -->
  </div>
</div>
```

### 5. Bullet Hierarchy (from PDF extraction)

| Level | Glyph | Color | Size |
|---|---|---|---|
| 1st | `●` (U+25CF) | `#10181F` | 18pt |
| 2nd | `○` (U+25CB) | `#AD2B43` | 16pt |
| 3rd | `■` (U+25A0) | `#000000` | 14pt |

```html
<ul style="list-style: none; padding-left: 0;">
  <li style="margin: 0.3em 0;"><span style="color: #10181F;">●</span> Main point
    <ul style="list-style: none; padding-left: 1.2em;">
      <li style="margin: 0.2em 0;"><span style="color: #AD2B43;">○</span> Sub point
        <ul style="list-style: none; padding-left: 1.2em;">
          <li style="margin: 0.15em 0;"><span style="color: #000000;">■</span> Detail</li>
        </ul>
      </li>
    </ul>
  </li>
</ul>
```

### 6. Colors (Trail of Bits theme)

| Element | Hex |
|---|---|
| Header/footer text | `#8294A3` |
| Headings / section titles | `#AD2B43` |
| Body text | `#10181F` |
| Links / accent | `#0097A7` |
| Code: keywords | `#D73A49` |
| Code: variables | `#24292E` |
| Code: comments | `#6A737D` |
| Code: numbers/literals | `#666666` |
| Code: type/contract names | `#005CC5` |

### 7. Footer

Every content slide needs an inline footer bar:

```html
<div style="display: flex; justify-content: space-between; align-items: center;
            font-size: 8pt; color: #8294A3; padding: 4px 0; margin-top: auto;">
  <span>Conference Name &nbsp;|&nbsp; Talk Title</span>
  <span><img src="/public/logo.jpg" style="width: 18px; height: 18px; vertical-align: middle;"> N</span>
</div>
```

The footer must be part of the body HTML (Toboggan has no built-in footer mechanism).

### 8. Cover Slide

Use a `<img>` tag with the full background image (the PDF cover text is embedded in the image):

```toml
[slides.body]
type = "Html"
raw = """<img src="/public/pdf_img_XX.jpg" alt="..." style="width: 100%; display: block;">"""
```

### 9. Code Syntax Highlighting

Apply inline colors matching extracted PDF:

```html
<pre><code class="language-solidity">
<span style="color: #D73A49;">function</span>
<span style="color: #24292E;">myFunc</span>(
<span style="color: #D73A49;">uint</span> <span style="color: #24292E;">x</span>)
<span style="color: #6A737D;">// comment</span>
</code></pre>
```

### 10. Images in Body

Place images alongside text using `flexbox`:

```html
<div style="display: flex; flex-direction: row; gap: 1em;">
  <div style="flex: 1;"><!-- text --></div>
  <div style="flex: 0 0 auto; max-width: 40%;">
    <img src="/public/img.jpg" style="max-width: 100%;">
  </div>
</div>
```

### 11. Server Command

```bash
toboggan-server \
  --host 0.0.0.0 --port 8081 \
  --public-dir "slides_ex/presentations/Talk Name/public" \
  "slides_ex/presentations/Talk Name/Talk Name.toml"
```

## Example: Full Slide Template

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Slide Title"

[slides.body]
type = "Html"
raw = """
<div style="display: flex; flex-direction: column; min-height: 40vh;">
  <div style="flex: 1; padding: 0.5em 0;">
    <h3 style="color: #AD2B43;">Subheading</h3>
    <ul style="list-style: none; padding-left: 0;">
      <li style="margin: 0.3em 0;"><span style="color: #10181F;">●</span> Item
        <ul style="list-style: none; padding-left: 1.2em;">
          <li style="margin: 0.2em 0;"><span style="color: #AD2B43;">○</span> Sub item</li>
        </ul>
      </li>
    </ul>
  </div>
  <div style="display: flex; justify-content: space-between; align-items: center;
              font-size: 8pt; color: #8294A3; padding: 4px 0; margin-top: auto;">
    <span>Conference &nbsp;|&nbsp; Talk Title</span>
    <span><img src="/public/logo.jpg" style="width: 18px; height: 18px; vertical-align: middle;"> N</span>
  </div>
</div>"""
```
