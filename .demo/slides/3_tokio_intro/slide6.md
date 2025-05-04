---
theme: quantum
---

# Tokio::select!

Macro [tokio::select!](https://docs.rs/tokio/latest/tokio/macro.select.html) waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.

```rs
async fn do_stuff_async() {}

async fn more_async_work() {}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_stuff_async() => {
            println!("do_stuff_async() completed first")
        }
        _ = more_async_work() => {
            println!("more_async_work() completed first")
        }
    };
}
```