# ZyraxBuster — Copilot Instructions

## Project Overview

ZyraxBuster is a fast, asynchronous directory/content discovery tool written in Rust. It's a gobuster/ffuf-style brute-forcer built on `tokio` + `reqwest`.

**Repository:** https://github.com/ans-inayat/zyraxbuster
**Author:** ans-inayat
**License:** MIT
**Language:** Rust (edition 2021)

---

## Project Structure

```
zyraxbuster/
├── src/
│   ├── main.rs      — Entry point, banner, wiring, output handling
│   ├── cli.rs       — Clap argument definitions (all CLI options)
│   ├── wordlist.rs  — Wordlist loading, auto-detection, candidate generation, listing
│   └── scanner.rs   — Async HTTP scanning engine with concurrent workers
├── Cargo.toml       — Dependencies and build configuration
├── README.md        — Full documentation with badges, features, usage
├── CHANGELOG.md     — Version history (Keep a Changelog format)
├── CONTRIBUTING.md  — Contribution guidelines, commit format, PR checklist
├── LICENSE          — MIT License
├── SECURITY.md      — Vulnerability reporting policy
├── .gitignore       — Git ignore rules
└── .github/
    ├── copilot-instructions.md — This file
    ├── ISSUE_TEMPLATE/
    │   ├── bug_report.md       — Bug report template
    │   └── feature_request.md  — Feature request template
    ├── PULL_REQUEST_TEMPLATE.md — PR template
    └── workflows/
        ├── ci.yml              — CI pipeline (check, test, fmt, clippy, cross-build)
        └── release.yml         — Auto-release on tag push (all platforms)
```

---

## Tech Stack

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x (full) | Async runtime |
| `reqwest` | 0.11.x (rustls-tls) | HTTP client |
| `clap` | 4.4.x (derive) | CLI argument parsing |
| `indicatif` | 0.17.x | Progress bar |
| `futures` | 0.3.x | Async stream utilities |
| `colored` | 2.x | Terminal colors |
| `rand` | 0.8.x | Random User-Agent selection |
| `serde_json` | 1.x | JSON output |

### Build Configuration

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

---

## Architecture

### Data Flow

```
CLI args (cli.rs)
    ↓
Wordlist loading (wordlist.rs)
    ↓
Candidate generation (wordlist.rs: build_candidates)
    ↓
Concurrent scanning (scanner.rs: run_scan)
    ↓
Filter results (scanner.rs: should_report)
    ↓
Output to console + file (main.rs)
```

### Key Types

```rust
// scanner.rs
pub struct HitResult {
    pub url: String,
    pub status: u16,
    pub length: u64,
    pub redirect: Option<String>,
}

// cli.rs
pub struct Args {
    pub url: Option<String>,
    pub wordlist: Option<String>,
    pub threads: usize,
    pub extensions: Option<String>,
    pub status_codes: Option<String>,
    pub blacklist_codes: String,
    pub timeout: u64,
    pub follow_redirects: bool,
    pub user_agent: String,
    pub random_agent: bool,
    pub add_slash: bool,
    pub output: Option<String>,
    pub json: bool,
    pub min_length: Option<u64>,
    pub exclude_length: Option<u64>,
    pub header: Vec<String>,
    pub insecure: bool,
    pub retries: u32,
    pub delay: u64,
    pub list_wordlists: bool,
}
```

### Key Functions

```rust
// wordlist.rs
pub fn auto_detect_wordlist() -> Option<String>
pub fn resolve_wordlist_path(input: &str) -> String
pub fn list_available_wordlists()
pub fn load_wordlist(path: &str) -> io::Result<Vec<String>>
pub fn build_candidates(words: &[String], extensions: &Option<String>, add_slash: bool) -> Vec<String>

// scanner.rs
pub fn build_client(args: &Args) -> reqwest::Result<Client>
pub async fn run_scan(args: Args, url: String, candidates: Vec<String>) -> Vec<HitResult>
```

---

## CLI Options Reference

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `-u, --url` | `String` | *(required)* | Target base URL |
| `-w, --wordlist` | `Option<String>` | auto | Wordlist path (auto-detects if omitted) |
| `-t, --threads` | `usize` | `40` | Concurrent workers |
| `-x, --extensions` | `Option<String>` | — | Extensions to append (comma-separated) |
| `-s, --status-codes` | `Option<String>` | — | Whitelist status codes |
| `-b, --blacklist-codes` | `String` | `404` | Blacklist status codes |
| `--timeout` | `u64` | `10` | Request timeout (seconds) |
| `-r, --follow-redirects` | `bool` | `false` | Follow redirects |
| `--user-agent` | `String` | `zyraxbuster/0.1` | Custom User-Agent |
| `--random-agent` | `bool` | `false` | Rotate User-Agent per request |
| `--add-slash` | `bool` | `false` | Add trailing slash |
| `-o, --output` | `Option<String>` | — | Output file path |
| `--json` | `bool` | `false` | JSON output format |
| `--min-length` | `Option<u64>` | — | Minimum content-length filter |
| `--exclude-length` | `Option<u64>` | — | Exclude exact content-length |
| `-H, --header` | `Vec<String>` | — | Custom headers (repeatable) |
| `-k, --insecure` | `bool` | `false` | Skip TLS verification |
| `--retries` | `u32` | `1` | Retry count |
| `--delay` | `u64` | `0` | Delay between requests (ms) |
| `--list-wordlists` | `bool` | `false` | List wordlists and exit |

---

## Wordlist Auto-Detection

When `-w` is omitted, ZyraxBuster searches these paths in order:

1. `/usr/share/wordlists/seclists/Discovery/Web-Content/common.txt`
2. `/usr/share/seclists/Discovery/Web-Content/common.txt`
3. `/usr/share/wordlists/seclists/Discovery/Web-Content/raft-large-directories.txt`
4. `/usr/share/seclists/Discovery/Web-Content/raft-large-directories.txt`
5. `/usr/share/wordlists/seclists/Discovery/Web-Content/raft-large-words.txt`
6. `/usr/share/seclists/Discovery/Web-Content/raft-large-words.txt`
7. `/usr/share/wordlists/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt`
8. `/usr/share/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt`
9. `/usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt`
10. `/usr/share/wordlists/dirbuster/directory-list-2.3-small.txt`
11. `/usr/share/wordlists/common.txt`
12. `/usr/share/wordlists/rockyou.txt`

Short names (e.g., `-w common.txt`) resolve against these directories:
- `/usr/share/wordlists/seclists/Discovery/Web-Content/`
- `/usr/share/seclists/Discovery/Web-Content/`
- `/usr/share/wordlists/`
- `/usr/share/dirbuster/wordlists/`

---

## Code Style

### Rust Conventions

- Use `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Keep functions focused and small
- Use meaningful variable names
- No unnecessary comments — code should be self-documenting

### Naming

- `snake_case` for functions, variables, modules
- `PascalCase` for types, enums
- `SCREAMING_SNAKE_CASE` for constants

### Error Handling

- Use `Result` types, not panics (except in CLI entry point)
- Provide helpful error messages with colored output
- Exit with code 1 on errors

---

## Common Patterns

### Async Scanning with Concurrency

```rust
stream::iter(candidates.into_iter())
    .for_each_concurrent(concurrency, |word| {
        // Clone Arc'd state for each task
        let client = client.clone();
        let results = Arc::clone(&results);
        async move {
            // Perform scan
            // Print hit if found
            // Push to results
            pb.inc(1);
        }
    })
    .await;
```

### Progress Bar

```rust
let pb = ProgressBar::new(candidates.len() as u64);
pb.set_style(
    ProgressStyle::with_template(
        "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
    )
    .unwrap()
    .progress_chars("=>-"),
);
```

### Colored Output

```rust
use colored::*;

println!("{}", "Success".green().bold());
println!("{}", "Error".red().bold());
println!("{}", "Warning".yellow());
println!("{}", "Dimmed text".dimmed());
```

---

## Build & Release

### Local Build

```bash
cargo build --release
# Binary at target/release/zyraxbuster
```

### Cross-Compilation

```bash
# Linux ARM64
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

### Creating a Release

```bash
# Tag and push
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# GitHub Actions automatically builds binaries for all platforms
```

---

## CI/CD

### CI Pipeline (`.github/workflows/ci.yml`)

Runs on push to main/master and PRs:
1. `cargo check` — Verify compilation
2. `cargo test` — Run tests
3. `cargo fmt --check` — Check formatting
4. `cargo clippy -- -D warnings` — Lint
5. Cross-platform builds (Linux x86_64/ARM64, macOS x86_64/ARM64, Windows)

### Release Pipeline (`.github/workflows/release.yml`)

Triggers on tag push (`v*`):
1. Build binaries for all platforms
2. Generate checksums
3. Create GitHub Release with binaries

---

## Testing

### Unit Tests

```bash
cargo test
```

### Manual Testing

```bash
# Start test server
python3 -m http.server 8080

# Create test wordlist
echo -e "admin\nlogin\ntest\nsecret" > test.txt

# Run scan
./target/release/zyraxbuster -u http://localhost:8080 -w test.txt
```

### Test Checklist

- [ ] Basic scanning works
- [ ] Extensions work (`-x php,html`)
- [ ] Status code filtering works (`-s 200`, `-b 404,500`)
- [ ] Content-length filtering works
- [ ] JSON output works (`--json -o results.json`)
- [ ] Auto wordlist detection works
- [ ] `--list-wordlists` works
- [ ] Random User-Agent works (`--random-agent`)
- [ ] Rate limiting works (`--delay 100`)
- [ ] Custom headers work (`-H "Key: Value"`)
- [ ] Error handling works (invalid URL, missing wordlist)

---

## Features Implemented

### v0.1.0 (Initial Release)

- [x] Async concurrent scanning (tokio + reqwest)
- [x] Auto wordlist detection from seclists
- [x] Short name wordlist resolution
- [x] `--list-wordlists` to browse available wordlists
- [x] Extension appending (`-x`)
- [x] Status code whitelist/blacklist (`-s`, `-b`)
- [x] Content-length filtering (`--min-length`, `--exclude-length`)
- [x] Random User-Agent rotation (`--random-agent`)
- [x] Rate limiting (`--delay`)
- [x] Custom headers (`-H`)
- [x] Redirect following (`-r`)
- [x] TLS bypass (`-k`)
- [x] Retry on failure (`--retries`)
- [x] JSON output (`--json`)
- [x] Colorized output with progress bar
- [x] Cross-platform support (Linux/macOS/Windows, x86_64 + ARM64)

### Roadmap (Future)

- [ ] VHost / subdomain fuzzing mode
- [ ] Recursive directory scanning
- [ ] FUZZ-keyword templating (like ffuf)
- [ ] Response body diffing / similarity filtering
- [ ] Proxy support (`--proxy`)
- [ ] Multiple wordlist support
- [ ] Rate limit auto-detection
- [ ] Table output mode

---

## Important Notes

1. **Wordlist is optional** — auto-detects if omitted
2. **URL is required** — except when using `--list-wordlists`
3. **Default blacklist is 404** — customize with `-b`
4. **JSON requires `-o`** — `--json` alone doesn't output JSON
5. **`--random-agent` overrides `--user-agent`** — when enabled, custom UA is ignored

---

## File Purposes

| File | Purpose | When to Edit |
|------|---------|--------------|
| `src/cli.rs` | CLI argument definitions | Adding new flags/options |
| `src/wordlist.rs` | Wordlist handling | Adding wordlist features |
| `src/scanner.rs` | Scanning engine | Changing scan behavior |
| `src/main.rs` | Entry point, wiring | Changing output, banner, flow |
| `Cargo.toml` | Dependencies | Adding/updating crates |
| `README.md` | Documentation | Feature changes |
| `CHANGELOG.md` | Version history | New releases |
| `.github/workflows/*.yml` | CI/CD | Build/release changes |

---

*This file helps GitHub Copilot understand the project structure and conventions.*
