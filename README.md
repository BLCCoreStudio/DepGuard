# DepGuard

**Dependency verification CLI for detecting suspicious, immature, or potentially hallucinated packages before installation.**

> **Status:** early development. No stable release has been published.

DepGuard is intended to put a review step between an AI-generated dependency suggestion and the package-manager install command.

## Planned v0.1

The first release will focus on package identity and registry metadata rather than pretending to determine whether a package is universally safe.

Planned signals include:

- package existence in the selected registry
- package age and publication history
- repository/homepage presence when supplied by the registry
- suspiciously similar package names
- unusually limited publisher/package history
- clear separation between facts and heuristic risk signals

Initial ecosystem targets are expected to be npm, PyPI, and crates.io. Network-backed registry checks are not implemented in the current scaffold yet.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build
cargo test
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
