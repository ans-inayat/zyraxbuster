# Contributing to ZyraxBuster

Thank you for your interest in contributing to ZyraxBuster! This document provides guidelines and information about contributing to this project.

## Code of Conduct

- Be respectful and constructive
- Focus on the technical issue, not the person
- Help create a welcoming environment for everyone

## How to Contribute

### Reporting Bugs

1. Check [existing issues](https://github.com/ans-inayat/zyraxbuster/issues) to avoid duplicates
2. Open a new issue using the **Bug Report** template
3. Include:
   - ZyraxBuster version (`--version`)
   - Rust version (`rustc --version`)
   - Operating system and architecture
   - Steps to reproduce
   - Expected vs actual behavior
   - Relevant error messages or logs

### Suggesting Features

1. Check [existing issues](https://github.com/ans-inayat/zyraxbuster/issues) for similar suggestions
2. Open a new issue using the **Feature Request** template
3. Describe:
   - The problem you're trying to solve
   - Your proposed solution
   - Alternatives you considered
   - Use cases

### Submitting Changes

1. **Fork** the repository
2. **Create a branch** for your change:
   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/my-bugfix
   ```
3. **Make your changes**
4. **Test** your changes:
   ```bash
   cargo build --release
   cargo test
   cargo clippy
   ```
5. **Commit** with a clear message:
   ```bash
   git commit -m "feat: add proxy support"
   # or
   git commit -m "fix: resolve timeout on slow connections"
   ```
6. **Push** to your fork:
   ```bash
   git push origin feature/my-feature
   ```
7. **Open a Pull Request** using the PR template

## Development Setup

### Prerequisites

- Rust 2021+ ([rustup.rs](https://rustup.rs/))
- Git

### Building

```bash
git clone https://github.com/ans-inayat/zyraxbuster.git
cd zyraxbuster
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
# Clippy for linting
cargo clippy

# Format code
cargo fmt
cargo fmt --check
```

## Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `style` | Code style changes (formatting, no logic change) |
| `refactor` | Code refactoring (no feature or fix) |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks |
| `perf` | Performance improvements |

### Examples

```
feat: add proxy support
fix: resolve timeout on slow connections
docs: update README with new examples
refactor: simplify wordlist loading
perf: reduce memory allocation in candidate generation
```

## Code Style

- Follow Rust conventions and idioms
- Use `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Keep functions focused and small
- Use meaningful variable and function names
- Add comments only when the intent is non-obvious

## Project Structure

```
src/
├── main.rs      — entry point, banner, wiring
├── cli.rs       — clap argument definitions
├── wordlist.rs  — wordlist loading, auto-detection
└── scanner.rs   — async HTTP scanning engine
```

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Testing

Test against a local server:

```bash
# Start a test server (e.g., Python)
python3 -m http.server 8080

# Run zyraxbuster against it
./target/release/zyraxbuster -u http://localhost:8080 -w test-wordlist.txt
```

### Manual Testing

Always test:
- Basic scanning with various wordlists
- Extension appending
- Status code filtering
- JSON output
- Error handling (invalid URLs, missing wordlists)
- Cross-platform builds (Linux, macOS, Windows)

## Pull Request Checklist

Before submitting your PR, ensure:

- [ ] Code compiles without errors (`cargo build --release`)
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] README is updated (if applicable)
- [ ] CHANGELOG is updated (for notable changes)
- [ ] Commit messages follow conventional format
- [ ] PR description clearly explains the changes

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new version
3. Create a git tag:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically build and publish binaries

## Questions?

If you have questions about contributing, feel free to open an issue with the **Question** label.

Thank you for contributing to ZyraxBuster!
