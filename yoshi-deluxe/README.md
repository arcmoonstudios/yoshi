# Yoshi-Deluxe Auto-Correction System

![Yoshi-Deluxe Logo](assets/YoshiDeluxeLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi-deluxe.svg)](https://crates.io/crates/yoshi-deluxe)
[![Docs.rs](https://docs.rs/yoshi-deluxe/badge.svg)](https://docs.rs/yoshi-deluxe)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

An intelligent, AST-driven auto-correction framework for Rust that finds, analyzes, and fixes compiler errors and clippy lints.

## What is Yoshi-Deluxe?

Yoshi-Deluxe is part of the Yoshi error-handling framework. It integrates with the Rust compiler (`cargo check`, `clippy`) to parse diagnostics, maps errors to precise locations in the Abstract Syntax Tree (AST), and uses intelligent heuristics to generate safe, context-aware fixes. It's built on the robust `yoshi-std` error handling framework for comprehensive diagnostics at every stage.

## Quick Start

Add Yoshi-Deluxe to your project's dependencies:

```toml
[dependencies]
yoshi-deluxe = "0.1.0"
```

Use the AutoCorrectionSystem to analyze a project and generate fixes:

```rust
use yoshi_deluxe::{AutoCorrectionSystem, Result, analyze_and_auto_fix};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // The path to the project you want to analyze
    let project_path = Path::new("./my-faulty-project");

    // Initialize the system with default settings
    let system = AutoCorrectionSystem::new();
    
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

## Key Features

- **Robust Error Analysis**: Parses cargo check and clippy JSON output with high accuracy.
- **Precise AST Mapping**: Maps compiler error byte-offsets to specific AST nodes for surgical modifications.
- **Intelligent Documentation Mining**: Scrapes docs.rs for API information, method signatures, and examples to inform corrections.
- **Context-Aware Code Generation**: Generates fixes using the surrounding code context, including local variables, imports, and trait implementations.
- **Safe AST Modifications**: Performs precise, byte-offset-based code replacements that preserve existing formatting.
- **Performance Optimization**: Features parallel processing and intelligent caching for fast analysis of large codebases.
- **Rich Diagnostics**: Built on yoshi-std for structured, traceable errors throughout the entire correction pipeline.

## How It Works

Yoshi-Deluxe follows a multi-stage pipeline to deliver high-quality code corrections:

1. **Analyze**: Executes cargo check and clippy on a target project to capture compiler diagnostics as JSON.
2. **Parse**: Deserializes the JSON output into structured CompilerDiagnostic objects.
3. **Map**: For each diagnostic, the ASTAnalysisEngine parses the source file and maps the error's byte-offset to a specific AST node, extracting the surrounding code context.
4. **Research**: If enabled, the DocsScrapingEngine fetches documentation from docs.rs for relevant types, searching for similar methods or traits that could resolve the error.
5. **Generate**: The CodeGenerationEngine uses the AST context, documentation, and a set of built-in heuristics to generate one or more CorrectionProposals.
6. **Apply**: The system can automatically apply proposals that meet a high safety and confidence threshold, creating backups of the original files.

## Documentation & Examples

- [Introduction & Concepts](docs/introduction.md)
- [System Architecture](docs/architecture.md)
- [Correction Strategies](docs/strategies.md)
- [Configuration Guide](docs/configuration.md)
- [API Docs](https://docs.rs/yoshi-deluxe)
- [Examples](examples/)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

---

Made by ArcMoon Studios
