# 📋 YOSHI STRATEGY CHECKLIST

## Overview

This document is a simple checklist of available strategies in the Yoshi framework.
**Actual implementations are in `yoshi-deluxe/src/strategies/`**

**EXECUTION PROTOCOL:**

- ✅ **Use yum binary** - `cargo run --bin yum -- apply-strategies --clippy`
- ✅ **Target specific strategies** - `cargo run --bin yum -- apply-strategy unwrap_used`
- ✅ **Apply to specific files** - `cargo run --bin yum -- apply-strategies --file path/to/file.rs`

## 📋 AVAILABLE STRATEGIES

### 🛡️ SAFETY STRATEGIES

- [ ] **PanicUsed** - Convert `panic!()` to proper error handling
- [ ] **ExpectUsed** - Convert `.expect()` to safer patterns
- [ ] **UnwrapUsed** - Convert `.unwrap()` to `?` operator or `if let`
- [ ] **IndexingSlicing** - Convert unsafe indexing to `.get()`

### 🎨 STYLE STRATEGIES

- [ ] **UninlinedFormatArgs** - Convert `format!("{}", var)` to `format!("{var}")`
- [ ] **RedundantFieldNames** - Use field init shorthand
- [ ] **SingleMatch** - Convert single-arm match to if let

### ⚡ PERFORMANCE STRATEGIES

- [ ] **UnnecessaryToOwned** - Remove unnecessary `.to_owned()` calls
- [ ] **UnnecessaryClone** - Remove unnecessary `.clone()` calls
- [ ] **StableSortPrimitive** - Use `.sort_unstable()` for primitives

### ✅ CORRECTNESS STRATEGIES

- [ ] **ComparisonToEmpty** - Convert `.len() == 0` to `.is_empty()`
- [ ] **CollapsibleIf** - Combine nested if statements

## 📂 STRATEGY LOCATIONS

- **Clippy Strategies**: `yoshi-deluxe/src/strategies/flawless_clippy.rs`
- **Compiler Error Strategies**: `yoshi-deluxe/src/strategies/error_correction.rs`

## 🎯 USAGE

```bash
# Apply all clippy strategies
cargo run --bin yum -- apply-strategies --clippy

# Apply specific strategy
cargo run --bin yum -- apply-strategy panic_used

# Apply to specific file (not yet implemented)
cargo run --bin yum -- apply-strategies --file src/lib.rs
```

## ✅ WORKING COMMANDS

- `cargo run --bin yum -- apply-strategies --clippy` - ✅ Working
- `cargo run --bin yum -- apply-strategy panic_used` - ✅ Working
- `cargo run --bin yum -- apply-strategy unnecessary_to_owned` - ✅ Working
- `cargo run --bin yum -- apply-strategy comparison_to_empty` - ✅ Working
