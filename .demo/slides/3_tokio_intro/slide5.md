---
theme: quantum
---

# Tokio Task

A task is a light weight, non-blocking unit of execution.
A task is similar to an OS thread, but rather than being managed by the OS scheduler, they are managed by the Tokio runtime.

[tokio::task](https://docs.rs/tokio/latest/tokio/task/index.html)

```rs
use tokio::task;

task::spawn(async {
    // perform some work here...
});
```