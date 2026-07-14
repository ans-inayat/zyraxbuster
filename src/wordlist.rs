use colored::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Common wordlist paths to search, in priority order
const WORDLIST_SEARCH_PATHS: &[&str] = &[
    "/usr/share/wordlists/seclists/Discovery/Web-Content/common.txt",
    "/usr/share/seclists/Discovery/Web-Content/common.txt",
    "/usr/share/wordlists/seclists/Discovery/Web-Content/raft-large-directories.txt",
    "/usr/share/seclists/Discovery/Web-Content/raft-large-directories.txt",
    "/usr/share/wordlists/seclists/Discovery/Web-Content/raft-large-words.txt",
    "/usr/share/seclists/Discovery/Web-Content/raft-large-words.txt",
    "/usr/share/wordlists/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt",
    "/usr/share/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt",
    "/usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt",
    "/usr/share/wordlists/dirbuster/directory-list-2.3-small.txt",
    "/usr/share/wordlists/common.txt",
    "/usr/share/wordlists/rockyou.txt",
];

/// Directories to scan for wordlists when listing
const WORDLIST_SCAN_DIRS: &[&str] = &[
    "/usr/share/wordlists/seclists/Discovery/Web-Content",
    "/usr/share/seclists/Discovery/Web-Content",
    "/usr/share/wordlists",
    "/usr/share/dirbuster/wordlists",
];

/// Auto-detect a suitable wordlist from common paths
pub fn auto_detect_wordlist() -> Option<String> {
    for path in WORDLIST_SEARCH_PATHS {
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

/// Resolve a wordlist path — handles relative paths, ~ expansion, and known directory names
pub fn resolve_wordlist_path(input: &str) -> String {
    // Expand ~
    let expanded = if let Some(rest) = input.strip_prefix("~/") {
        if let Some(home) = std::env::var("HOME").ok() {
            format!("{}/{}", home, rest)
        } else {
            input.to_string()
        }
    } else {
        input.to_string()
    };

    let path = Path::new(&expanded);

    // If it exists as-is, return it
    if path.exists() {
        return expanded;
    }

    // Try appending to common base directories
    let bases = &[
        "/usr/share/wordlists/seclists/Discovery/Web-Content",
        "/usr/share/seclists/Discovery/Web-Content",
        "/usr/share/wordlists",
        "/usr/share/dirbuster/wordlists",
    ];

    for base in bases {
        let candidate = format!("{}/{}", base, input);
        if Path::new(&candidate).exists() {
            return candidate;
        }
    }

    // Return original (will error later if not found)
    expanded
}

/// List available wordlists from common paths
pub fn list_available_wordlists() {
    println!("{}", "Available wordlists:".bright_cyan().bold());
    println!();

    for dir in WORDLIST_SCAN_DIRS {
        let path = Path::new(dir);
        if !path.exists() {
            continue;
        }

        println!("{}", format!("{}:", dir).bright_yellow());

        if let Ok(entries) = fs::read_dir(path) {
            let mut files: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let p = e.path();
                    p.extension()
                        .map_or(false, |ext| ext == "txt" || ext == "lst")
                        || p.is_dir()
                })
                .map(|e| e.path())
                .collect();

            files.sort();

            for file in &files {
                let name = file
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if file.is_dir() {
                    // List .txt files inside subdirectories
                    println!("  {}/", name.bright_green());
                    if let Ok(sub_entries) = fs::read_dir(file) {
                        let mut sub_files: Vec<PathBuf> = sub_entries
                            .filter_map(|e| e.ok())
                            .filter(|e| {
                                let p = e.path();
                                p.extension()
                                    .map_or(false, |ext| ext == "txt" || ext == "lst")
                            })
                            .map(|e| e.path())
                            .collect();
                        sub_files.sort();
                        for sub in &sub_files {
                            let sub_name = sub
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let size = fs::metadata(sub).map_or(0, |m| m.len());
                            let size_str = format_size(size);
                            println!("    {:<55} {}", sub_name, size_str);
                        }
                    }
                } else {
                    let size = fs::metadata(file).map_or(0, |m| m.len());
                    let size_str = format_size(size);
                    println!("  {:<55} {}", name, size_str);
                }
            }
        }
        println!();
    }

    println!(
        "{} Use -w <path> to select a wordlist, or just omit -w to auto-detect",
        "Tip:".bright_cyan()
    );
    println!(
        "{} Short names work too: -w common.txt or -w raft-large-directories.txt",
        "".dimmed()
    );
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Load a wordlist file into a vec of non-empty, trimmed lines,
/// skipping comments (#) and blank lines.
pub fn load_wordlist(path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let words: Vec<String> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect();
    Ok(words)
}

/// Build the full candidate list: base words plus word+extension combos.
pub fn build_candidates(
    words: &[String],
    extensions: &Option<String>,
    add_slash: bool,
) -> Vec<String> {
    let mut candidates = Vec::new();

    let exts: Vec<String> = match extensions {
        Some(e) => e
            .split(',')
            .map(|s| s.trim().trim_start_matches('.').to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        None => Vec::new(),
    };

    for w in words {
        let mut base = w.clone();
        if add_slash && !base.ends_with('/') {
            base.push('/');
        }
        candidates.push(base.clone());

        for ext in &exts {
            candidates.push(format!("{}.{}", w, ext));
        }
    }

    candidates
}
