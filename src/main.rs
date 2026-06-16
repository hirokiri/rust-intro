fn main() {
    // ==========================================
    // Rule 1 & 3: Ownership and Scope
    // ==========================================
    {
        // s is not valid here, it’s not yet declared
        let s = String::from("hello"); // s is valid from this point forward
        println!("Inside scope: {}", s);
    } // this scope is now over, and `s` is no longer valid. 
      // Rust automatically calls `drop` to free the Heap memory!

    // ==========================================
    // Rule 2: Move Semantics (The "One Owner" Rule)
    // ==========================================
    let s1 = String::from("Rust");
    let s2 = s1; 

    // println!("{}, world!", s1); // ❌ ERROR! s1 has been "moved" into s2.
    // s1 is no longer valid! This prevents "double free" errors.
    println!("s2 owns the string now: {}", s2);

    // ==========================================
    // Deep Copying with `.clone()`
    // ==========================================
    let s3 = String::from("Clone me");
    let s4 = s3.clone(); // This explicitly copies the Heap data.

    // Both are valid because we created a totally separate copy!
    println!("s3: {}, s4: {}", s3, s4);

    // ==========================================
    // Stack-Only Data: The `Copy` Trait
    // ==========================================
    // Primitive types (integers, floats, bools) are stored entirely on the Stack.
    // They implement a special trait called `Copy`.
    let x = 5;
    let y = x; // This is a Copy, not a Move!

    // Because it's on the Stack, copying is cheap, so Rust doesn't invalidate `x`.
    println!("x: {}, y: {}", x, y); // ✅ Both work perfectly!

    // ==========================================
    // Ownership and Functions
    // ==========================================
    let my_string = String::from("Function data");
    
    takes_ownership(my_string); // my_string's value moves into the function...
    // println!("{}", my_string); // ❌ ERROR! my_string is no longer valid here!

    let my_int = 42;
    makes_copy(my_int); // my_int is a Copy type, so it just copies the value.
    println!("my_int is still valid: {}", my_int); // ✅ Works fine!
}

fn takes_ownership(some_string: String) {
    println!("Function took ownership of: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("Function made a copy of: {}", some_integer);
} // some_integer goes out of scope. Nothing special happens because it's just on the stack.