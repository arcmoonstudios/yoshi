# yoshi (facade crate)

Main entry point for the Yoshi error framework.

## Overview

This crate re-exports the functionality from the underlying implementation crates:

- `yoshi-std` - Core error types and functionality
- `yoshi-derive` - Procedural macros for generating error types

## Features

- `std` - Standard library support (default)
- `derive` - Re-export derive macros
- `serde` - Serialization support
- `tracing` - Integration with tracing crate
- `full` - All features enabled

## License

This crate is licensed under the [Business Source License 1.1](../LICENSE).
