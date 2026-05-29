# Writing Effective Invariants

### Boolean properties

```solidity
// echidna_ prefix = automatic property test
function echidna_total_supply_balanced() public view returns (bool) {
    return totalSupply() == balanceOf(address(this)) + balanceOf(owner);
}
```

### Assertion mode

```solidity
function mint(uint256 amount) public {
    uint256 before = totalSupply();
    _mint(msg.sender, amount);
    assert(totalSupply() == before + amount);  // Echidna checks this
}
```
