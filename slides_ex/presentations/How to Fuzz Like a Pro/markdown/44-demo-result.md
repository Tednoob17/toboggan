# Demo

// 1 — `desired_amount > 0`

```solidity
function assert_no_free_token(uint desired_amount) public {
    require(desired_amount > 0);
    _valid_buy(desired_amount, 0);
    assert(false);
}

//        1                  0
function _valid_buy(uint desired_tokens, uint wei_sent) internal view {
    uint required_wei_sent = (desired_tokens / 10) * decimals;
    require(wei_sent >= required_wei_sent);
}
```
