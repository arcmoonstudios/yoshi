# Publishing the Yoshi Crates

This document outlines the process for publishing the Yoshi crates to crates.io. Follow these steps to ensure a smooth publishing process.

## Prerequisites

Before publishing, ensure that:

1. All tests pass (`cargo test --workspace`)
2. The code builds in release mode (`cargo build --release`)
3. Clippy checks pass (`cargo clippy --all-targets --all-features -- -D warnings`)
4. Documentation is complete (`cargo doc --all-features`)

## Publishing Order

Due to the dependencies between crates, the Yoshi crates must be published in a specific order:

1.First publish **yoshi-std**:

```bash
   cd yoshi-std
   cargo publish
```

2.Next publish **yoshi-derive** (which depends on yoshi-std):

```bash
   cd ../yoshi-derive
   cargo publish
```

3.Finally publish **yoshi** (which depends on both yoshi-std and optionally yoshi-derive):

```bash
   cd ../yoshi
   cargo publish
```

## Package Validation

The repository includes a `cratecheck.py` script that validates the packages before publishing:

```python
python cratecheck.py
```

Note that the script will show a warning about package creation until yoshi-std is published to crates.io. This is expected for the first publication.

## Version Management

Ensure that the version numbers in all `Cargo.toml` files are properly updated according to [Semantic Versioning](https://semver.org/) rules.

## Changelog

Update the CHANGELOG.md file with a summary of the changes before creating a new release.

## Post-Publication

After publishing, create a new GitHub release with the appropriate tag and release notes.
