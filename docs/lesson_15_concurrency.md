Lesson 15 — Concurrency (Threads, Channels, Mutex)

Overview

Covers threads, channels (mpsc), and shared state patterns with Arc and Mutex. Explain Send/Sync traits and common pitfalls (data races, deadlocks).

Examples

- thread::spawn and join
- mpsc::channel for message passing
- Arc<Mutex<T>> for shared mutable state

Exercises

1. Spawn N threads that increment a shared counter using Arc<Mutex<i32>>.
2. Build a producer-consumer pipeline using channels.
3. Explain why Mutex is necessary and what Send/Sync mean.
