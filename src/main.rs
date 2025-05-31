use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use url::{Url};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let start_url = "https://example.com".to_string();

    let mut queue: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut failed_urls: Vec<String> = Vec::new();
    let mut succeeded_urls: Vec<String> = Vec::new();

    let selector = Selector::parse("a").expect("Failed to parse selector");
    queue.push_back(start_url.clone());
    visited.insert(start_url.clone());

    while let Some(url) = queue.pop_front() {
        println!("\nVisiting: {}", url);

        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("Request error for {}: {}", url, err);
                failed_urls.push(url);
                continue;
            }
        };

        if !response.status().is_success() {
            eprintln!("Non-success status for {}: {}", url, response.status());
            failed_urls.push(url);
            continue;
        }

        let body = match response.text().await {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Error reading body from {}: {}", url, err);
                failed_urls.push(url);
                continue;
            }
        };

        let base_url = match Url::parse(&url) {
            Ok(base) => base,
            Err(err) => {
                eprintln!("Failed to parse base URL {}: {}", url, err);
                continue;
            }
        };

        {
            let document = Html::parse_document(&body);

            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    if let Ok(resolved_url) = base_url.join(href) {
                        let link = resolved_url.to_string();

                        if !visited.contains(&link) {
                            println!("  → Found: {}", link);
                            visited.insert(link.clone());
                            queue.push_back(link);
                        }
                    }
                }
            }
        }

        succeeded_urls.push(url);
    }

    println!("\nScraping complete.");
    println!("Succeeded: {}", succeeded_urls.len());
    println!("Failed: {}", failed_urls.len());
}
