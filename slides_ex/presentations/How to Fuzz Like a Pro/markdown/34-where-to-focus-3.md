# Where to Focus?

### Think about composition

- Can `transfer` and `transferFrom` be equivalent?
  - `transfer(to, value) ?= transferFrom(msg.sender, to, value)`
- Is transfer additive-like?
  - `transfer(to, v0); transfer(to, v1) ?= transfer(to, v0 + v1)`
