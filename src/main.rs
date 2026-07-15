mod cli;
mod scanner;
mod wordlist;

use clap::Parser;
use cli::Args;
use colored::*;
use std::fs::File;
use std::io::Write;

const BANNER: &str = r#"
 ________     _______            __   ______  _    _  _____ _______ ______ _____  
 |___  /\ \   / /  __ \     /\    \ \ / /  _ \| |  | |/ ____|__   __|  ____|  __ \ 
    / /  \ \_/ /| |__) |   /  \    \ V /| |_) | |  | | (___    | |  | |__  | |__) |
   / /    \   / |  _  /   / /\ \    > < |  _ <| |  | |\___ \   | |  |  __| |  _  / 
  / /__    | |  | | \ \  / ____ \  / . \| |_) | |__| |____) |  | |  | |____| | \ \ 
 /_____|   |_|  |_|  \_\/_/    \_\/_/ \_\____/ \____/|_____/   |_|  |______|_|  \_\ 

        fast async content discovery, written in Rust
        github.com/ans-inayat/zyraxbuster
"#;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Handle --list-wordlists
    if args.list_wordlists {
        wordlist::list_available_wordlists();
        return;
    }

    // Validate required args
    let url = match &args.url {
        Some(u) => u.clone(),
        None => {
            eprintln!("{} --url is required", "[!] Error:".red().bold());
            std::process::exit(1);
        }
    };

    println!("{}", BANNER.bright_cyan());

    // Resolve wordlist path (auto-detect if needed)
    let wordlist_path = match &args.wordlist {
        Some(w) => wordlist::resolve_wordlist_path(w),
        None => {
            // No wordlist specified — try auto-detection
            match wordlist::auto_detect_wordlist() {
                Some(path) => {
                    println!(
                        "{} auto-detected wordlist: {}",
                        "[+]".green().bold(),
                        path.bright_yellow()
                    );
                    path
                }
                None => {
                    eprintln!(
                        "{} no wordlist specified and no common wordlist found in /usr/share/wordlists/ or /usr/share/seclists/",
                        "[!] Error:".red().bold()
                    );
                    eprintln!(
                        "{} use -w <path> to specify a wordlist, or --list-wordlists to see available ones",
                        "[!]".yellow()
                    );
                    std::process::exit(1);
                }
            }
        }
    };

    println!("{} {}", "[+] Target:".bold(), url);
    println!("{} {}", "[+] Wordlist:".bold(), wordlist_path);
    println!("{} {}", "[+] Threads:".bold(), args.threads);
    if let Some(ext) = &args.extensions {
        println!("{} {}", "[+] Extensions:".bold(), ext);
    }
    if args.random_agent {
        println!("{} {}", "[+] User-Agent:".bold(), "random (per request)".dimmed());
    } else {
        println!("{} {}", "[+] User-Agent:".bold(), args.user_agent);
    }
    if args.delay > 0 {
        println!("{} {}ms", "[+] Delay:".bold(), args.delay);
    }
    println!(
        "{} {}",
        "[+] Excluded status codes:".bold(),
        args.blacklist_codes
    );
    println!();

    let words = match wordlist::load_wordlist(&wordlist_path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!(
                "{} could not read wordlist '{}': {}",
                "[!] Error:".red().bold(),
                wordlist_path,
                e
            );
            std::process::exit(1);
        }
    };

    if words.is_empty() {
        eprintln!("{} wordlist is empty", "[!] Error:".red().bold());
        std::process::exit(1);
    }

    let candidates = wordlist::build_candidates(&words, &args.extensions, args.add_slash);
    println!(
        "{} {} candidate paths generated\n",
        "[+] Loaded:".bold(),
        candidates.len()
    );

    let output_path = args.output.clone();
    let json_output = args.json;
    let results = scanner::run_scan(args, url, candidates).await;

    println!(
        "\n{} scan complete, {} results found",
        "[+]".green().bold(),
        results.len()
    );

    if let Some(path) = output_path {
        match File::create(&path) {
            Ok(mut f) => {
                if json_output {
                    // JSON output
                    let json_results: Vec<serde_json::Value> = results
                        .iter()
                        .map(|r| {
                            serde_json::json!({
                                "url": r.url,
                                "status": r.status,
                                "length": r.length,
                                "redirect": r.redirect,
                            })
                        })
                        .collect();
                    let _ = serde_json::to_writer_pretty(&mut f, &json_results);
                } else {
                    for r in &results {
                        let _ =
                            writeln!(f, "{} [Status: {}] [Size: {}]", r.url, r.status, r.length);
                    }
                }
                println!("{} results written to {}", "[+]".green().bold(), path);
            }
            Err(e) => {
                eprintln!("{} could not write output file: {}", "[!] Error:".red().bold(), e)
            }
        }
    }
}
