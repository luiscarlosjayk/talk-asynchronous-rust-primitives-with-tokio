---
theme: minimal
---

# Notify

Notify provides a basic mechanism to notify a single task of an event. Notify itself does not carry any data. Instead, it is to be used to signal another task to perform an operation.

A Notify can be thought of as a Semaphore starting with `0` permits. The `notified().await` method waits for a permit to become available, and `notify_one()` sets a permit if there currently are no available permits.

If `notify_one()` is called before `notified().await`, then the next call to `notified().await` will complete immediately, consuming the permit. Any subsequent calls to `notified().await` will wait for a new permit.

If `notify_one()` is called multiple times before `notified().await`, only a single permit is stored. The next call to `notified().await` will complete immediately, but the one after will wait for a new permit.