use colored::*;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use reqwest::{Client, StatusCode};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::cli::Args;

#[derive(Debug, Clone)]
pub struct HitResult {
    pub url: String,
    pub status: u16,
    pub length: u64,
    pub redirect: Option<String>,
}

const RANDOM_USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:126.0) Gecko/20100101 Firefox/126.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:125.0) Gecko/20100101 Firefox/125.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:125.0) Gecko/20100101 Firefox/125.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0",
];

pub fn build_client(args: &Args) -> reqwest::Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    for h in &args.header {
        if let Some((k, v)) = h.split_once(':') {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(k.trim().as_bytes()),
                reqwest::header::HeaderValue::from_str(v.trim()),
            ) {
                headers.insert(name, val);
            }
        }
    }

    Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .user_agent(&args.user_agent)
        .default_headers(headers)
        .redirect(if args.follow_redirects {
            reqwest::redirect::Policy::limited(5)
        } else {
            reqwest::redirect::Policy::none()
        })
        .danger_accept_invalid_certs(args.insecure)
        .build()
}

fn parse_code_set(s: &str) -> HashSet<u16> {
    s.split(',')
        .filter_map(|c| c.trim().parse::<u16>().ok())
        .collect()
}

pub async fn run_scan(args: Args, url: String, candidates: Vec<String>) -> Vec<HitResult> {
    let client = build_client(&args).expect("failed to build HTTP client");
    let base = url.trim_end_matches('/').to_string();

    let whitelist = args.status_codes.as_ref().map(|s| parse_code_set(s));
    let blacklist = parse_code_set(&args.blacklist_codes);

    let pb = ProgressBar::new(candidates.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let results: Arc<Mutex<Vec<HitResult>>> = Arc::new(Mutex::new(Vec::new()));
    let concurrency = args.threads.max(1);
    let retries = args.retries;
    let min_length = args.min_length;
    let exclude_length = args.exclude_length;
    let random_agent = args.random_agent;
    let delay = args.delay;

    stream::iter(candidates)
        .for_each_concurrent(concurrency, |word| {
            let client = client.clone();
            let base = base.clone();
            let results = Arc::clone(&results);
            let pb = pb.clone();
            let whitelist = whitelist.clone();
            let blacklist = blacklist.clone();

            async move {
                // Rate limiting delay
                if delay > 0 {
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }

                let target = format!("{}/{}", base, word.trim_start_matches('/'));

                let mut attempt = 0;
                let resp = loop {
                    // Build request with optional random user-agent
                    let mut req = client.get(&target);
                    if random_agent {
                        let agent = RANDOM_USER_AGENTS.choose(&mut rand::thread_rng());
                        if let Some(a) = agent {
                            req = req.header(reqwest::header::USER_AGENT, *a);
                        }
                    }

                    match req.send().await {
                        Ok(r) => break Some(r),
                        Err(_e) if attempt < retries => {
                            attempt += 1;
                            continue;
                        }
                        Err(_) => break None,
                    }
                };

                if let Some(resp) = resp {
                    let status = resp.status();
                    let redirect = resp
                        .headers()
                        .get(reqwest::header::LOCATION)
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    let length = resp.content_length().unwrap_or(0);

                    if should_report(
                        status,
                        &whitelist,
                        &blacklist,
                        length,
                        min_length,
                        exclude_length,
                    ) {
                        let hit = HitResult {
                            url: target.clone(),
                            status: status.as_u16(),
                            length,
                            redirect: redirect.clone(),
                        };
                        print_hit(&hit);
                        results.lock().await.push(hit);
                    }
                }

                pb.inc(1);
            }
        })
        .await;

    pb.finish_with_message("done");
    Arc::try_unwrap(results).unwrap().into_inner()
}

fn should_report(
    status: StatusCode,
    whitelist: &Option<HashSet<u16>>,
    blacklist: &HashSet<u16>,
    length: u64,
    min_length: Option<u64>,
    exclude_length: Option<u64>,
) -> bool {
    let code = status.as_u16();

    if let Some(wl) = whitelist {
        if !wl.contains(&code) {
            return false;
        }
    } else if blacklist.contains(&code) {
        return false;
    }

    if let Some(min) = min_length {
        if length < min {
            return false;
        }
    }
    if let Some(excl) = exclude_length {
        if length == excl {
            return false;
        }
    }

    true
}

fn print_hit(hit: &HitResult) {
    let status_str = format!("{}", hit.status);
    let colored_status = match hit.status {
        200..=299 => status_str.green(),
        300..=399 => status_str.yellow(),
        400..=499 => status_str.red(),
        500..=599 => status_str.magenta(),
        _ => status_str.white(),
    };

    let redirect_info = hit
        .redirect
        .as_ref()
        .map(|r| format!(" -> {}", r))
        .unwrap_or_default();

    println!(
        "{:<60} [{}] [Size: {}]{}",
        hit.url.cyan(),
        colored_status,
        hit.length,
        redirect_info.dimmed()
    );
}

fn print_vhost_hit(hit: &HitResult) {
    let status_str = format!("{}", hit.status);
    let colored_status = match hit.status {
        200..=299 => status_str.green(),
        300..=399 => status_str.yellow(),
        400..=499 => status_str.red(),
        500..=599 => status_str.magenta(),
        _ => status_str.white(),
    };

    let redirect_info = hit
        .redirect
        .as_ref()
        .map(|r| format!(" -> {}", r))
        .unwrap_or_default();

    println!(
        "{:<55} [{}] [Size: {}]{}",
        hit.url.cyan(),
        colored_status,
        hit.length,
        redirect_info.dimmed()
    );
}

/// Make a single baseline request to detect the default response size/status
async fn make_baseline_request(client: &Client, url: &str) -> (StatusCode, u64) {
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let length = resp.content_length().unwrap_or(0);
            (status, length)
        }
        Err(_) => (StatusCode::default(), 0),
    }
}

/// VHost/subdomain fuzzing mode — fuzzes the Host header instead of URL path
pub async fn run_vhost_scan(
    args: Args,
    url: String,
    domain: String,
    candidates: Vec<String>,
) -> Vec<HitResult> {
    let client = build_client(&args).expect("failed to build HTTP client");
    let base = url.trim_end_matches('/').to_string();

    let whitelist = args.status_codes.as_ref().map(|s| parse_code_set(s));
    let blacklist = parse_code_set(&args.blacklist_codes);

    // Auto-detect baseline if --auto-filter is enabled
    let baseline = if args.auto_filter {
        let (b_status, b_length) = make_baseline_request(&client, &base).await;
        println!(
            "{} baseline: {} [Size: {}]",
            "[+] Baseline:".bold(),
            b_status.as_u16(),
            b_length
        );
        Some((b_status.as_u16(), b_length))
    } else {
        None
    };

    let pb = ProgressBar::new(candidates.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let results: Arc<Mutex<Vec<HitResult>>> = Arc::new(Mutex::new(Vec::new()));
    let concurrency = args.threads.max(1);
    let retries = args.retries;
    let min_length = args.min_length;
    let exclude_length = args.exclude_length;
    let random_agent = args.random_agent;
    let delay = args.delay;

    stream::iter(candidates)
        .for_each_concurrent(concurrency, |word| {
            let client = client.clone();
            let base = base.clone();
            let domain = domain.clone();
            let results = Arc::clone(&results);
            let pb = pb.clone();
            let whitelist = whitelist.clone();
            let blacklist = blacklist.clone();
            let baseline = baseline.clone();

            async move {
                if delay > 0 {
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }

                let vhost = format!("{}.{}", word.trim(), domain);
                let target = format!("{}/", base);

                let mut attempt = 0;
                let resp = loop {
                    let mut req = client.get(&target);
                    req = req.header(reqwest::header::HOST, &vhost);

                    if random_agent {
                        let agent = RANDOM_USER_AGENTS.choose(&mut rand::thread_rng());
                        if let Some(a) = agent {
                            req = req.header(reqwest::header::USER_AGENT, *a);
                        }
                    }

                    match req.send().await {
                        Ok(r) => break Some(r),
                        Err(_e) if attempt < retries => {
                            attempt += 1;
                            continue;
                        }
                        Err(_) => break None,
                    }
                };

                if let Some(resp) = resp {
                    let status = resp.status();
                    let redirect = resp
                        .headers()
                        .get(reqwest::header::LOCATION)
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    let length = resp.content_length().unwrap_or(0);

                    // If auto_filter is on, skip responses matching baseline
                    if let Some((b_status, b_length)) = baseline {
                        if status.as_u16() == b_status && length == b_length {
                            pb.inc(1);
                            return;
                        }
                    }

                    if should_report(
                        status,
                        &whitelist,
                        &blacklist,
                        length,
                        min_length,
                        exclude_length,
                    ) {
                        let hit = HitResult {
                            url: vhost,
                            status: status.as_u16(),
                            length,
                            redirect: redirect.clone(),
                        };
                        print_vhost_hit(&hit);
                        results.lock().await.push(hit);
                    }
                }

                pb.inc(1);
            }
        })
        .await;

    pb.finish_with_message("done");
    Arc::try_unwrap(results).unwrap().into_inner()
}
