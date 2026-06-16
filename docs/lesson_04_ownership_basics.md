# Lesson 4: Ownership Basics 👑

Ownership is Rust's most unique feature. It enables Rust to make memory safety guarantees without needing a garbage collector.

## The 3 Rules of Ownership
1. Each value in Rust has a variable that’s called its **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value will be **dropped** (memory is freed).

---

## Scope and `drop`

A scope is the range within a program for which an item is valid, usually denoted by `{}`.

```rust
{
    let s = String::from("hello"); // s is valid from here
    // do stuff with s
} // scope ends. s is no longer valid.
// Under the hood, Rust calls a special function `drop` to free the memory here.
```

---

## Stack vs Heap & `String`

- **Stack:** Fast, fixed size. Stores primitives like `i32`, `bool`, `f64`.
- **Heap:** Slower to allocate, dynamic size. Stores things that can grow, like `String` (which is different from the fixed-size `&str` literal).

```rust
let s1 = String::from("hello");
let s2 = s1; 
```
In many languages, `s1` and `s2` would now point to the same memory. 
In Rust, because of the "One Owner" rule, **`s1` is moved into `s2`**. 

`s1` is immediately invalidated. If you try to `println!("{}", s1);`, the compiler will throw an error. This prevents the "double free" bug where two variables try to free the same memory when they go out of scope.

### `.clone()`
If you *want* to deeply copy the heap data, you explicitly use `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2); // ✅ Works perfectly
```

---

## The `Copy` Trait (Stack-Only Data)

Types that have a known, fixed size are stored entirely on the stack. They implement a special trait called `Copy`. 

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y); // ✅ Works!
```
Because `x` is an integer, making a copy is so cheap and fast that Rust doesn't invalidate `x`. 
Types that are `Copy`: Integers, Booleans, Floats, Chars, and Tuples (if they only contain `Copy` types).

---

## Ownership in Functions

Passing a variable to a function behaves exactly like assignment. It will either **move** or **copy**.

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid here! It was moved into the function.

    let x = 5;
    makes_copy(x);
    // x is still valid here, because i32 is Copy.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. Memory freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // goes out of scope, nothing special happens.
```

---

## Key Takeaways
1. **Ownership replaces Garbage Collection.**
2. **Move Semantics:** Assigning a `String` (Heap data) to another variable *invalidates* the first variable.
3. **Copy Semantics:** Primitive types (Stack data) are cheaply copied and remain valid.
4. **Function calls** transfer ownership just like assignment.

---

## Exercises

### Exercise 4-1: The Move
Try to compile this code and read the error message. Then, fix it using `.clone()`.
```rust
fn main() {
    let my_name = String::from("Alice");
    let name_copy = my_name;
    println!("My name is {}", my_name);
}
```

### Exercise 4-2: Return of the Owner
You can return ownership from a function! Write a function `give_ownership()` that creates a `String` and returns it. Assign the result to a variable in `main` and print it.

### Exercise 4-3: Take and Give Back
Write a function `take_and_give_back(s: String) -> String` that takes a string, prints it, and then returns it back to the caller so they can keep using it.

---

## Reference
- [The Rust Book — Ch.4.1: What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
