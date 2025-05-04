use std::time::Instant;
// We need to add the #[tokio::main] attribute to the main function to use Tokio.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let status_1 = get_status("https://docs.rs/reqwest/0.12.15/reqwest").await?;
    println!("status 1: {}", status_1);
    let status_2 = get_status("https://docs.rs/tokio/latest/tokio").await?;
    println!("status 2: {}", status_2);
    println!(
        "Total execution time: {}ms",
        start_time.elapsed().as_millis()
    );
    Ok(())
}
// Get the status of a website and measure the time it takes to fetch it
async fn get_status(url: &str) -> Result<reqwest::StatusCode, reqwest::Error> {
    let start_time = Instant::now();
    let resp = reqwest::get(url).await?;
    let duration = start_time.elapsed().as_millis();
    println!("Took {:?}ms to fetch url '{}'", duration, url);
    Ok(resp.status())
}