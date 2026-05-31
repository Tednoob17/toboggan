# Showcase: Rebuilding Trail of Bits Slides

This page documents how the PDF presentation **"How to Fuzz Like a Pro"**
was rebuilt as a Toboggan TOML slide deck. Each of the 51 PDF pages is
rendered as a high-resolution JPEG image and displayed as one slide.

## Original PDF

<object data="../assets/how-to-fuzz-like-a-pro.pdf" type="application/pdf" width="100%" height="600px">
  <p>Your browser does not support embedded PDFs.
  <a href="../assets/how-to-fuzz-like-a-pro.pdf">Download the original PDF</a>.</p>
</object>

## Global configuration

At the top of the TOML file, the presentation title, date, and a custom
`<head>` injection hide the Toboggan footer bar:

```toml
title = "How to Fuzz Like a Pro"
date = "2025-05-31"
head = """
<style>
  .toboggan-footer { display: none; }
</style>
"""
```

Each PDF page is rendered at 3x zoom (2160×1215 px) using PyMuPDF,
saved as JPEG quality 95. Links and Twitter handles found in the PDF
text are overlaid as clickable `<a>` elements with percentage-based
positioning. The complete TOML file (880 lines) is at
`slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml`.
Images are in `slides_ex/presentations/How to Fuzz Like a Pro/public/`.

## Slide 1 — PDF page 1

**Content:**

```
1
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
```

**TOML code:**

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

## Slide 2 — PDF page 2

**Content:**

```
2
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Building secure contracts:
How to fuzz like a pro
```

**TOML code:**

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

## Slide 3 — PDF page 3

**Content:**

```
3
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Who am I?
•
Nat Chin (@0xicingdeath)
Trail of Bits: trailofbits.com
◦
We help developers to build safer software
```

**TOML code:**

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

> This slide has a **clickable overlay** — the URL is extracted from the PDF
> text and positioned as a transparent `<a>` element on top of the image.

## Slide 4 — PDF page 4

**Content:**

```
4
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Agenda
•
How to ﬁnd bugs?
What is property based testing?
How to deﬁne good invariants?
Comparison with similar tools
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_04.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 5 — PDF page 5

**Content:**

```
5
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
How to Find Bugs?
/// @notice Allow users to buy token. 1 ether = 10 tokens
/// @param tokens The numbers of token to buy
/// @dev Users can send more ether than token to be bought, to give gifts to the
team.
function buy...
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_05.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 6 — PDF page 6

**Content:**

```
6
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
How to Find Bugs?
•
Main techniques
◦
Unit tests
Manual analysis
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_06.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 7 — PDF page 7

**Content:**

```
7
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Full automated - Example
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_07.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 8 — PDF page 8

**Content:**

```
8
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
How to Find Bugs?
•
Semi automated analysis
◦
Beneﬁts
▫
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_08.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 9 — PDF page 9

**Content:**

```
9
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
What is property based testing?
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_09.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 10 — PDF page 10

**Content:**

```
10
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Fuzzing
•
Stress the program with
random inputs*
◦
Most basic fuzzer: randomly
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_10.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 11 — PDF page 11

**Content:**

```
11
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Property based testing
•
Traditional fuzzer usually for crashes
◦
Smart contracts don’t (really) have crashes
User deﬁnes invariants
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_11.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 12 — PDF page 12

**Content:**

```
12
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Invariant
•
Something that must
always be true
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_12.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 13 — PDF page 13

**Content:**

```
13
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_13.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 14 — PDF page 14

**Content:**

```
14
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna
•
Smart contract fuzzer
Open source:
github.com/crytic/echidna
Heavily used in audits & mature
```

**TOML code:**

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

> This slide has a **clickable overlay** — the URL is extracted from the PDF
> text and positioned as a transparent `<a>` element on top of the image.

## Slide 15 — PDF page 15

**Content:**

```
15
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Invariant - Token’s total supply
User balance never exceeds total supply
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_15.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 16 — PDF page 16

**Content:**

```
16
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna - Workﬂow
•
Write invariant as Solidity code
“User balance never exceeds total supply”
function echidna_balance_of_total_supply() public
returns(bool){
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_16.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 17 — PDF page 17

**Content:**

```
17
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna - Workﬂow
contract Token {
uint256 totalSupply;
mapping (address => uint256) balances;
function transfer(address to, uint256
amount) {
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_17.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 18 — PDF page 18

**Content:**

```
18
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna - Demo
pragma solidity 0.7.0;
contract Token{
mapping(address => uint) public balances;
function transfer(address to, uint value) public{
balances[msg.sender] -= value;
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_18.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 19 — PDF page 19

**Content:**

```
19
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna - Demo
pragma solidity 0.7.0;
contract TestToken is Token {
address echidna_caller = msg.sender;
constructor() public {
balances[echidna_caller] = 10000;
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_19.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 20 — PDF page 20

**Content:**

```
20
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna - Demo
https://github.com/crytic/building-secure-contracts/blob/master/program-analysis/echidna/Exercise-1.md
```

**TOML code:**

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
  <a href="https://github.com/crytic/building-secure-contracts/blob/master/program-analysis/echidna/Exercise-1.md" target="_blank" rel="noopener"
     style="position:absolute;left:9%;top:78%;width:77.2%;height:3.1%;cursor:pointer;"></a>
</div>
"""

[slides.notes]
type = "Empty"
```

> This slide has a **clickable overlay** — the URL is extracted from the PDF
> text and positioned as a transparent `<a>` element on top of the image.

## Slide 21 — PDF page 21

**Content:**

```
21
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna - Workﬂow
pragma solidity 0.7.0;
contract Token {   //address(0x0)       1
function transfer(address to, uint value)
public{
balances[msg.sender] -= value;
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_21.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 22 — PDF page 22

**Content:**

```
22
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
How to deﬁne good invariants
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_22.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 23 — PDF page 23

**Content:**

```
23
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Deﬁning good invariants
•
Start small, and iterate
Steps
1.
Deﬁne invariants in English
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_23.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 24 — PDF page 24

**Content:**

```
24
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Identify invariants
•
Sit down and think about what the contract is supposed to do
Write the invariant in plain English
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_24.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 25 — PDF page 25

**Content:**

```
25
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Identify invariants: Maths
•
Math library
◦
Commutative property
▫
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_25.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 26 — PDF page 26

**Content:**

```
26
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Identify invariants: tokens
•
ERC20.total_supply
◦
No user should have a balance > total_supply
ERC20.transfer:
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_26.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 27 — PDF page 27

**Content:**

```
27
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Identify invariants: tokens
•
ERC20.total_supply
◦
No user should have a balance > total_supply
ERC20.transfer:
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_27.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 28 — PDF page 28

**Content:**

```
28
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Write invariants in Solidity
•
Identify the target of the invariant
◦
Function-level invariant
▫
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_28.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 29 — PDF page 29

**Content:**

```
29
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Function-level invariant
•
Inherit the target
Create function and call the targeted function
Use assert to check the property
contract TestMath is Math{
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_29.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 30 — PDF page 30

**Content:**

```
30
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
•
Require initialization
◦
Simple initialization: constructor or inheritance
Complex initialization: leverage your unit test/deployment scripts
etheno
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_30.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 31 — PDF page 31

**Content:**

```
31
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
System level invariant
contract TestToken is Token {
address echidna_caller =
0x00a329C0648769a73afAC7F9381e08fb43DBEA70;
constructor() public{
balances[echidna_caller] = 10000;
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_31.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 32 — PDF page 32

**Content:**

```
32
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Where to focus?
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_32.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 33 — PDF page 33

**Content:**

```
33
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Where to focus?
•
In practice: you don’t know where the bugs are
Code coverage vs behavior coverage
◦
Cover as many functions as possible or;
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_33.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 34 — PDF page 34

**Content:**

```
34
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
•
Try different strategies
◦
Behavior coverage ﬁrst
▫
Focus on 1 or 2 components
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_34.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 35 — PDF page 35

**Content:**

```
35
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
•
Start simple, then think about composition, related behaviors,
etc…
◦
Can transfer and transferFrom be equivalent?
▫
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_35.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 36 — PDF page 36

**Content:**

```
36
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
•
Start simple, then think about composition, related behaviors,
etc…
◦
Can transfer and transferFrom be equivalent?
▫
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_36.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 37 — PDF page 37

**Content:**

```
37
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Where to focus?
•
Building your own experience will make you more efficient over
time
Learn on how to think about invariants is a key component to
write better code
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_37.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 38 — PDF page 38

**Content:**

```
38
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_38.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 39 — PDF page 39

**Content:**

```
39
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
/// @notice Allow users to buy token. 1 ether = 10 tokens
/// @param tokens The numbers of token to buy
/// @dev Users can send more ether than token to be bought, to give gifts to the
team.
function buy(uint tokens...
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_39.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 40 — PDF page 40

**Content:**

```
40
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
•
buy is stateful
_valid_buy is stateless
◦
Start with it
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_40.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 41 — PDF page 41

**Content:**

```
41
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
•
What invariants?
function _valid_buy(uint desired_tokens, uint wei_sent) internal view{
uint required_wei_sent = (desired_tokens / 10) * decimals;
require(wei_sent >= required_wei_sent);
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_41.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 42 — PDF page 42

**Content:**

```
42
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
•
What invariants?
◦
If wei_sent is zero, desired_tokens must be zero
function _valid_buy(uint desired_tokens, uint wei_sent) internal view{
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_42.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 43 — PDF page 43

**Content:**

```
43
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
function assert_no_free_token(uint desired_amount) public {
require(desired_amount>0);
_valid_buy(desired_amount, 0);
assert(false); // this should never be reached
}
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_43.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 44 — PDF page 44

**Content:**

```
44
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
<Demo>
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_44.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 45 — PDF page 45

**Content:**

```
45
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Demo
// 1
function assert_no_free_token(uint desired_amount) public {
require(desired_amount>0);
_valid_buy(desired_amount, 0);
assert(false); // this should never be reached
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_45.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 46 — PDF page 46

**Content:**

```
46
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Comparison with similar tools
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_46.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 47 — PDF page 47

**Content:**

```
47
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Other fuzzers
•
Inbuilt in dapp, brownie, foundry, ..
Might be easier for simple test, however
◦
Less powerful (e.g. not stateful in foundry)
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_47.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 48 — PDF page 48

**Content:**

```
48
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Formal methods based approach
•
Manticore, KEVM, Certora, ..
Provide proofs, however
◦
More diﬃcult to use
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_48.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 49 — PDF page 49

**Content:**

```
49
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Echidna’s advantages
•
Echidna has unique additional advanced features
◦
Can target high gas consumption functions
Diﬀerential fuzzing
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_49.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 50 — PDF page 50

**Content:**

```
50
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Conclusion
```

**TOML code:**

```toml
[[slides]]
kind = "Standard"

[slides.title]
type = "Empty"

[slides.body]
type = "Html"
raw = """
<div style="width:100%;height:100%;position:relative;display:flex;align-items:center;justify-content:center;">
  <img src="public/page_50.jpg" style="max-width:100%;max-height:100%;object-fit:contain;display:block;">
</div>
"""

[slides.notes]
type = "Empty"
```

## Slide 51 — PDF page 51

**Content:**

```
51
DeFi Security Summit     |     Building Secure Contracts: Fuzzing like a pro
Conclusion
•
https://github.com/crytic/echidna
To learn more: github.com/crytic/building-secure-contracts
Start by writing invariants in English, then write Solidity properties
◦
```

**TOML code:**

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

> This slide has a **clickable overlay** — the URL is extracted from the PDF
> text and positioned as a transparent `<a>` element on top of the image.

## How to serve

```bash
toboggan-server \
  "slides_ex/presentations/How to Fuzz Like a Pro/how-to-fuzz-like-a-pro.toml" \
  --public-dir "slides_ex/presentations/How to Fuzz Like a Pro/public" \
  --port 8081 --host 0.0.0.0
```