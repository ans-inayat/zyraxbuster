use clap::Parser;

/// zyraxbuster - a fast async directory/content discovery tool written in Rust
#[derive(Parser, Debug, Clone)]
#[command(
    name = "zyraxbuster",
    version,
    about = "Fast async directory/file brute-forcer (gobuster/ffuf-style) written in Rust",
    long_about = "ZyraxBuster is a fast, asynchronous content discovery tool.\n\nGitHub: https://github.com/ans-inayat/zyraxbuster\n\nSupports auto-detection of wordlists from common paths:\n  /usr/share/wordlists/seclists/Discovery/Web-Content/\n  /usr/share/seclists/Discovery/Web-Content/\n  /usr/share/wordlists/\n  /usr/share/dirbuster/wordlists/"
)]
pub struct Args {
    /// Target base URL, e.g. https://example.com (required unless --list-wordlists)
    #[arg(short, long)]
    pub url: Option<String>,

    /// Path to wordlist file. If omitted, auto-detects from /usr/share/wordlists/
    #[arg(short, long)]
    pub wordlist: Option<String>,

    /// Number of concurrent workers
    #[arg(short = 't', long, default_value_t = 40)]
    pub threads: usize,

    /// Comma separated list of extensions to append, e.g. php,html,txt
    #[arg(short = 'x', long)]
    pub extensions: Option<String>,

    /// Comma separated list of status codes to show (default: all except -b excluded)
    #[arg(short = 's', long)]
    pub status_codes: Option<String>,

    /// Comma separated list of status codes to hide/exclude
    #[arg(short = 'b', long, default_value = "404")]
    pub blacklist_codes: String,

    /// Request timeout in seconds
    #[arg(long, default_value_t = 10)]
    pub timeout: u64,

    /// Follow redirects
    #[arg(short = 'r', long, default_value_t = false)]
    pub follow_redirects: bool,

    /// Custom User-Agent
    #[arg(long, default_value = "zyraxbuster/0.1")]
    pub user_agent: String,

    /// Use a random User-Agent per request
    #[arg(long, default_value_t = false)]
    pub random_agent: bool,

    /// Add a trailing slash to each probed word
    #[arg(long, default_value_t = false)]
    pub add_slash: bool,

    /// Output results to a file
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Output results in JSON format (use with -o)
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Only show results with this minimum content-length (bytes), useful for filtering noise
    #[arg(long)]
    pub min_length: Option<u64>,

    /// Hide results with this exact content-length (bytes), useful for filtering soft-404 pages
    #[arg(long)]
    pub exclude_length: Option<u64>,

    /// Extra custom header(s), format "Key: Value", can be repeated
    #[arg(short = 'H', long)]
    pub header: Vec<String>,

    /// Skip TLS certificate verification
    #[arg(short = 'k', long, default_value_t = false)]
    pub insecure: bool,

    /// Retry count on network error
    #[arg(long, default_value_t = 1)]
    pub retries: u32,

    /// Delay between requests in milliseconds (rate limiting)
    #[arg(long, default_value_t = 0)]
    pub delay: u64,

    /// List available wordlists from common paths and exit
    #[arg(long, default_value_t = false)]
    pub list_wordlists: bool,
}
