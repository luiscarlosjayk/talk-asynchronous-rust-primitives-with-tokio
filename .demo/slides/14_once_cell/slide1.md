---
theme: minimal
---

# OnceCell

A thread-safe cell that can be written to only once.

A OnceCell is typically used for global variables that need to be initialized once on first use, but need no further changes. The OnceCell in Tokio allows the initialization procedure to be asynchronous.