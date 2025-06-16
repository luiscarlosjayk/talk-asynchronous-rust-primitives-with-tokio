---
theme: minimal
---

# Broadcast channel

## How it works?

A `Sender` is used to broadcast values to all connected `Receiver` values. `Sender` handles are clone-able, allowing concurrent send and receive actions. `Sender` and `Receiver` are both Send and Sync as long as T is Send.

When a value is sent, all `Receiver` handles are notified and will receive the value. The value is stored once inside the channel and cloned on demand for each receiver. Once all receivers have received a clone of the value, the value is released from the channel.

A channel is created by calling `channel`, specifying the maximum number of messages the channel can retain at any given time.

New Receiver handles are created by calling `Sender::subscribe`. The returned `Receiver` will receive values sent after the call to subscribe.