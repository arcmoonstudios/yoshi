# Contributing to Yoshi

We welcome and appreciate contributions from the community to the Yoshi enterprise error handling framework! Your involvement helps us build robust, high-performance, and reliable software solutions. This guide outlines the processes and standards for contributing to any crate within the Yoshi monorepo (e.g., `yoshi`, `yoshi-std`, `yoshi-derive`, `yoshi-benches`).

## Code of Conduct

Please review our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing. We are committed to fostering an open and inclusive environment.

## How to Contribute

There are several ways you can contribute to Yoshi:

1. **Bug Reports**: Report issues you encounter to help us improve stability.
2. **Feature Requests**: Suggest new ideas or enhancements to extend functionality.
3. **Documentation Improvements**: Help us refine and expand our documentation, examples, and guides.
4. **Code Contributions**: Submit pull requests for bug fixes, new features, or performance optimizations.

### Reporting Bugs

If you find a bug, please open an issue on our [GitHub Issues](https://github.com/arcmoonstudios/yoshi/issues) page. When reporting a bug, please include:

* A clear and concise description of the bug.
* Steps to reproduce the behavior.
* Expected behavior vs. actual behavior.
* Relevant error messages or stack traces.
* Your Rust version (`rustc --version`) and operating system.
* Any additional context or code snippets that might be helpful.

### Requesting Features

To request a new feature or enhancement, open an issue on [GitHub Issues](https://github.com/arcmoonstudios/yoshi/issues). Describe the feature, its use case, and why you believe it would be valuable to the Yoshi framework.

### Submitting Pull Requests

Follow these steps to contribute code via a Pull Request:

1. **Fork the Repository**: Fork the `arcmoonstudios/yoshi` repository on GitHub.
2. **Clone Your Fork**:

```bash
git clone https://github.com/YOUR_USERNAME/yoshi.git
cd yoshi
```

3.**Create a New Branch**:

```bash
git checkout -b feature/your-feature-name-or-bugfix/issue-number
```

(e.g., `feature/add-error-context/123` or `bugfix/fix-panic-on-empty-input/45`)

4.**Make Your Changes**: Implement your bug fix or feature. Ensure your changes adhere to our [Code Quality Standards](#code-quality-standards).

5.**Test Your Changes**: Run tests and benchmarks to ensure correctness and performance. See [Testing and Benchmarking](#testing-and-benchmarking).

6.**Commit Your Changes**: Write clear and concise commit messages. See [Commit Guidelines](#commit-guidelines).

```bash
git commit -m "feat: Add new error kind for network timeouts (#123)"
```

7.**Push to Your Fork**:

```bash
git push origin feature/your-feature-name-or-bugfix/issue-number
```

8.**Open a Pull Request**: Go to your forked repository on GitHub and open a pull request to the `main` branch of `arcmoonstudios/yoshi`. Fill out the PR template provided.

## Development Setup

To set up your local development environment:

1.**Install Rust**: If you don't have Rust installed, follow the instructions on [https://rustup.rs/](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2.**Add Rust Components**: Install necessary components for linting and formatting.

```bash
rustup component add clippy rustfmt
```

3.**Install Cargo Tools (Recommended)**:

```bash
cargo install cargo-audit # For dependency vulnerability checks
cargo install cargo-outdated # For checking outdated dependencies
```

4.**Run Development Checks**:

```bash
# Run all checks across all crates in the monorepo
cargo check --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
cargo audit
```

## Code Quality Standards

All contributions are expected to meet the following quality standards, aligned with ArcMoon Studios Enterprise principles:

1. **Readability**: Code must be clean, well-structured, and easy to understand.
2. **Documentation**: All public APIs (functions, structs, enums, traits) must have comprehensive Rustdoc comments with clear explanations and examples where appropriate.
3. **Testing**: New features and bug fixes must be accompanied by appropriate unit and/or integration tests. Minimum 90% line coverage is generally expected for core logic.
4. **Performance**: Contributions should not introduce performance regressions. For performance-critical areas, benchmarks should be updated or added to validate performance. See [Testing and Benchmarking](#testing-and-benchmarking).
5. **Safety**: We strive for 100% safe Rust. Any `unsafe` code must be explicitly justified, thoroughly reviewed, and accompanied by detailed safety comments.
6. **Consistency**: Adhere to existing code style and naming conventions within the project.

## Testing and Benchmarking

Before submitting a Pull Request, ensure all tests pass and consider running benchmarks.

### Running Tests

```bash
# Run unit tests for all crates
cargo test --workspace

# Run integration tests with all features enabled (recommended for CI)
cargo test --workspace --all-features
```

### Running Benchmarks (for performance-sensitive changes)

If your changes might impact performance (e.g., changes to `yoshi-std`'s core types or algorithms), run the benchmarks in the `yoshi-benches` crate.

```bash
# Navigate to the yoshi-benches crate
cd yoshi/yoshi-benches

# Run all benchmarks
cargo bench

# Run specific benchmarks, e.g., error creation and context attachment
cargo bench --bench error_creation
cargo bench --bench error_context

# Compare against a baseline (e.g., 'main' branch) to detect regressions
cargo bench -- --baseline main --threshold 0.05 # Fails if >5% regression
```

*Note: Benchmarking requires a [nightly Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchain-overrides) to enable Criterion's `bench` feature.*

## Commit Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. This helps us maintain a clear commit history and automate changelog generation.

* **Type**: `feat`, `fix`, `docs`, `chore`, `refactor`, `perf`, `test`, `build`, `ci`, `revert`.
* **Scope (Optional)**: Indicate the part of the codebase affected (e.g., `yoshi-std`, `derive`, `benches`, `docs`).
* **Subject**: Concise summary of the change, less than 50 characters, in imperative mood.
* **Body (Optional)**: More detailed explanation if needed.
* **Footer (Optional)**: Reference issues, PRs, or breaking changes.

Examples:

```text
feat(yoshi-std): Add new `AlreadyExists` YoshiKind (#123)

This commit introduces a new YoshiKind variant for resource conflicts.
It ensures better categorization of common API errors.

Closes #123
```

```text
fix(derive): Correct macro expansion for nested enums

Addresses an issue where the `yoshi_derive` macro would fail to
generate correct error implementations for deeply nested enum variants.
```

## Security Policy

For information on reporting security vulnerabilities, please refer to our [Security Policy](SECURITY.md).

## License

By contributing to Yoshi, you agree that your contributions will be licensed under the project's [Business Source License 1.1 (BSL-1.1)](LICENSE).

---

Thank you for your interest in contributing to Yoshi! We look forward to your contributions.

🌙 ArcMoon Studios - Where precision meets innovation in error handling technology 🌙
