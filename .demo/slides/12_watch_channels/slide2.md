---
theme: minimal
---

# Watch channel

## How it works?

The Receiver half provides an asynchronous changed method. This method is ready when a new, unseen value is sent via the Sender half.

`Receiver::changed()` returns `Ok(())` on receiving a new value, or `Err(error::RecvError)` if the Sender has been dropped.

If the current value is seen, then it will sleep until either a new message is sent via the Sender half, or the Sender is dropped.

New Receiver instances can be created with `Sender::subscribe()`. The current value at the time the Receiver is created is considered seen.

If the receiver intends to await notifications from changed in a loop, `Receiver::borrow_and_update()` should be preferred over `Receiver::borrow()`
