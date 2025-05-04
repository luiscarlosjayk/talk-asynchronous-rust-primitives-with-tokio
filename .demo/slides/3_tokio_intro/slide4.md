---
theme: quantum
---

# Tokio::join!

[Macro join!](https://docs.rs/tokio/latest/tokio/macro.join.html) waits on multiple concurrent branches, returning when **all** branches complete regardless if any complete with Err.

ℹ️ Use [Macro try_join!](https://docs.rs/tokio/latest/tokio/macro.try_join.html) to return early when Err is encountered.

```rs
async fn do_stuff_async() {}

async fn more_async_work() {}

#[tokio::main]
async fn main() {
    let (first, second) = tokio::join!(
        do_stuff_async(),
        more_async_work()
    );
}
```