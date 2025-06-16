---
theme: minimal
---

# RwLock

This type of lock allows a number of readers or at most one writer at any point in time. The write portion of this lock typically allows modification of the underlying data (exclusive access) and the read portion of this lock typically allows for read-only access (shared access).

In comparison, a Mutex does not distinguish between readers or writers that acquire the lock, therefore causing any tasks waiting for the lock to become available to yield. An RwLock will allow any number of readers to acquire the lock as long as a writer is not holding the lock.