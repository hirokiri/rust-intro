// Lesson 15 — Concurrency (Threads, Channels, Mutex)

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn run() {
    println!("Lesson 15 — Concurrency examples");

    // 1) Spawn threads that return values via JoinHandle
    let handles: Vec<_> = (0..4)
        .map(|i| {
            // capture i as i32 for return, but convert to u64 for Duration
            thread::spawn(move || {
                let millis = 10_u64 * (i as u64);
                thread::sleep(Duration::from_millis(millis));
                (i * 2) as i32
            })
        })
        .collect();

    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("Collected thread results: {:?}", results);

    // 2) mpsc channel with multiple producers
    let (tx, rx) = mpsc::channel();
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("msg {}", i)).unwrap();
        });
    }
    // Drop original sender so iterator ends when all clones are dropped
    drop(tx);
    for msg in rx.iter() {
        println!("Received via channel: {}", msg);
    }

    // 3) Shared state with Arc<Mutex<T>> and avoiding deadlocks
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // lock scope is minimal to avoid holding lock while doing other work
            {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
            // do other non-shared work here
            thread::sleep(Duration::from_millis(5));
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("Counter after threads: {}", *counter.lock().unwrap());

    // 4) Note: data races are prevented by Rust's ownership and Sync/Send traits.
    println!("Concurrency demo complete.");
}
