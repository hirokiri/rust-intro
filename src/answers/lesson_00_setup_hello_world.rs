// Lesson 00 — Setup & "Hello, World!"
// Examples: simple print, ASCII art, and quick cargo commands to try.
// Entry: pub fn run()
pub fn run() {
    // Exercise 0-1: simple prints
    let name = "Hiro";
    let language_count = 4;
    println!(
        "Hi, I'm {} and I know {} programming languages!",
        name, language_count
    );

    println!("  crab");
    println!(" /||\\");
    println!("/ || \\");

    println!("Try these commands:");
    println!("cargo check");
    println!("cargo build");
    println!("cargo build --release");
}

fn main() {
    run();
}
