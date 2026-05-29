# Echidna — Fuzzing

- **Property-based fuzzer** for Ethereum smart contracts
- You write **invariants**, Echidna tries to **break them**
- Supports Solidity and Vyper
- Found **300+ real-world vulnerabilities**

<!-- pause -->

### How it works
```
Write invariant ──> Echidna generates random sequences ──> Violation found?
                              │                                    │
                              ▼                                    ▼
                         Shrink input ──────────────────> Report + replay file
```
