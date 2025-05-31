readme_content = """
# Rust Web Scraper with Robots.txt Compliance

A simple asynchronous web scraper built in Rust using Tokio and Reqwest that:

- Respects `robots.txt` rules via [`robotstxt_with_cache`](https://crates.io/crates/robotstxt_with_cache)
- Parses and follows links on pages
- Handles errors gracefully

---

## Features

- Async HTTP requests with `reqwest` and `tokio`
- HTML parsing with `scraper`
- Domain-limited crawling using `url` crate
- Robots.txt fetching and caching to respect crawl rules
- Command-line configurable start URL (currently hardcoded)
- Prints discovered links on the console

---

## Getting Started

### Prerequisites

- Rust toolchain (stable) installed: [https://rustup.rs/](https://rustup.rs/)

### Build and Run

```bash
cargo run --release
