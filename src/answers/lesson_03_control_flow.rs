// Lesson 03 — Control Flow
// Exercises: if/else expression, for loop (FizzBuzz example), and loop returning a value via break.
// Entry: pub fn run()
pub fn run() {
    // Exercise 3-1: if/else expression
    let temperature = 24;
    let weather = if temperature > 30 {
        "Hot"
    } else if temperature < 10 {
        "Cold"
    } else {
        "Nice"
    };
    println!("Weather: {}", weather);

    // Exercise 3-2: FizzBuzz using for loop
    for number in 1..=20 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
    }

    // Exercise 3-3: loop that returns a value with break
    let mut attempts = 0;
    let result = loop {
        attempts += 1;

        if attempts == 5 {
            break "Connection established";
        }
    };

    println!("{}", result);
}

fn main() {
    run();
}
