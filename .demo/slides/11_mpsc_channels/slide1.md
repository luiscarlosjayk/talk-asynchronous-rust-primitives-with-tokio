---
theme: minimal
---

# Mpsc channel

A multi-producer, single-consumer queue for sending values between asynchronous tasks.

This module provides two variants of the channel: bounded and unbounded. The bounded variant has a limit on the number of messages that the channel can store, and if this limit is reached, trying to send another message will wait until a message is received from the channel. An unbounded channel has an infinite capacity, so the send method will always complete immediately.

> ℹ️ This channel is also suitable for the single-producer single-consumer use-case.