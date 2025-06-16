---
theme: minimal
---

# Oneshot channel

A one-shot channel is used for sending a single message between asynchronous tasks.
The `channel` function is used to create a `Sender` and `Receiver` handle pair that form the channel. ℹ️ The channel can be used only  **once**.

```rs
    #[tokio::main]
    async fn () {
        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            if let Err(_) = tx.send(3) {
                println!("the receiver dropped");
            }
        });

        match rx.await {
            Ok(v) => println!("got = {:?}", v),
            Err(_) => println!("the sender dropped"),
        }
    }
```