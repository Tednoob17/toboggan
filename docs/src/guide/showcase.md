# Showcase: Rebuilding Trail of Bits Slides

> **Note**: This page shows existing slides rebuilt in Toboggan as a **demonstration**
> of what the format looks like for a real talk. The normal workflow is to write your
> slides directly in Markdown or TOML — there is no need to deconstruct existing PDFs.
> This example is just here to show how a real-world presentation maps to Toboggan's
> format and to give you a starting point for your own slides.

This page demonstrates how Toboggan can reproduce real conference presentations from their source code.

## "How to Fuzz Like a Pro" — DeFi Security Summit 2024

This talk by **Nat Chin** and **Josselin Feist** of Trail of Bits introduces Echidna,
a property-based fuzzer for Ethereum smart contracts. The original PDF is embedded below;
the Toboggan source code that reproduces it is shown alongside.

### Original PDF

<object data="../assets/how-to-fuzz-like-a-pro.pdf" type="application/pdf" width="100%" height="600px">
  <p>Your browser does not support embedded PDFs.
  <a href="../assets/how-to-fuzz-like-a-pro.pdf">Download the original PDF</a>.</p>
</object>

### Complete TOML file

The full file is at `slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml`
(51 slides, 880 lines). Here is the complete content — this is what your own TOML file
should look like:

```toml
title = "How to Fuzz Like a Pro"
date = "2025-05-31"
head = """
<style>
  .toboggan-footer { display: none; }
</style>
"""

# ── Slide 1 (PDF page 1) ──────────────────────────────────
# Simple image: every slide follows this exact pattern.
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_01.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"

# ── Slides 2-13, 15-19, 21-50 (PDF pages 2-13, 15-19, 21-50) ──
# All identical pattern — just change the page_XX.jpg filename.
# ...

# ── Slide 14 (PDF page 14) — with clickable link overlay ──
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_14.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
  <a href="https://github.com/crytic/echidna" target="_blank" rel="noopener"
     title="github.com/crytic/echidna"
     style="position:absolute;left:14%;top:41.8%;width:30.9%;height:5%;cursor:pointer;"></a>
</div>
"""

[slides.notes]
type = "Empty"

# ── Slide 20 (PDF page 20) — with link overlay ──
# ...

# ── Slide 51 (PDF page 51) — with 3 link overlays ──
# ...
```

**Simple slide pattern** — 46 slides out of 51:

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_XX.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

**Slide with clickable overlay** — 5 slides (pages 3, 14, 20, 51):

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_14.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
  <a href="https://github.com/crytic/echidna" target="_blank" rel="noopener"
     title="github.com/crytic/echidna"
     style="position:absolute;left:14%;top:41.8%;width:30.9%;height:5%;cursor:pointer;"></a>
</div>
"""

[slides.notes]
type = "Empty"
```

The `left`/`top`/`width`/`height` are percentage-based coordinates from the
PDF text bounding box (720×405 pt page → CSS %).

### To serve

```bash
toboggan-server \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public" \
  --port 8081 --host 0.0.0.0

# Then open http://localhost:8081
```

---

### How to Run It

```bash
# Serve the TOML file with its public image directory
toboggan-server \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public" \
  --port 8081 --host 0.0.0.0

# Then open http://localhost:8081
```

### How It Was Made

1. The original PDF slides were collected from the
   [Trail of Bits publications repository](https://github.com/trailofbits/publications).
2. Each PDF page was rendered at 3x zoom (2160×1215 px) using PyMuPDF,
   saved as JPEG quality 95.
3. URLs and Twitter handles were detected from PDF text spans and converted
   to clickable overlays with percentage-based positioning.
4. The result is a `slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml`
   file with 51 slides, one per PDF page.

### Image generation script

```python
import fitz
doc = fitz.open("slides.pdf")
zoom = 3.0
mat = fitz.Matrix(zoom, zoom)
for i in range(len(doc)):
    pix = doc[i].get_pixmap(matrix=mat)
    pix.save(f"public/page_{i+1:02d}.jpg", jpg_quality=95)
```

### Files

| File | Description |
|------|-------------|
| `how-to-fuzz-like-a-pro.toml` | Presentation definition (51 slides) |
| `public/page_01.jpg` … `public/page_51.jpg` | Full-page slide renders at 2160×1215 px |
| `docs/src/assets/how-to-fuzz-like-a-pro.pdf` | Original PDF source |

## "Building Secure Smart Contracts" — Trail of Bits Training

The `slides_ex/` directory also includes a multi-part presentation on smart contract
security, reconstructed from Trail of Bits' open-source training material at
[secure-contracts.com](https://secure-contracts.com/).

Unlike the pre-compiled TOML above, this one starts from **Markdown source files**
in a folder structure:

```text
slides_ex/presentations/Building Secure Smart Contracts/
├── _cover.md
├── 01-motivation/
│   ├── _part.md
│   ├── 01-the-landscape.md
│   └── 02-automation-pyramid.md
├── 02-tools/
│   ├── _part.md
│   ├── 01-slither.md
│   └── 02-echidna.md
├── 03-static-analysis/
│   ├── _part.md
│   ├── 01-detecting-reentrancy.md
│   └── 02-slither-python-api.md
├── 04-fuzzing/
│   ├── _part.md
│   ├── 01-writing-invariants.md
│   └── 02-configuration.md
└── 05-conclusion/
    ├── _part.md
    ├── 01-takeaways.md
    └── 02-resources.md
```

Convert and serve it:

```bash
toboggan-cli "slides_ex/presentations/Building Secure Smart Contracts/" -o building_secure.toml
toboggan-server building_secure.toml
```

### Key Features Demonstrated

| Feature | Usage |
|---------|-------|
| **Cover slides** | `_cover.md` at the folder root |
| **Part dividers** | `_part.md` files in subdirectories |
| **Slide ordering** | Numerical prefixes (`01-`, `02-`) control order |
| **Pause points** | `<!-- pause -->` creates step-by-step reveals |
| **Speaker notes** | `<!-- notes -->` for presenter-only content |
| **Code blocks** | Fenced code with language tags for syntax highlighting |
| **Frontmatter** | `+++` delimited TOML for per-slide metadata |
| **Multi-format output** | Convert to `toml`, `json`, `yaml`, or `html` |

## Try It Yourself

The full source for both examples is in the `slides_ex/` directory of the repository.
Clone the repo and experiment:

```bash
git clone https://github.com/Tednoob17/toboggan
cd toboggan

# Try the pre-built talk
cargo run -p toboggan-server -- \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public"

# Or build from markdown sources
cargo run -p toboggan-cli -- "slides_ex/presentations/Building Secure Smart Contracts/" -o my_talk.toml
cargo run -p toboggan-server -- my_talk.toml
```

Then open http://localhost:8080 and present.

## Exporting to PDF

Toboggan does not currently export slides to PDF — it is a live presentation system
designed for real-time, multi-device playback via a WebSocket server. If you need a
PDF version of your slides, you can use your browser's **Print --> Save as PDF** feature
while viewing the presentation at `http://localhost:8080`, or use a tool like
[`wkhtmltopdf`](https://wkhtmltopdf.org/) to render the HTML output.
