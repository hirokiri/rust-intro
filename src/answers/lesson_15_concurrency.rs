// Lesson 15 — Concurrency (Threads, Channels, Mutex)

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

pub fn run() {
    println!("Lesson 15 — Concurrency examples");

    // Simple thread spawn and join
    let handle = thread::spawn(|| {
        println!("Hello from a spawned thread");
    });
    handle.join().unwrap();

    // Channel example
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("message from thread").unwrap();
    });
    println!("Received: {}", rx.recv().unwrap());

    // Shared state with Arc<Mutex<T>>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("Counter: {}", *counter.lock().unwrap());
}
