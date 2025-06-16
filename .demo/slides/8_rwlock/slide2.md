---
theme: minimal
---

# RwLock

## How it works?

Threads call `read()` to lock the `RwLock` with a shared read access, causing the current task to yield until the lock has been acquired.

The calling task will yield until there are no writers which hold the lock. There may be other readers inside the lock when the task resumes.

Threads that need exclusive access shall call `write()` to lock the `RwLock` with exclusive write access, causing the current task to yield until the lock has been acquired.

The calling task will yield while other writers or readers currently have access to the lock.