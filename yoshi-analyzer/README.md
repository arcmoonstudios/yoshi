# yoshi-analyzer

![Yoshi Logo](assets/YoshiLogo.png)

[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/arcmoonstudios/yoshi/blob/main/LICENSE)

**Unified Elite Yoshi Framework Analyzer** - Comprehensively analyzes and quantifies sophisticated implementations of error correction strategies in the Yoshi framework codebase.

## What It Analyzes

This analyzer specifically examines the Yoshi framework's error correction strategy implementations and provides:

- **🔍 Strategy Implementation Analysis** - Detects `yoshi_af!` macro usage and `YoshiError` derive implementations
- **📊 Automation Safety Classification** - Categorizes error strategies by automation safety levels (Ultra Safe, Safe, Review, Caution, Manual)
- **🎯 Pattern Exhaustiveness Analysis** - Identifies missing error patterns and redundant implementations
- **🔧 Sophistication Metrics** - Measures implementation quality and feature completeness
- **💎 Derive Synergy Analysis** - Evaluates compatibility between strategies and derive macros
- **⚡ Performance Metrics** - Tracks analysis performance and coverage statistics

## Usage

```bash
# Run comprehensive analysis
cargo run -- analyze

# Generate automation safety report
cargo run -- safety --yoshi-af-only

# Analyze pattern exhaustiveness
cargo run -- patterns --witnesses

# Check derive synergy
cargo run -- derive-synergy --recommendations

# Dead code elimination analysis
cargo run -- dead-code --dry-run

# Typo detection
cargo run -- typos --imports
```

## Output Formats

- **Comprehensive** - Detailed analysis with all metrics
- **Table** - Tabular format for easy comparison
- **JSON** - Machine-readable format for tooling
- **Markdown** - Documentation-friendly format
- **Diagnostic** - rustc-style diagnostic output

## Analysis Categories

### Automation Safety Levels

- **💎 Ultra Safe** - yoshi_af! + YoshiError + high sophistication + formal verification
- **🟢 Safe** - yoshi_af! protection with comprehensive implementation
- **🟡 Review** - partial protection or medium sophistication requiring review
- **🟠 Caution** - basic implementation needing careful manual review
- **🔴 Manual** - complex semantics or high risk requiring manual intervention

### Implementation Status

- **Enhanced** - Enhanced with yoshi_af! macro protection and advanced features
- **Fully Implemented** - Comprehensive error handling
- **Partially Implemented** - Some missing functionality
- **Not Implemented** - Requires creation
- **Duplicate** - Duplicate implementation detected
- **Derive Synergistic** - Perfect synergy with derive macro

This analyzer is specifically designed for analyzing the Yoshi framework's internal error correction strategies and is not intended as a general-purpose Rust code analyzer.
