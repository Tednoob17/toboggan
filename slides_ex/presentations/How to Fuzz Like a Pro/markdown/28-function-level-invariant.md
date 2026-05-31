# Function-level Invariant

```solidity
contract TestMath is Math {
    function test_commutative(uint a, uint b)
        public {
        assert(add(a, b) == add(b, a));
    }
}
```
