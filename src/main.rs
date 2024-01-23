use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::sync::Semaphore;
// use tokio::time::{sleep, Duration};
use whois_rust::{WhoIs, WhoIsLookupOptions};

#[derive(Debug, Serialize, Deserialize)]
struct WhoisInfo {
    #[serde(flatten)]
    other_fields: std::collections::BTreeMap<String, String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to a file containing one ip per line
    #[arg(short, long)]
    file: String,
    /// Path to json servers file
    #[arg(short, long)]
    json: String,
    ///Number of parallel whois workers
    #[arg(short, long, default_value_t = 4)]
    workers: i32,
    ///json fields to extract eg org-name|organization,country
    #[arg(short, long, default_value = "org-name|organization,country")]
    capture: String,
}

fn parse_whois_to_json(whois: &str) -> WhoisInfo {
    // Define the regex pattern
    let pattern_str = r"(?mi)^([a-z-]+):\s*(.+?)$";
    let pattern = Regex::new(pattern_str).unwrap();
    // Create a map to store key-value pairs
    let mut whois_map = WhoisInfo {
        other_fields: std::collections::BTreeMap::new(),
    };
    // Iterate over regex captures
    for capture in pattern.captures_iter(whois) {
        let key = capture.get(1).unwrap().as_str().to_lowercase();
        let value = capture.get(2).unwrap().as_str().trim().to_string();
        // Assign the captured values to the corresponding fields
        whois_map.other_fields.insert(key.to_string(), value);
    }
    whois_map
}

async fn ip(semaphore: Arc<Semaphore>, ip: String) -> WhoisInfo {
    println!("{} is waiting in line", ip);
    let json = whoiser(semaphore, ip).await;
    json
}

async fn whoiser(semaphore: Arc<Semaphore>, ip: String) -> WhoisInfo {
    let permit = semaphore.acquire().await.unwrap();
    let whois = WhoIs::from_path("servers.json").unwrap();
    let result: String = whois
        .lookup(WhoIsLookupOptions::from_string(ip).unwrap())
        .unwrap();
    let json_result = parse_whois_to_json(&result);
    drop(permit);
    json_result
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match File::open(args.file) {
        Ok(file_handle) => {
            let reader = io::BufReader::new(file_handle);
            let mut people_handlers = Vec::new();
            let num_of_tellers = args.workers;
            let semaphore = Semaphore::new(num_of_tellers.try_into().unwrap());
            let semaphore_arc = Arc::new(semaphore);

            for line in reader.lines() {
                match line {
                    Ok(line_content) => {
                        people_handlers.push(tokio::spawn(ip(semaphore_arc.clone(), line_content)))
                    }
                    Err(_e) => {}
                }
            }

            let mut results = Vec::new();
            for handle in people_handlers {
                results.push(handle.await.unwrap());
            }
            for res in results {
                println!("Result: {:?}", res.other_fields.get("country"));
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}
