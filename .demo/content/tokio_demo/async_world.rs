async fn say_world() -> String {
    "world".to_string()
}

// We need to add the #[tokio::main] attribute to the main function to use Tokio.
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();
    // This println! comes first
    println!("hello");
    // Calling `.await` on `op` starts executing `say_world`.
    let say_world_result = op.await;
    println!("{}", say_world_result);
}