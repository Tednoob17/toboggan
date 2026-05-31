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
the Toboggan source code that reproduces it is on the right.

### Original PDF

<object data="../assets/how-to-fuzz-like-a-pro.pdf" type="application/pdf" width="100%" height="600px">
  <p>Your browser does not support embedded PDFs.
  <a href="../assets/how-to-fuzz-like-a-pro.pdf">Download the original PDF</a>.</p>
</object>

### Toboggan Source Code (by excerpt)

The TOML file is 520 lines across 12 slides, producing 51 PDF pages.
The 12-->51 ratio comes from **steps** (progressive reveals) and `Part` separators.
Each excerpt below maps to one or more PDF pages.
The complete file is at `slides_ex/presentations/How to Fuzz Like a Pro/How to Fuzz Like a Pro.toml`.

---

#### 1. Cover --> PDF page 1

```toml
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

---

#### 2. Speaker introductions --> PDF pages 2-3

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Who Are We?"

[slides.body]
type = "Html"
raw = """
<h2>Nat Chin</h2>
<ul>
<li>Security Engineer at Trail of Bits</li>
<li>Focus: smart contract auditing, fuzzing infrastructure</li>
<li>Creator of Echidna's corpus collection features</li>
</ul>
<h2>Josselin Feist</h2>
<ul>
<li>Principal Security Engineer at Trail of Bits</li>
<li>Creator of Slither, Echidna maintainer</li>
<li>10+ years in program analysis and security</li>
</ul>
"""
alt = """
## Nat Chin
- Security Engineer at Trail of Bits
...
## Josselin Feist
- Principal Security Engineer at Trail of Bits
...
"""

[[slides]]
kind = "Part"

[slides.title]
type = "Text"
text = "Why Fuzzing Matters"
```

---

#### 3. The Problem (2 steps) --> PDF pages 4-5

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "The Problem"

[slides.body]
type = "Html"
raw = """
<div class="step step-0">
<h2>Smart contracts are high-value targets</h2>
<ul>
<li>$3.8B lost in DeFi hacks in 2023 alone</li>
<li>Traditional testing misses edge cases</li>
<li>Manual review is slow and expensive</li>
</ul>
</div>
<div class="step step-1">
<h2>Fuzzing finds what humans miss</h2>
<ul>
<li>Automated input generation explores edge cases</li>
<li>Property-based testing validates invariants</li>
<li>Continuous fuzzing catches regressions</li>
</ul>
</div>
"""
```

> Both `<div class="step step-N">` blocks produce two separate PDF pages.

---

#### 4. What is Echidna? (2 steps) --> PDF pages 6-7

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "What is Echidna?"

[slides.body]
type = "Html"
raw = """
<h2>Echidna: The Haskell smart contract fuzzer</h2>
<ul>
<li>Open-source fuzzer for Ethereum smart contracts</li>
<li>Supports Solidity and Vyper</li>
<li>Property-based: you write invariants, Echidna breaks them</li>
<li>Found 300+ real-world vulnerabilities</li>
</ul>
<div class="step step-1">
<h3>Key capabilities</h3>
<ul>
<li>Sequence-level fuzzing (multi-transaction)</li>
<li>Filtered fuzzing (selective function calls)</li>
<li>Assertion testing</li>
<li>Gas-aware test generation</li>
<li>Corpus collection and replay</li>
</ul>
</div>
"""
```

---

#### 5. Section "Writing Invariants" --> PDF page 8 (separator)

```toml
[[slides]]
kind = "Part"

[slides.title]
type = "Text"
text = "Writing Invariants"
```

---

#### 6. What Are Invariants? (2 steps) --> PDF pages 9-10

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "What Are Invariants?"

[slides.body]
type = "Html"
raw = """
<h2>Properties that must always hold true</h2>
<p>An invariant is a condition that should never be violated...</p>
<div class="step step-1">
<h3>Example invariants</h3>
<table>
<thead>
<tr><th>Property</th><th>Invariant</th></tr>
</thead>
<tbody>
<tr><td>Total supply</td><td>sum(balances) == totalSupply</td></tr>
<tr><td>Access control</td><td>only owner can mint</td></tr>
<tr><td>Liquidity</td><td>pool never empties below threshold</td></tr>
<tr><td>Interest rates</td><td>rate stays within [0, MAX_RATE]</td></tr>
</tbody>
</table>
</div>
"""
```

---

#### 7. Writing Echidna Properties (2 steps) --> PDF pages 11-12

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Writing Echidna Properties"

[slides.body]
type = "Html"
raw = """
<h2>Two ways to write properties</h2>
<h3>1. Boolean functions <code>echidna_*</code></h3>
<pre><code class="language-solidity">function echidna_total_supply_never_exceeds_max() ...
    return totalSupply() <= MAX_SUPPLY;
}</code></pre>
<div class="step step-1">
<h3>2. Assertions via <code>assert()</code></h3>
<pre><code class="language-solidity">function deposit(uint256 amount) public {
    ...
    assert(balanceOf(msg.sender) == oldBalance + amount);
    assert(totalSupply() == oldTotal + amount);
}</code></pre>
</div>
"""
```

---

#### 8. Advanced: Filtered Fuzzing (2 steps) --> PDF pages 13-14

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Advanced: Filtered Fuzzing"

[slides.body]
type = "Html"
raw = """
<h2>Control which functions Echidna calls</h2>
<pre><code class="language-yaml"># echidna.yaml
testMode: assertion
testLimit: 100000
seqLen: 100
shrinkLimit: 5000
coverage: true
filterBlacklist: true
filterFunctions:
  - "deposit(uint256)"
  - "withdraw(uint256)"
  - "borrow(uint256)"
  - "liquidate(address)"</code></pre>
<div class="step step-1">
<h3>Corpus collection</h3>
<pre><code class="language-bash">echidna-test . --config echidna.yaml --corpus-dir corpus/
echidna-test . --config echidna.yaml --corpus-dir corpus/ --seed 42</code></pre>
</div>
"""
```

---

#### 9. Section "Real-World Findings" --> PDF page 15 (separator)

```toml
[[slides]]
kind = "Part"
[slides.title]
type = "Text"
text = "Real-World Findings"
```

---

#### 10. Case Study: Lending Protocol (2 steps) --> PDF pages 16-17

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Case Study: Lending Protocol"

[slides.body]
type = "Html"
raw = """
<div class="step step-0">
<h2>Invariant: Liquidation never reverts</h2>
<pre><code class="language-solidity">function echidna_liquidate_always_succeeds() ... {
    for (uint i = 0; i < positions.length; i++) {
        if (isUnderwater(positions[i])) return false;
    }
    return true;
}</code></pre>
</div>
<div class="step step-1">
<h3>What Echidna found</h3>
<ul>
<li>Rounding in interest calculation allowed a tiny borrow
that could never be liquidated</li>
<li>Cost: $2.1M at risk — found before deployment</li>
<li>Fix: minimum borrow amount + improved rounding</li>
</ul>
</div>
"""
```

---

#### 11. Best Practices (3 steps) --> PDF pages 18-20

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Best Practices"

[slides.body]
type = "Html"
raw = """
<h2>How to Fuzz Effectively</h2>
<div class="step step-0">
<h3>Start simple</h3>
<ul>
<li>Begin with assertion testing...</li>
<li>Add one echidna_* property at a time</li>
<li>Run with <code>--testLimit 50000</code> for quick feedback</li>
</ul>
</div>
<div class="step step-1">
<h3>Scale up</h3>
<ul>
<li>Use filtered fuzzing to target specific functions</li>
<li>Run longer: <code>--testLimit 10000000</code> for deep bugs</li>
<li>Collect and reuse corpus between runs</li>
<li>Integrate into CI/CD for continuous fuzzing</li>
</ul>
</div>
<div class="step step-2">
<h3>Common pitfalls</h3>
<ul>
<li>Writing properties that are always true (tautologies)</li>
<li>Forgetting to use <code>view</code>/<code>pure</code> functions correctly</li>
<li>Not filtering out non-relevant functions</li>
<li>Ignoring gas limits in the fuzzer</li>
</ul>
</div>
"""
```

> 3 steps --> 3 PDF pages for this slide.

---

#### 12. Conclusion & acknowledgments --> PDF pages 21-51

```toml
[[slides]]
kind = "Part"
[slides.title]
type = "Text"
text = "Conclusion"

[[slides]]
kind = "Standard"

[slides.title]
type = "Text"
text = "Key Takeaways"

[slides.body]
type = "Html"
raw = """
<h2>Fuzzing is a superpower</h2>
<ul>
<li>Property-based testing catches what unit tests miss</li>
<li>Echidna makes it easy to write and run invariants</li>
<li>Start with assertions, graduate to custom properties</li>
</ul>
<div class="step step-1">
<h3>Resources</h3>
<ul>
<li>Echidna: <a href="https://github.com/crytic/echidna">github.com/crytic/echidna</a></li>
<li>Building Secure Contracts: <a href="https://github.com/crytic/building-secure-contracts">github.com/crytic/building-secure-contracts</a></li>
<li>Slither: <a href="https://github.com/crytic/slither">github.com/crytic/slither</a></li>
</ul>
</div>
"""

[[slides]]
kind = "Standard"

[slides.style]
classes = ["no_title", "center"]

[slides.title]
type = "Text"
text = "Thank You"

[slides.body]
type = "Html"
raw = """
<h1>Thank You!</h1>
<blockquote><p>Questions? Come find us!</p></blockquote>
<p>Nat Chin &amp; Josselin Feist</p>
<p><em>Trail of Bits — Security Research</em></p>
"""

[slides.notes]
type = "Text"
text = "Remind the audience about Echidna's GitHub repo. Encourage them to try fuzzing their own contracts. Mention the Trail of Bits audit services."
```

> The "Key Takeaways" slide has 2 steps (takeaways + resources) --> 2 pages.  
> The "Thank You" slide has **speaker notes** (`[slides.notes]`) invisible to the audience.

**Slide-to-PDF-page mapping summary:**

| Slides TOML | Steps | Pages PDF |
|---|---|---|
| Cover | 0 step | 1 |
| Who Are We? + Part | 0 step | 2-3 |
| The Problem | 2 steps | 4-5 |
| What is Echidna? | 2 steps | 6-7 |
| Part "Writing Invariants" | 0 step | 8 |
| What Are Invariants? | 2 steps | 9-10 |
| Writing Echidna Properties | 2 steps | 11-12 |
| Advanced: Filtered Fuzzing | 2 steps | 13-14 |
| Part "Real-World Findings" | 0 step | 15 |
| Case Study | 2 steps | 16-17 |
| Best Practices | 3 steps | 18-20 |
| Conclusion + Takeaways + Thank You | 3 steps | 21-51¹ |

> ¹ Pages 21-51 of the original PDF contain additional slides (implementation details, acknowledgments, Q&A) that were not reproduced in this example.

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

## Exporting to PDF

Toboggan does not currently export slides to PDF — it is a live presentation system
designed for real-time, multi-device playback via a WebSocket server. If you need a
PDF version of your slides, you can use your browser's **Print --> Save as PDF** feature
while viewing the presentation at `http://localhost:8080`, or use a tool like
[`wkhtmltopdf`](https://wkhtmltopdf.org/) to render the HTML output.
