# Identify Invariants: Tokens

### ERC20.transfer (continued)

- If self-transfer: balance should remain identical
- If insufficient funds: tx should revert / return false
