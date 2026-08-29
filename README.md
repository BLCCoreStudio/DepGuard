# DepGuard

**Review dependency manifests for risky or weakly constrained entries before you install anything.**

> **Status:** development preview. No stable release has been published.

DepGuard is a small local CLI that scans `Cargo.toml`, `package.json`, and `requirements.txt` before the package-manager install step. It highlights entries that deserve human review — such as wildcard versions, unconstrained requirements, and direct Git/path/URL dependencies — without claiming that a package is safe or malicious.

```bash
depguard scan Cargo.toml
depguard scan package.json
depguard scan requirements.txt
```

## Current preview

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

RepoDoctor already has broader repository-level dependency diagnostics, while DepGuard is intentionally narrower: it explores **pre-install manifest review** as a small standalone CLI.

The two products will not be allowed to grow duplicate rule sets blindly. An integration evaluation is tracked in the private RepoDoctor development repository to map existing rule coverage, identify genuinely new DepGuard signals, and avoid duplicate findings/scoring before any shared behavior is moved into RepoDoctor.

Until that evaluation is complete:

- DepGuard remains a focused development preview
- RepoDoctor remains the primary full-repository diagnostics product
- overlapping behavior should not be advertised as separate innovation

## Scope

Registry-backed package-existence, age, publisher-history, repository metadata, and look-alike-name checks are not implemented in the current preview. Until such checks exist, DepGuard will not label a dependency as a hallucinated package or make a universal safety judgment.

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