    tokio::select!(
        stat = get_status("https://docs.rs/reqwest/0.12.15/reqwest") => println!("status 1: {}", stat.unwrap()),
        stat = get_status("https://docs.rs/tokio/latest/tokio") => println!("status 2: {}", stat.unwrap()),
    );