---
theme: minimal
---

# Channels

The most common form of synchronization in a Tokio program is **message passing**. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state.

Message passing is implemented using **channels**. A channel supports sending a message from one producer task to one or more consumer tasks. There are a few flavors of channels provided by Tokio. Each channel flavor supports different message passing patterns. When a channel supports multiple producers, many separate tasks may send messages. When a channel supports multiple consumers, many different separate tasks may receive messages.