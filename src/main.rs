use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::sync::Semaphore;
use whois_rust::{WhoIs, WhoIsLookupOptions};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;


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
    #[arg(short, long, default_value = "whois.json")]
    output_json: String,
    ///Number of parallel whois workers
    #[arg(short, long, default_value_t = 4)]
    workers: i32,
    //json fields to extract eg name:org-name|organization,country
    // #[arg(short, long, default_value = "name:org-name|organization,country")]
    // capture: String,
    ///pretty print output json
    #[arg(short, long)]
    pretty: bool,
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

async fn ip(semaphore: Arc<Semaphore>, ip: String, progress_bar: Arc<ProgressBar>) -> WhoisInfo {
    let json = whoiser(semaphore, ip, progress_bar).await;
    json
}

async fn whoiser(semaphore: Arc<Semaphore>, ip: String, progress_bar: Arc<ProgressBar>) -> WhoisInfo {
    // progress_bar.println(format!("\n{} is being served by the teller", ip));
    let permit = semaphore.acquire().await.unwrap();
    let whois = WhoIs::from_path("servers.json").unwrap();
    let result: String = whois
        .lookup(WhoIsLookupOptions::from_string(ip.clone()).unwrap())
        .unwrap();
    let mut json_result = parse_whois_to_json(&result);
    json_result
        .other_fields
        .insert("ip".to_string(), ip.clone());
    // progress_bar.println(format!("{} is now leaving the teller", ip));
    progress_bar.inc(1);
    progress_bar.set_message(ip);
    drop(permit);
    json_result
}

fn get_value<'a>(map: &'a BTreeMap<String, String>, keys_to_try: Vec<&'a str>) -> &'a str {
    for key in keys_to_try {
        if let Some(value) = map.get(key) {
            return value;
        }
    }
    // If none of the keys are present, return a default value
    "unk"
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    // let mut parts: Vec<Vec<&str>> = Vec::<Vec<&str>>::new();
    // let binding = args.capture.to_lowercase();
    // let captures: Vec<&str> = binding.split(',').collect();
    // for alt in &captures {
    //     let alt_vec: Vec<&str> = alt.split('|').collect();
    //     parts.push(alt_vec);
    // }

    
    match File::open(&args.file) {
        Ok(file_handle) => {
            let reader = io::BufReader::new(file_handle);
            let mut people_handlers = Vec::new();
            let num_of_tellers = args.workers;
            let semaphore = Semaphore::new(num_of_tellers.try_into().unwrap());
            let semaphore_arc = Arc::new(semaphore);


            let line_count= fs::read_to_string(args.file).expect("Error reading file").lines().count().try_into().unwrap();


            let progress_bar = Arc::new(ProgressBar::new(line_count));            
                progress_bar.set_style(
                    ProgressStyle::with_template("[{bar:40.green}] {pos}/{len} {percent}% {elapsed_precise}/{eta_precise} \t\t[{msg:.blue}]").expect("REASON")
                        .progress_chars("█▇▆▅▄▃▂▁  "));
        

            for line in reader.lines() {
                match line {
                    Ok(line_content) => {                        
                        let progress_bar_clone = progress_bar.clone();
                        people_handlers.push(tokio::spawn(ip(semaphore_arc.clone(), line_content.trim().to_string(), progress_bar_clone )))
                    }
                    Err(_e) => {}
                }
            }

            let mut results = Vec::new();
            for handle in people_handlers {
                results.push(handle.await.unwrap());                
            }

            let mut json_array: Vec<Value> = Vec::new();
            for res in results {
                let keys_to_try = vec!["org-name", "organization", "orgname"];
                let item = json!({
                    res.other_fields.get("ip").unwrap(): json!( {"country":res.other_fields.get("country").unwrap_or(&"unk-country".to_string()),
                "name":get_value(&res.other_fields, keys_to_try),})
                });
                json_array.push(item.clone());
            }

            //Save to json file
            let file = File::create(args.output_json).expect("Failed to create file");
            if args.pretty {
                serde_json::to_writer_pretty(&file, &json_array)
                    .expect("Failed to write JSON to file");
            } else {
                serde_json::to_writer(&file, &json_array).expect("Failed to write JSON to file");
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}
