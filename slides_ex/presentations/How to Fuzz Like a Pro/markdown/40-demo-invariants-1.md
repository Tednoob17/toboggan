# Demo

### What invariants?

```solidity
function _valid_buy(uint desired_tokens, uint wei_sent)
    internal view {
    uint required_wei_sent = (desired_tokens / 10) * decimals;
    require(wei_sent >= required_wei_sent);
}
```
