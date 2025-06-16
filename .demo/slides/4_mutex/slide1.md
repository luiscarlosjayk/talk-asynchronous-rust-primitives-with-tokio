---
theme: minimal
---

# Sync Mutexes

Mutex stands for mutual exclusive access, which means that it's a mechanism to block the threads that access a shared resource. Therefore, we're communicating by sharing memory.

Only one thread can hold the lock to the Mutex, other threads wanting to do same, will be blocked until the lock is released.