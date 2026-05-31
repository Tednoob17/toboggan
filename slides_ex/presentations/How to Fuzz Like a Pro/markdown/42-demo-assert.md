# Demo

```solidity
function assert_no_free_token(uint desired_amount)
    public {
    require(desired_amount > 0);
    _valid_buy(desired_amount, 0);
    assert(false); // should never be reached
}
```
