# Contributing

Focused registry integrations, package-identity checks, risk-model discussions, tests, and documentation improvements are welcome.

Before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Risk signals must be explainable and should distinguish verified registry facts from heuristics. Security reports belong in the private channel described by `SECURITY.md`.
