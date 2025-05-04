    let (status_1, status_2, blocking_result) = tokio::join!(
        get_status("https://docs.rs/reqwest/0.12.15/reqwest"),
        get_status("https://docs.rs/tokio/latest/tokio"),
        async { blocking_synchronous_call() }
    );
    println!("status 1: {}", status_1.unwrap());
    println!("status 2: {}", status_2.unwrap());
    println!("blocking_result: {}", blocking_result);