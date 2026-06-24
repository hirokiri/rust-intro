// Lesson 17 — Macros & Advanced Features

// Simple declarative macro example
macro_rules! shout {
    ($msg:expr) => {
        println!("{}!!!", $msg.to_uppercase());
    };
}

pub fn run() {
    println!("Lesson 17 — Macros & Advanced Features");
    shout!("hello macros");

    // Demonstrate using a small closure and iterator adapter (advanced idiom)
    let nums = vec![1, 2, 3];
    let doubled: Vec<_> = nums.iter().map(|n| n * 2).collect();
    println!("doubled = {:?}", doubled);
}
