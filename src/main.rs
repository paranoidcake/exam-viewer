mod scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    scraper::scrape_exams("./assets/papers").await?;
    
    Ok(())
}