# Proof

Run these checks in order from the repository root. Every command must exit zero.

| Check | Command | Machine |
| --- | --- | --- |
| build | `cargo build --release --locked` | |
| test | `cargo test --locked` | |
| clippy | `cargo clippy --all-targets --locked -- -D warnings` | |
| fmt | `cargo fmt --check` | |
| test-windows | `cargo test --locked` | windows |
| test-macos | `cargo test --locked` | ci |

## Requirements

- The stable Rust toolchain for edition 2024 (Rust 1.85 or newer) with Cargo on every machine that runs a check; the `clippy` and `rustfmt` components where `clippy` and `fmt` run.
- `test-windows` runs on a Windows Hand with the same toolchain installed.
- `test-macos` runs as the `test-macos` job in `.github/workflows/proof.yml` on a GitHub Actions macOS runner, using the runner's preinstalled Rust toolchain.
- No package-manager install step is needed; Cargo fetches the dependencies recorded in `Cargo.lock`.

## Coverage

- `build`: `Cargo.lock`
- `build`: `Cargo.toml`
- `build`: `src/**`
- `clippy`: `Cargo.lock`
- `clippy`: `Cargo.toml`
- `clippy`: `src/**`
- `clippy`: `tests/**`
- `fmt`: `Cargo.toml`
- `fmt`: `src/**`
- `fmt`: `tests/**`
- `test-macos`: `Cargo.lock`
- `test-macos`: `Cargo.toml`
- `test-macos`: `src/**`
- `test-macos`: `tests/**`
- `test-windows`: `Cargo.lock`
- `test-windows`: `Cargo.toml`
- `test-windows`: `src/**`
- `test-windows`: `tests/**`
- `test`: `Cargo.lock`
- `test`: `Cargo.toml`
- `test`: `src/**`
- `test`: `tests/**`
- Review-only: `.github/workflows/proof.yml`
- Review-only: `.gitignore`
- Review-only: `AGENTS.md`
- Review-only: `CLAUDE.md`
- Review-only: `LICENSE`
- Review-only: `README.md`
- Review-only: `docs/agents/**`
