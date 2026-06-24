// Lesson 01 — Variables & Mutability
// Exercises covered: 1-1..1-4 — mutable variables, mutability, shadowing, parsing and scoping.
// Entry: pub fn run()
pub fn run() {
    // Exercise 1-1
    // This intentionally does not compile:
    //
    // let x = 5;
    // x = 6;
    //
    // Fix: declare it with `mut` if reassignment is intended.
    let mut x = 5;
    x = 6;
    println!("mutable x = {}", x);

    // Exercise 1-2
    let mut temp = 100.0;
    println!("Celsius: {}", temp);
    temp = temp * 9.0 / 5.0 + 32.0;
    println!("Fahrenheit: {}", temp);

    // Exercise 1-3
    let data = "  42  ";
    let data = data.trim();
    let data: i32 = data.parse().unwrap();
    println!("Parsed: {}", data);

    // Exercise 1-4
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner x = {}", x);
    }
    println!("outer x = {}", x);
}

