# Property Based Testing

- Traditional fuzzers look for **crashes**
  - Smart contracts don't (really) have crashes
- **Property based testing**
  - User defines **invariants**
  - Fuzzer generates random inputs to check invariants
  - "Unit tests on steroids"
