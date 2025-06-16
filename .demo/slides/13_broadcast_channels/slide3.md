---
theme: minimal
---

# Broadcast channel

## Lagging

The channel's buffer (which we set to a capacity) will fill up. To make room for the new messages, the broadcast channel will start dropping the oldest messages that the slow receiver hasn't read yet.

When lagging task finally calls recv().await, it won't get the next message. Instead, it will get an error: Err(RecvError::Lagged(n)). This error tells the slow consumer, "You fell behind and missed n messages."

This is a critical feature, not a bug! It ensures that one slow client cannot bring down the entire channel for everyone else. The communication moves on, and the slow client is notified that it missed part of the history.