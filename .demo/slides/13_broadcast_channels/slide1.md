---
theme: minimal
---

# Broadcast channel

A multi-producer, multi-consumer broadcast queue. Each sent value is seen by all consumers.


Each consumer will receive each value. This channel can be used to implement “fan out” style patterns common with pub / sub or “chat” systems.

This is also the channel you should use if you want to broadcast values from a single producer to many consumers. There is no dedicated spmc broadcast channel.