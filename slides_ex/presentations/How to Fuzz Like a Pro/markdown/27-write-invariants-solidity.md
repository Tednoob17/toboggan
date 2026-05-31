# Write Invariants in Solidity

### Identify the target

- **Function-level invariant**
  - Ex: arithmetic's associativity
  - Usually stateless
  - Can craft specific scenarios

- **System-level invariant**
  - Ex: user's balance < totalSupply
  - Usually stateful
  - All functions must be considered
