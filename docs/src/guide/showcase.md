# Showcase: Rebuilding Trail of Bits Slides

This page shows how Toboggan can reproduce a real PDF presentation as a TOML slide deck.

## "How to Fuzz Like a Pro" — DeFi Security Summit 2024

### Original PDF

<object data="../assets/how-to-fuzz-like-a-pro.pdf" type="application/pdf" width="100%" height="600px">
  <p>Your browser does not support embedded PDFs.
  <a href="../assets/how-to-fuzz-like-a-pro.pdf">Download the original PDF</a>.</p>
</object>

### TOML code slide by slide

The full file is `slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml`.
Each PDF page becomes one slide. Images are in `public/`.

#### Slide 1 → PDF page 1 (Cover)

```toml
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
```

#### Slide 2 → PDF page 2 (Speaker: Nat Chin)

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_02.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

#### Slide 3 → PDF page 3 (Speaker: Josselin Feist, with @0xicingdeath link)

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_03.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
  <a href="https://x.com/0xicingdeath" target="_blank" rel="noopener"
     title="x.com/0xicingdeath"
     style="position:absolute;left:25.9%;top:29.9%;width:18.1%;height:5.3%;cursor:pointer;"></a>
</div>
"""

[slides.notes]
type = "Empty"
```

#### Slides 4-13 → PDF pages 4-13 (Presentation body — simple images)

All follow the same pattern as slide 1, just with `page_04.jpg` through `page_13.jpg`.

#### Slide 14 → PDF page 14 (Echidna overview, with link)

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

#### Slides 15-19 → PDF pages 15-19 (Presentation body — simple images)

#### Slide 20 → PDF page 20 (Exercise link)

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_20.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
  <a href="https://github.com/crytic/building-secure-contracts/blob/master/program-analysis/echidna/Exercise-1.md"
     target="_blank" rel="noopener"
     style="position:absolute;left:9%;top:78%;width:77.2%;height:3.1%;cursor:pointer;"></a>
</div>
"""

[slides.notes]
type = "Empty"
```

#### Slides 21-50 → PDF pages 21-50 (Presentation body — simple images)

#### Slide 51 → PDF page 51 (Conclusion, 3 links)

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_51.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
  <a href="https://github.com/crytic/echidna" target="_blank" rel="noopener"
     style="position:absolute;left:14%;top:29.9%;width:37.5%;height:4.7%;cursor:pointer;"></a>
  <a href="https://github.com/crytic/building-secure-contracts" target="_blank" rel="noopener"
     style="position:absolute;left:28.9%;top:34.9%;width:47.1%;height:4.5%;cursor:pointer;"></a>
  <a href="https://jobs.lever.co/trailofbits" target="_blank" rel="noopener"
     style="position:absolute;left:22.8%;top:68%;width:31.8%;height:4.5%;cursor:pointer;"></a>
</div>
"""

[slides.notes]
type = "Empty"
```

### Global config (top of the file)

```toml
title = "How to Fuzz Like a Pro"
date = "2025-05-31"
head = """
<style>
  .toboggan-footer { display: none; }
</style>
"""
```

### To serve

```bash
toboggan-server \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public" \
  --port 8081 --host 0.0.0.0
```
