fn main() {
    dotenv::dotenv().ok();
    match std::env::var("GEMINI_API_KEY") {
        Ok(key) => println!("API key found: {}...", &key[..8.min(key.len())]),
        Err(e) => println!("Error loading API key: {}", e),
    }
}
