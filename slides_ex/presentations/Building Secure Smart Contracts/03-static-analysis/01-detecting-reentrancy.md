# Detecting Reentrancy with Slither

Slither's data flow analysis tracks **state variable writes after external calls**:

```solidity
function withdraw(uint256 amount) external {
    require(balances[msg.sender] >= amount);
    (bool ok, ) = msg.sender.call{value: amount}("");  // external call
    balances[msg.sender] -= amount;                     // state write after call
}
```

<!-- pause -->

### Slither output
```
Reentrancy in withdraw(uint256):
  External call to user-controlled address at line 42
  State variable written after call at line 43
  Severity: HIGH
  SWC-ID: SWC-107
```
