---
theme: minimal
---

# Barrier

## How it works?

All threads that call `wait` on the Barrier, will wait until the nth task calls wait.

Barriers are re-usable after all tasks have rendezvoused once, and can be used continuously by incoming tasks.

A single (arbitrary) future will receive a `BarrierWaitResult` that returns true from `BarrierWaitResult::is_leader` when returning from this function, and all other tasks will receive a result that will return `false` from is_leader.