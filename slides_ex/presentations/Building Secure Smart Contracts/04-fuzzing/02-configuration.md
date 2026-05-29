# Configuring Echidna

```yaml
# echidna.yaml
testMode: assertion
testLimit: 100000
seqLen: 100
shrinkLimit: 5000
coverage: true
filterBlacklist: true
filterFunctions:
  - "mint(uint256)"
  - "burn(uint256)"
  - "transfer(address,uint256)"
```

<!-- pause -->

```bash
# Run the fuzzer
echidna-test . --config echidna.yaml

# Collect corpus for faster future runs
echidna-test . --corpus-dir corpus/
```
