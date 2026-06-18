# Lesson 5: Borrowing & References 🤝

In Lesson 4, we saw how Rust's **Ownership** rules can feel a bit restrictive. If passing a value to a function always transfers ownership (moves it), we have to constantly return values just to keep using them. 

To solve this, Rust uses a concept called **References and Borrowing**. Instead of passing ownership, we can pass a pointer to the data.

---

## What is Borrowing?

Think of ownership like owning a book. 
- If you **give** the book to a friend (Ownership Move), you can no longer read it.
- If you **lend** the book to a friend (Borrowing), they can read it, but they must return it to you eventually. You remain the owner.

In Rust, we create a reference by using the ampersand (`&`) symbol.

```rust
fn main() {
    let s1 = String::from("hello");

    // We pass &s1, which is a reference to s1.
    // We are "borrowing" the value, not moving ownership.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len); // ✅ s1 is still valid here!
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it doesn't own what it points to,
  // nothing happens. The underlying String is NOT dropped!
```

---

## Stack representation of References

Under the hood, a reference is just a pointer to the memory where the original owner stores its data:

```
Stack (calculate_length)
[ s ] -------> Stack (main) [ s1 ] -------> Heap [ "hello" ]
```

---

## Mutable References (`&mut`)

By default, references are **immutable** (read-only). You cannot modify something you have borrowed.

If you want to modify a borrowed value, you must use a **mutable reference**: `&mut`.

```rust
fn main() {
    let mut s = String::from("hello"); // The original variable MUST be mutable too!

    change(&mut s); // Pass a mutable reference
    
    println!("{}", s); // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

---

## The Golden Rules of References ⚖️

This is where the compiler (the Borrow Checker) becomes strict to prevent data races and memory bugs. Rust enforces **two main rules** at compile time:

1. **You can have either:**
   - Any number of **immutable references** (`&T`) to a resource, **OR**
   - Exactly one **mutable reference** (`&mut T`) to a resource.
2. **References must always be valid** (no dangling references).

Let's break down *why* these rules exist.

### Rule 1: One Mutable OR Many Immutables

A **data race** happens when:
- Two or more pointers access the same memory at the same time.
- At least one of the pointers is writing/modifying.
- There is no synchronization.

By enforcing "One Mutable OR Many Immutables", Rust guarantees data race prevention at compile time.

#### Example: Violating Rule 1 (Two Mutable References)
```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // ❌ ERROR! Cannot borrow `s` as mutable more than once at a time.

println!("{}, {}", r1, r2);
```

#### Example: Violating Rule 1 (Immutable + Mutable)
```rust
let mut s = String::from("hello");

let r1 = &s;     // Fine (immutable borrow)
let r2 = &s;     // Fine (multiple immutable borrows are okay)
let r3 = &mut s; // ❌ ERROR! Cannot borrow `s` as mutable because it is also borrowed as immutable.

println!("{}, {}, and {}", r1, r2, r3);
```
Why is this an error? If `r1` and `r2` are reading the data, they assume the data won't change under their feet. If `r3` writes to the data, `r1` and `r2` could read inconsistent or corrupted state!

---

## Non-Lexical Lifetimes (NLL) 🧠

In older versions of Rust, a reference's scope lasted until the end of the enclosing curly braces `{}`. 
Today, Rust uses **Non-Lexical Lifetimes (NLL)**. The compiler is smart enough to realize that a reference's scope ends **at the last line it is used**, rather than at the end of the block.

This code **compiles successfully**:
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // Immutable borrow starts
    let r2 = &s; // Immutable borrow starts
    println!("{} and {}", r1, r2); // r1 and r2 are last used here

    let r3 = &mut s; // ✅ No error! The compiler knows r1 and r2 are no longer used.
    r3.push_str(", world");
    println!("{}", r3);
}
```

---

## Preventing Dangling References

A **dangling pointer** is a pointer that references a location in memory that has been deallocated or given to someone else. In Rust, the compiler guarantees that references will never be dangling references.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // ❌ ERROR! We return a reference to s, but s goes out of scope and is dropped!
} // Here, s goes out of scope and is dropped. Its memory is gone.
  // The reference would point to invalid memory! Rust prevents this.
```

---

## How this compares to other languages

### Python
In Python, practically everything is a reference to a heap-allocated object.
```python
x = [1, 2, 3]
y = x
y.append(4)
print(x) # [1, 2, 3, 4] - x mutated unexpectedly!
```
Because Python has no compile-time borrowing rules, it is very easy to pass an object to a function or assign it to another variable and have it mutated unexpectedly, leading to hard-to-find bugs. Python uses garbage collection to manage the lifetime of the objects.

### Go
Go has pointers (`*T` and `&T`), but lacks restrictions on multiple mutable borrows.
```go
func modify(slice []int) {
    slice[0] = 999
}
```
You can pass pointers and mutate them from multiple goroutines concurrently, which can lead to data races. Go detects these at runtime using the `-race` detector, but Rust prevents them *entirely at compile time*.

### TypeScript
Like Python, objects and arrays are passed by reference.
```typescript
const user = { name: "Alice" };
const admin = user;
admin.name = "Bob";
console.log(user.name); // "Bob" - Mutated unexpectedly!
```
TypeScript has no built-in protection against shared mutable state. Developers often use libraries like Immutable.js or spreading (`{ ...user }`) to avoid bugs. Rust solves this at the language level without performance overhead.

---

## Key Takeaways
1. **References (`&T`)** allow you to refer to a value without taking ownership.
2. **Borrowing** is the act of creating a reference.
3. **Immutable references (`&T`)** are read-only. You can have unlimited concurrent immutable references.
4. **Mutable references (`&mut T`)** allow modification. You can have **only one** active mutable reference to a piece of data at a time.
5. **No Dangling References:** The compiler ensures that data outlives any reference pointing to it.

---

## Exercises

### Exercise 5-1: The Read-Only View
Create a function called `print_length` that takes an immutable reference to a `String` (i.e., `&String`), prints its length, and does not take ownership. Call it twice in `main` with the same string.

### Exercise 5-2: The Modifier
Write a function `add_suffix(s: &mut String)` that appends the string `" - Modified"` to the string passed to it. In your `main` function:
1. Declare a mutable string.
2. Call `add_suffix` on it.
3. Print the string to verify the modification.

### Exercise 5-3: Borrow Checker Duel
The following code has a borrow checker error. Copy it into your project, read the compiler error using `cargo check`, and fix it so it compiles successfully by adjusting where the references are used or when their scopes end.
```rust
fn main() {
    let mut data = String::from("Rust");

    let ref1 = &data;
    let ref2 = &mut data;

    println!("ref1: {}", ref1);
    ref2.push_str(" is awesome!");
    println!("ref2: {}", ref2);
}
```

---

## Reference
- [The Rust Book — Ch.4.2: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
