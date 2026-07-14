# Assistant Knowledge (project-owned)

This file documents actions performed and recommended maintenance steps so the project owns the knowledge instead of relying on external assistants.

## Changes performed by the assistant
- Added docs/ (Home, Installation, Usage, Contributing).
- Fixed formatting with `cargo fmt` and applied small code fixes suggested by `cargo clippy`.
- Ensured CI (./github/workflows/ci.yml) passes: check, test, fmt, clippy, matrix builds.

## How to reproduce locally
- Install Rust toolchain: `rustup default stable`
- Install components: `rustup component add rustfmt clippy`
- Run checks: `cargo check && cargo test && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo build --release`

## CI notes
- Workflows: `.github/workflows/ci.yml` (check/test/fmt/clippy/build) and `release.yml`.
- Trigger CI: push commits or `git commit --allow-empty -m "ci: trigger" && git push`.

## Wiki
- If a GitHub wiki is desired, push content to `ans-inayat/zyraxbuster.wiki.git` (requires repo access). Alternatively, maintain docs/ in repo (already done).

## Maintenance tips
- Keep rust-toolchain and action dtolnay/rust-toolchain in sync with supported Rust versions.
- Run `cargo fmt` before committing to avoid CI failures.

If you want, convert these docs into a wiki later or expand the docs with FAQs and troubleshooting steps.
## FAQ

- Q: Why did CI fail earlier?
  A: Formatting (cargo fmt) and clippy warnings caused failures. Run `cargo fmt` and `cargo clippy` locally before pushing.

- Q: How to trigger CI manually?
  A: Push a commit or create an empty commit: `git commit --allow-empty -m "ci: trigger" && git push`.

## Troubleshooting

- If `cargo fmt` or `cargo clippy` are missing: `rustup component add rustfmt clippy`.
- If CI reports new clippy lints, follow the suggestions from the clippy output (they are safe and idiomatic).

## Contact / Maintenance

Open issues or PRs for further changes. Keep the `dtolnay/rust-toolchain` action and rust toolchain version aligned.
