// Lesson 05 — Borrowing & References
// Exercises: immutable & mutable borrows, &String parameter length, &mut modifications, and borrowing rules examples.
// Entry: pub fn run()
pub fn run() {
    // Exercise 5-1: immutable borrow &String
    let language = String::from("Rust");
    print_length(&language);
    print_length(&language);

    // Exercise 5-2: mutable borrow &mut modifications
    let mut message = String::from("Borrow me");
    add_suffix(&mut message);
    println!("{}", message);

    // Exercise 5-3: demonstrate borrowing rules (immutable then mutable)
    let mut data = String::from("Rust");
    let ref1 = &data;
    println!("ref1: {}", ref1);

    let ref2 = &mut data;
    ref2.push_str(" is awesome!");
    println!("ref2: {}", ref2);
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn add_suffix(s: &mut String) {
    s.push_str(" - Modified");
}

fn main() {
    run();
}
