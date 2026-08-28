# DepGuard

**Dependency review CLI for catching risky manifest patterns before installation.**

> **Status:** development preview. No stable release has been published.

DepGuard puts a review step between a dependency suggestion and the package-manager install command. The current preview performs deterministic **local manifest checks**; it does not claim that a package is safe, malicious, real, or hallucinated.

## Current preview

Supported manifests:

- `Cargo.toml`
- `package.json`
- `requirements.txt`

Current review signals include:

- wildcard or floating versions such as `*` and `latest`
- unconstrained Python requirements
- Git, path, URL, VCS, and other direct dependency sources

Example:

```bash
depguard scan Cargo.toml
```

A clean scan exits `0`. Review signals exit `3`. Usage or read errors exit `2`.

## Scope

Registry-backed package-existence, age, publisher-history, repository metadata, and look-alike-name checks are planned. Until those checks exist, DepGuard will not label a dependency as a hallucinated package or make a universal safety judgment.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build --locked
cargo test --locked
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
