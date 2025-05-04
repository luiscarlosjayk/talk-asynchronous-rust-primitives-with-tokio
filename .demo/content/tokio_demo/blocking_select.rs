    tokio::select!(
        stat = get_status("https://docs.rs/reqwest/0.12.15/reqwest") => println!("status 1: {}", stat.unwrap()),
        stat = get_status("https://docs.rs/tokio/latest/tokio") => println!("status 2: {}", stat.unwrap()),
        _ = async {
            let result = blocking_synchronous_call();
            println!("{}", result);
        } => println!("This is going to take a while")
    );