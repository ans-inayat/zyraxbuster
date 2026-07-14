# Changelog

All notable changes to ZyraxBuster will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-14

### Initial Release

#### Added

- **Core scanning engine**
  - Async concurrent directory/content brute-forcing
  - Configurable worker pool (`-t`, default: 40)
  - HTTP/HTTPS support with TLS verification bypass (`-k`)
  - Redirect following toggle (`-r`)
  - Retry on network errors (`--retries`)
  - Request timeout configuration (`--timeout`)

- **Wordlist management**
  - Wordlist loading from file
  - Auto-detection from common paths (`/usr/share/wordlists/`, seclists)
  - Short name resolution (e.g., `-w common.txt`)
  - `--list-wordlists` to browse available wordlists
  - Comment and blank line filtering

- **Filtering**
  - Status code whitelist (`-s`) and blacklist (`-b`)
  - Content-length minimum filter (`--min-length`)
  - Content-length exclusion filter (`--exclude-length`)
  - Extension appending (`-x php,html,bak`)

- **Output**
  - Colorized live results
  - Progress bar with ETA
  - File output (`-o`)
  - JSON output format (`--json`)

- **Stealth features**
  - Random User-Agent rotation (`--random-agent`)
  - Rate limiting with delay (`--delay`)

- **CLI**
  - Full help output (`--help`)
  - Version info (`--version`)
  - Custom headers (`-H`, repeatable)
  - Trailing slash option (`--add-slash`)

#### Changed

- Updated banner to block-style ASCII art
- Wordlist (`-w`) is now optional (auto-detects if omitted)

#### Fixed

- None (initial release)

---

## [Unreleased]

### Planned

- VHost / subdomain fuzzing mode
- Recursive directory scanning
- FUZZ-keyword templating (like ffuf)
- Response body diffing / similarity filtering
- Proxy support (`--proxy`)
- Multiple wordlist support
- Rate limit auto-detection
- Table output mode
