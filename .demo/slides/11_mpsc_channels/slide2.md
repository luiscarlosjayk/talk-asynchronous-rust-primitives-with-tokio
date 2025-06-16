---
theme: minimal
---

# Mpsc channel

## How it works?

There are two methods in `tokio::sync::mpsc` to create the channel:

1. `channel(buffer: usize)`: Creates a bounded mpsc channel for communicating between asynchronous tasks with **backpressure**.

2. `unbounded_channel()`: Creates an unbounded mpsc channel for communicating between asynchronous tasks without backpressure.