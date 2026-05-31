# Echidna — Workflow

Write invariant as Solidity code:

```solidity
function echidna_balance_of_total_supply()
    public returns (bool) {
    return balanceOf(msg.sender) <= _totalSupply;
}
```

> "User balance never exceeds total supply"
