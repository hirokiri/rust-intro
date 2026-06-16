fn main() {
    // ==========================================
    // 1. `if` expressions
    // ==========================================
    let number = 7;

    // Standard if/else (No parentheses needed around the condition!)
    if number < 5 {
        println!("Condition was true");
    } else if number == 7 {
        println!("Number is 7");
    } else {
        println!("Condition was false");
    }

    // `if` is an expression! You can assign its result to a variable.
    // This is Rust's version of the ternary operator (Python: "a" if cond else "b")
    let condition = true;
    let assigned_number = if condition { 5 } else { 6 };
    println!("Assigned number is: {}", assigned_number);
    
    // Note: Both arms of the `if` expression MUST return the same type.
    // let bad = if true { 5 } else { "six" }; // ❌ ERROR!

    // ==========================================
    // 2. `loop` (Infinite loop)
    // ==========================================
    let mut counter = 0;
    
    // `loop` runs forever until you explicitly `break`.
    // It's like Go's `for { }` or Python's `while True:`
    let result = loop {
        counter += 1;

        if counter == 10 {
            // You can actually return a value from a loop!
            break counter * 2; 
        }
    };
    println!("The result of the loop is: {}", result);

    // ==========================================
    // 3. `while` loops
    // ==========================================
    let mut n = 3;
    
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!!!");

    // ==========================================
    // 4. `for` loops (The most common and safest!)
    // ==========================================
    let a = [10, 20, 30, 40, 50];

    // Iterating over a collection (Safest way: no out-of-bounds errors!)
    // Similar to Go's `for _, val := range a` or Python's `for val in a:`
    for element in a {
        println!("the value is: {}", element);
    }

    // Iterating over a range (inclusive start, exclusive end: `start..end`)
    // To make it inclusive on both sides, use `..=`
    for number in (1..4).rev() { // .rev() reverses the range!
        println!("{}!", number);
    }
    println!("LIFTOFF AGAIN!!!");
}
