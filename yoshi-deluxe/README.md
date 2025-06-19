# Yoshi-Deluxe Auto-Correction System

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi-deluxe.svg)](https://crates.io/crates/yoshi-deluxe)
[![Docs.rs](https://docs.rs/yoshi-deluxe/badge.svg)](https://docs.rs/yoshi-deluxe)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

An intelligent auto-correction engine for Rust that analyzes compiler errors and generates fixes automatically.

## What is Yoshi-Deluxe?

Yoshi-Deluxe is building the future of Rust development - a system that understands your code, learns from compiler diagnostics, and generates intelligent fixes. It analyzes your code's AST, mines documentation for context, and applies surgical corrections that preserve your intent while fixing the issues. This is where Yoshi's error handling meets cutting-edge code analysis.

## Quick Start

Add Yoshi-Deluxe to your project's dependencies:

```toml
[dependencies]
yoshi-deluxe = "0.1"
```

Use the YoshiACSystem to analyze a project and generate fixes:

```rust
use yoshi_deluxe::{YoshiACSystem, Result, analyze_and_auto_fix};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // The path to the project you want to analyze
    let project_path = Path::new("./my-faulty-project");

    // Initialize the system with default settings
    let system = YoshiACSystem::new();

    // Analyze the project and get correction proposals
    let corrections = system.analyze_and_correct(project_path).await?;

    println!("Found {} potential corrections", corrections.len());
    for correction in &corrections {
        println!("File: {}", correction.file_path.display());
        println!("Issue: {}", correction.diagnostic.message);
        if let Some(proposal) = correction.best_proposal() {
            println!("💡 Suggestion: {}", proposal.corrected_code);
            println!("🎯 Confidence: {:.1}%", proposal.confidence * 100.0);
            println!("🛡️ Safety Level: {}", proposal.safety_level);
        }
    }

    // You can also analyze and apply safe fixes automatically
    let (corrections, applied) = analyze_and_auto_fix(project_path).await?;
    println!("\nAutomatically applied {} safe corrections.", applied.len());

    Ok(())
}
```

### The Auto-Correction Pipeline

```bash
cargo check → JSON diagnostics → CompilerDiagnostic → ASTContext → CorrectionProposal → AppliedCorrection
     ↓              ↓                    ↓              ↓              ↓                ↓
[Compiler] [DiagnosticProcessor] [ASTAnalysisEngine] [DocsEngine] [CodeGenEngine] [FileSystem]
```

## Core Capabilities

- **Deep Error Analysis** - Parse cargo check and clippy JSON output with surgical precision
- **AST Intelligence** - Map compiler errors to exact syntax tree locations and extract context
- **Documentation Mining** - Scrape docs.rs for API patterns and usage examples
- **Context-Aware Generation** - Create fixes that understand your code's intent and style
- **Safe Transformations** - Apply precise changes that preserve formatting and semantics
- **High Performance** - Parallel processing and intelligent caching for real-time feedback
- **Robust Diagnostics** - Built on yoshi-core for comprehensive error reporting throughout the pipeline

## How it works

The auto-correction pipeline operates in six intelligent stages:

1. **Analyze** - Execute cargo check and clippy to capture comprehensive compiler diagnostics
2. **Parse** - Transform JSON diagnostics into structured, queryable objects
3. **Map** - Parse source files and precisely map each error to its AST node with full context
4. **Research** - Mine docs.rs for relevant API documentation, patterns, and usage examples
5. **Generate** - Synthesize fixes using AST context, documentation insights, and proven heuristics
6. **Apply** - Execute safe transformations with automatic backups and rollback capabilities

The system learns from each correction, building a knowledge base of successful patterns.

## Testing

The `yoshi-deluxe` crate has comprehensive auto-correction system testing:

### Test Statistics

- **81 Unit Tests** - Auto-correction engine and system components
- **2 Doc Tests** - Working examples for key functionality
- **4 Real Auto-Correction Tests** - End-to-end auto-correction validation
- **0 Ignored Tests** - Every test validates real auto-correction functionality

### Running Tests

```bash
# Run all yoshi-deluxe tests
cargo test -p yoshi-deluxe

# Run with all features
cargo test -p yoshi-deluxe --all-features

# Run specific test categories
cargo test -p yoshi-deluxe real_autocorrection_system
cargo test -p yoshi-deluxe integration_tests
cargo test -p yoshi-deluxe system::tests
```

### Test Categories

- **System Tests:** Auto-correction system initialization and configuration
- **AST Tests:** Abstract syntax tree parsing and analysis
- **Diagnostics Tests:** Error pattern detection and analysis
- **Code Generation Tests:** Auto-correction proposal generation
- **Metrics Tests:** Performance monitoring and analysis
- **Integration Tests:** End-to-end auto-correction workflows
- **Real Auto-Correction Tests:** Actual code correction validation

### Key Test Features

- **Real Code Analysis:** Tests analyze actual Rust code for error patterns
- **Auto-Correction Validation:** Verifies generated corrections are valid
- **Performance Monitoring:** Tracks auto-correction system performance
- **Safety Validation:** Ensures corrections don't break existing functionality
- **Pattern Detection:** Validates comprehensive error pattern recognition
- **LSP Integration:** Tests IDE integration capabilities

## Documentation & Examples

- [Introduction & Concepts](../docs/overview.md)
- [Macro Guide](../docs/macro.md)
- [Error Context & Metadata](../docs/context.md)
- [Performance Details](../docs/perf.md)
- [API Docs](https://docs.rs/yoshi-deluxe)
- [Examples](https://github.com/arcmoonstudios/yoshi/tree/main/yoshi/examples/)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

---

Made by ArcMoon Studios
