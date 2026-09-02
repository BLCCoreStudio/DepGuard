# DepGuard

**Focused pre-install dependency-manifest review research.**

> **Status: maintenance-only companion project.** New repository-level dependency diagnostics and integration work should target [RepoDoctor](https://github.com/BLCCoreStudio/RepoDoctor). DepGuard remains public to preserve its focused implementation, history, and existing links.

DepGuard is a small local CLI that scans `Cargo.toml`, `package.json`, and `requirements.txt` before the package-manager install step. It highlights entries that deserve human review — such as wildcard versions, unconstrained requirements, and direct Git/path/URL dependencies — without claiming that a package is safe or malicious.

```bash
depguard scan Cargo.toml
depguard scan package.json
depguard scan requirements.txt
```

## Current implementation

Supported manifests:

- `Cargo.toml`
- `package.json`
- `requirements.txt`

Current review signals include:

- wildcard or floating versions such as `*` and `latest`
- unconstrained Python requirements
- Git, path, URL, VCS, and other direct dependency sources

A clean scan exits `0`. Review signals exit `3`. Usage or read errors exit `2`.

## Relationship to RepoDoctor

[RepoDoctor](https://github.com/BLCCoreStudio/RepoDoctor) is the primary maintained repository-health product and already covers broader dependency diagnostics alongside security, testing, CI/CD, documentation, configuration, and architecture.

DepGuard is intentionally kept narrow and should not grow a second competing dependency-analysis product. Useful rules may be promoted into RepoDoctor when they add non-duplicative value and can be integrated with appropriate tests and scoring semantics.

For new integrated dependency-health work, use **RepoDoctor**.

## Scope and limitations

Registry-backed package-existence, age, publisher-history, repository metadata, and look-alike-name checks are not implemented here. DepGuard does not label dependencies as malicious, hallucinated, or universally safe.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build --locked
cargo test --locked
```

## Security

See [SECURITY.md](SECURITY.md) for reporting guidance and limitations.

## License

MIT © BLC Core Studio
