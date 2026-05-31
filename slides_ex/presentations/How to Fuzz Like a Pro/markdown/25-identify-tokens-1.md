# Identify Invariants: Tokens

### ERC20.totalSupply

- No user should have a balance > totalSupply

### ERC20.transfer

After calling `transfer`:
- Sender balance should decrease by amount
- Receiver balance should increase by amount
