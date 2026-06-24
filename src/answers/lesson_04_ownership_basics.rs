// Lesson 04 — Ownership Basics
// Exercises: moving ownership, cloning, functions that take and return ownership (give/take pattern).
// Entry: pub fn run()
pub fn run() {
    // Exercise 4-1: cloning vs moving ownership
    let my_name = String::from("Alice");
    let name_copy = my_name.clone();
    println!("My name is {}", my_name);
    println!("Copied name is {}", name_copy);

    // Exercise 4-2: function that gives ownership
    let owned = give_ownership();
    println!("Received ownership of: {}", owned);

    // Exercise 4-3: take and return ownership
    let message = String::from("Rust ownership");
    let message = take_and_give_back(message);
    println!("Still usable after return: {}", message);
}

fn give_ownership() -> String {
    String::from("owned string")
}

fn take_and_give_back(s: String) -> String {
    println!("Taking ownership of: {}", s);
    s
}

