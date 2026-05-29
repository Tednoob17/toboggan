# Slither's Python API

Write custom detectors for project-specific patterns:

```python
from slither import Slither
from slither.core.declarations import FunctionContract

def find_uniswap_interactions(slither: Slither):
    for contract in slither.contracts:
        for function in contract.functions:
            for call in function.internal_calls:
                if "swap" in call.name:
                    print(f"{function.name} calls {call.name}")
                    check_slippage_protection(function)
```

<!-- pause -->

### Use cases
- Enforce project-specific coding standards
- Detect dangerous third-party interactions
- Validate upgradeability patterns
