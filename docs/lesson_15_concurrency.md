Lesson 15 — Concurrency (Threads, Channels, Mutex)

Overview

This lesson covers Rust's concurrency primitives: threads, channels (mpsc), and shared-state patterns with Arc + Mutex. Rust prevents data races at compile time by requiring types to be Send/Sync when shared between threads.

Key concepts

- thread::spawn — create OS threads, returns JoinHandle
- mpsc::channel — multi-producer, single-consumer message passing
- Arc<T> — atomically reference-counted pointer for sharing ownership across threads
- Mutex<T> — mutual exclusion for interior mutability
- Send and Sync marker traits

Code examples

- Threads producing values with JoinHandle
- Multiple producers sending messages to one receiver
- Arc<Mutex<T>> with minimal lock scope to avoid deadlocks

Exercises

1. Spawn N threads that increment a shared counter using Arc<Mutex<i32>> and measure time.
2. Implement a producer-consumer pipeline where producers send work items and a consumer aggregates results.
3. Explain what types must implement Send/Sync and why !Send types (like raw pointers) cannot be moved between threads.

Notes

- Keep lock scopes short: lock only while accessing shared data, not during expensive work.
- Prefer message passing for concurrency where possible — it simplifies reasoning about shared state.
