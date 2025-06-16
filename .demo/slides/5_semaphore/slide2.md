---
theme: minimal
---

# Semaphore

## How it works?

When acquire is called and the semaphore has remaining permits, the function immediately returns a permit. However, if no remaining permits are available, acquire (asynchronously) waits until an outstanding permit is dropped. At this point, the freed permit is assigned to the caller.