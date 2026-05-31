# Showcase: Rebuilding Trail of Bits Slides

> **Note**: This page shows existing slides rebuilt in Toboggan as a **demonstration**
> of what the format can do with real-world content. The normal workflow is to write
> slides directly in Markdown or TOML — there is no need to deconstruct existing PDFs.

This page demonstrates how Toboggan reproduces real conference presentations from PDF.

## "How to Fuzz Like a Pro" — DeFi Security Summit 2024

This talk by **Nat Chin** and **Josselin Feist** of Trail of Bits introduces Echidna,
a property-based fuzzer for Ethereum smart contracts.

### What we built

Each PDF page is rendered as a high-resolution JPEG (2160×1215 px at 3x zoom)
and displayed as a full-width slide image. Links and Twitter handles found in
the PDF are extracted and overlaid as clickable `<a>` elements.

**The result**: 51 slides that look identical to the original PDF, with clickable
links, no titles, no footer bar.

### Live server

```bash
toboggan-server \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --host 0.0.0.0 --port 8080 \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public"
```

Then open http://localhost:8080.

### TOML structure

Each slide is minimal — just an image wrapped in a flex container. Pages with links
have transparent clickable overlays positioned absolutely over the image.

```toml
title = "How to Fuzz Like a Pro"
date = "2025-05-31"

# Custom CSS injected into <head> — hides the footer bar
head = """
<style>
  .toboggan-footer { display: none; }
</style>
"""

[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;
            display:flex;align-items:center;justify-content:center;">
  <img src="public/page_01.jpg"
       style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"

[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;
            display:flex;align-items:center;justify-content:center;">
  <img src="public/page_14.jpg"
       style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
  <a href="https://github.com/crytic/echidna"
     target="_blank" rel="noopener"
     title="github.com/crytic/echidna"
     style="position:absolute;left:14%;top:41.8%;
            width:30.9%;height:5%;cursor:pointer;"></a>
</div>
"""
```

### How images are generated

The PDF pages are rendered using PyMuPDF at 3x zoom (2160×1215 px),
saved as JPEG quality 95 for the best quality/size trade-off:

```python
import fitz
doc = fitz.open("slides.pdf")
zoom = 3.0
mat = fitz.Matrix(zoom, zoom)
for i in range(len(doc)):
    pix = doc[i].get_pixmap(matrix=mat)
    pix.save(f"public/page_{i+1:02d}.jpg", jpg_quality=95)
```

### How clickable links are extracted

URLs and Twitter handles are detected from PDF text spans and positioned
using percentage-based coordinates (relative to the 720×405 pt page):

```python
for span in page.get_text("dict") blocks:
    for url in re.findall(r'https?://\S+', span["text"]):
        bbox = span["bbox"]
        left = bbox[0] / 720 * 100    # → CSS left: %
        top  = bbox[1] / 405 * 100    # → CSS top: %
        w = (bbox[2]-bbox[0])/720*100 # → CSS width: %
        h = (bbox[3]-bbox[1])/405*100 # → CSS height: %
```

The overlay `<a>` tags are inserted into the slide HTML at the correct
percentage positions, making them clickable even though the slide is an image.

### Files

| File | Description |
|------|-------------|
| `how-to-fuzz-like-a-pro.toml` | Presentation definition (51 slides) |
| `public/page_01.jpg` … `public/page_51.jpg` | Full-page slide renders |
| `docs/src/assets/how-to-fuzz-like-a-pro.pdf` | Original PDF source |

### Key details

- **Resolution**: 2160×1215 px per slide (3x PDF zoom)
- **Format**: JPEG quality 95 (~130 KB average per slide)
- **Total size**: ~8.9 MB for all 51 pages
- **Clickable links**: 5 URLs + 1 Twitter handle (`@0xicingdeath`)
- **Footer**: Hidden via `head` CSS injection
- **Slide titles**: Removed (not needed for image-based slides)
