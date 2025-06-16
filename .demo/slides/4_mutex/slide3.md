---
theme: minimal
---

# Async Mutexes

This type acts similarly to `std::sync::Mutex`, with two major differences: `lock` is an async method so does not block, and the lock guard is designed to be held across `.await` points.

This makes the async mutex more expensive than the blocking mutex, so the blocking mutex should be preferred in the cases where it can be used. The primary use case for the async mutex is to provide shared mutable access to IO resources such as a database connection.

If the value behind the mutex is just data, it’s usually appropriate to use a blocking mutex such as the one in the standard library.