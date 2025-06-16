---
theme: minimal
---

# Semaphore

A semaphore is a non-negative integer variable that is shared between various threads, which works as a signaling mechanism among concurrent callers to access a shared resource at a time.

A semaphore maintains a set of permits. Permits are used to synchronize access to a shared resource. A semaphore differs from a mutex in that it can allow more than one concurrent caller to access the shared resource at a time.