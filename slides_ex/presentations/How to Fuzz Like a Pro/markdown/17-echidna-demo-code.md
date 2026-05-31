# Echidna — Demo

```solidity
pragma solidity 0.7.0;

contract Token {
    mapping(address => uint) public balances;

    function transfer(address to, uint value) public {
        balances[msg.sender] -= value;
        balances[to] += value;
    }
}
```
