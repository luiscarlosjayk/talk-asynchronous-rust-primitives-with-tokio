async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    tokio::spawn(heartbeat(1));
    let (status_1, status_2, blocking_result) = tokio::join!(
        get_status("https://docs.rs/reqwest/0.12.15/reqwest"),
        get_status("https://docs.rs/tokio/latest/tokio"),
        async { tokio::task::spawn_blocking(blocking_synchronous_call).await }
    );
    println!("status 1: {}", status_1.unwrap());
    println!("status 2: {}", status_2.unwrap());
    println!("blocking_result: {}", blocking_result.unwrap());
    println!(
        "Total execution time: {}ms",
        start_time.elapsed().as_millis()
    );
    Ok(())
}