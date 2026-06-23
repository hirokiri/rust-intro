// Lesson 02 — Data Types & Functions
// Examples: typed variables (integers, floats, bool, char), tuple swap(), fibonacci(), and block expressions.
// Exercises: demonstrate types and simple functions returning values.
// Entry: pub fn run()
pub fn run() {
    // Exercise 2-1: demonstrate typed variables
    let integer: i64 = 9_223_372_036;
    let float: f32 = 3.14;
    let is_learning: bool = true;
    let crab: char = 'R';

    println!("integer: {}", integer);
    println!("float: {}", float);
    println!("bool: {}", is_learning);
    println!("char: {}", crab);

    // Exercise 2-2: swap two integers using a function
    let (x, y) = swap(10, 20);
    println!("x = {}, y = {}", x, y);

    // Exercise 2-3: fibonacci function examples
    println!("fibonacci(0) = {}", fibonacci(0));
    println!("fibonacci(1) = {}", fibonacci(1));
    println!("fibonacci(7) = {}", fibonacci(7));

    // Exercise 2-4: block expressions and math (area)
    let radius = 5.0_f64;
    let area = {
        let pi = std::f64::consts::PI;
        pi * radius * radius
    };
    println!("Area = {}", area);
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    current
}

fn main() {
    run();
}
