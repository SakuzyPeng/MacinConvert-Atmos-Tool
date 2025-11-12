# Repository Guidelines

## Project Structure & Module Organization
Core logic lives in `src/`: `main.rs` wires the CLI flow, `cli.rs` defines Clap options, `decoder.rs`/`merger.rs` talk to GStreamer, and `channels.rs`, `format.rs`, `tools.rs`, `error.rs` hold channel tables, format probes, tool discovery, and shared errors. Sample Dolby assets for manual checks sit in `audio/`, optional proprietary binaries belong in `dolby-tools/` (see `README.md` for the expected subfolders), helper automation resides in `scripts/`, and git hooks live in `.githooks/`.

## Build, Test, and Development Commands
- `cargo build` or `cargo build --release`: compile debug or optimized binaries into `target/`.
- `cargo run -- --input audio/sample_input.ec3 --channels 5.1`: end-to-end decode with provided sample media.
- `cargo test`: execute Rust unit/integration suites under `src/` and `tests/`.
- `cargo fmt`, `cargo clippy -- -D warnings`, `cargo check --all-targets`: formatting, linting, and static analysis (mirrors the pre-commit hook order).
- `cargo audit`: dependency vulnerability scan; install via `cargo install cargo-audit` if missing.

## Coding Style & Naming Conventions
Stick to the default `rustfmt` profile (4-space indents, trailing commas in multi-line literals) and keep modules single-purpose. Use `PascalCase` for types, `snake_case` for functions/fields, and match CLI flag names to their long options (`--channels`, `--single`). Prefer returning `Result<T, ToolError>` with early `?` propagation and log via `log` macros.

## Testing Guidelines
Add lightweight unit tests alongside the module they cover (`mod tests` blocks) and grow integration tests under `tests/` as decoding flows expand. Treat files in `audio/` as read-only fixtures—copy them to a scratch directory before mutation. When introducing a channel layout or parser, assert both detection and WAV ordering, and run `cargo test` plus a representative `cargo run` before opening a PR.

## Commit & Pull Request Guidelines
Follow short, imperative commit subjects similar to `Rewrite in Rust: MacinConvert-Atmos-Tool`; scope body lines to explain user-visible changes or tooling updates. PRs should describe the motivation, list verification commands, mention any Dolby tool setup steps, and link issues via `Closes #ID`. Ensure the local pre-commit hook (installed with `scripts/setup-hooks.sh`) passes or call out justified skips.

## Tooling & Configuration Tips
Keep Dolby binaries out of git by parking them under `dolby-tools/`, which the resolver checks before falling back to `/Applications/Dolby/Dolby Reference Player.app/Contents`. Export alternate locations via the env vars read in `tools.rs` when needed. Enable verbose traces with `RUST_LOG=debug cargo run -- ...` to inspect each pipeline stage during reviews.
