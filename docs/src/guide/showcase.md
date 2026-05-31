# Showcase: Rebuilding Trail of Bits Slides

This page shows how Toboggan can reproduce a real PDF presentation as a TOML slide deck.

## "How to Fuzz Like a Pro" — DeFi Security Summit 2024

### Original PDF

<object data="../assets/how-to-fuzz-like-a-pro.pdf" type="application/pdf" width="100%" height="600px">
  <p>Your browser does not support embedded PDFs.
  <a href="../assets/how-to-fuzz-like-a-pro.pdf">Download the original PDF</a>.</p>
</object>

### TOML code

The file `slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml`
converts each PDF page into a slide. 46 slides are plain page images, 5 slides have
clickable URL overlays. The full file (880 lines) is shown below:

```toml
title = "How to Fuzz Like a Pro"
date = "2025-05-31"
head = """
<style>
  .toboggan-footer { display: none; }
</style>
"""

# ── Slide 1 ──
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

# ── Slides 2-13, 15-19, 21-50 (same pattern) ──
# ...

# ── Slide 14 — with clickable link ──
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

### To serve

```bash
toboggan-server \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public" \
  --port 8081 --host 0.0.0.0
```
