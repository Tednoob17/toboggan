# Demo

```solidity
/// @notice Allow users to buy token. 1 ether = 10 tokens
function buy(uint tokens) public payable {
    _valid_buy(tokens, msg.value);
    _mint(msg.sender, tokens);
}

function _valid_buy(uint desired_tokens, uint wei_sent)
    internal view {
    uint required_wei_sent = (desired_tokens / 10) * decimals;
    require(wei_sent >= required_wei_sent);
}
```
