---
theme: quantum
---

# Tokio Setup

1. Let's create our tokio_setup package with the help of Cargo.
We'll name the folder as 1_tokio_setup and the package as tokio_setup.

ℹ️ Notice that we had to specify the package name differently than the folder name because package's names cannot start with numbers.

```bash
cd src/examples
cargo new 1_tokio_setup --name tokio_setup
cargo add tokio --features full
```