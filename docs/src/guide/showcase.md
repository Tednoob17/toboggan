# Showcase: Rebuilding Trail of Bits Slides

This page demonstrates how Toboggan can reproduce real conference presentations from their source code.

## "How to Fuzz Like a Pro" — DeFi Security Summit 2024

This talk by **Nat Chin** and **Josselin Feist** of Trail of Bits introduces Echidna,
a property-based fuzzer for Ethereum smart contracts. The original PDF is embedded below;
the Toboggan source code that reproduces it is on the right.

### Original PDF

<object data="../assets/how-to-fuzz-like-a-pro.pdf" type="application/pdf" width="100%" height="600px">
  <p>Your browser does not support embedded PDFs.
  <a href="../assets/how-to-fuzz-like-a-pro.pdf">Download the original PDF</a>.</p>
</object>

### Toboggan Source Code

The talk was reconstructed as a Toboggan TOML file from the original slide content.
Each slide maps to a `[[slides]]` entry — the cover, speaker intro, technical content,
code examples, and conclusion:

```toml
title = "How to Fuzz Like a Pro"
date = "2024-06-12"

[[slides]]
kind = "Cover"

[slides.style]
classes = ["no_title", "cover"]

[slides.title]
type = "Text"
text = "How to Fuzz Like a Pro"

[slides.body]
type = "Html"
raw = """
<h1>How to Fuzz Like a Pro</h1>
<blockquote>
<p>Property-based testing for smart contracts with Echidna</p>
</blockquote>
<p style="margin-top: 2em;"><strong>Nat Chin</strong> &amp; <strong>Josselin Feist</strong></p>
<p>DeFi Security Summit — 2024</p>
"""
alt = """
# How to Fuzz Like a Pro
> Property-based testing for smart contracts with Echidna

**Nat Chin** & **Josselin Feist**
DeFi Security Summit — 2024
"""
```

Support for **step-by-step reveals**, **speaker notes**, **code highlighting**, and
**multi-format content** (HTML with accessibility alternatives) is built into every slide.

### How to Run It

```bash
# Serve the TOML file directly
toboggan-server "slides_ex/presentations/How to Fuzz Like a Pro/How to Fuzz Like a Pro.toml"

# Then open http://localhost:8080
```

### How It Was Made

1. The original PDF slides were collected from the
   [Trail of Bits publications repository](https://github.com/trailofbits/publications).
2. Each slide's content (title, body, notes) was extracted and converted to Toboggan's TOML format.
3. Rich HTML slides use inline HTML with an `alt` fallback for accessibility.
4. Step-by-step reveals are supported via `<div class="step step-N">` markers.
5. The result is a `slides_ex/presentations/How to Fuzz Like a Pro/How to Fuzz Like a Pro.toml`
   file that can be served by `toboggan-server`.

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
cargo run -p toboggan-server -- "slides_ex/presentations/How to Fuzz Like a Pro/How to Fuzz Like a Pro.toml"

# Or build from markdown sources
cargo run -p toboggan-cli -- "slides_ex/presentations/Building Secure Smart Contracts/" -o my_talk.toml
cargo run -p toboggan-server -- my_talk.toml
```

Then open http://localhost:8080 and present.
