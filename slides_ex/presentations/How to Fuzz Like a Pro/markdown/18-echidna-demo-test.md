# Echidna — Demo

```solidity
contract TestToken is Token {
    address echidna_caller = msg.sender;

    constructor() public {
        balances[echidna_caller] = 10000;
    }

    function echidna_test_balance()
        public view returns (bool) {
        return balances[msg.sender] <= 10000;
    }
}
```
