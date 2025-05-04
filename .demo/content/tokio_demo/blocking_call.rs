fn blocking_synchronous_call() -> String {
    std::thread::sleep(Duration::from_secs(5));
    "Finally done".to_string()
}