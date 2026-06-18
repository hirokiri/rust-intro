# Lesson 6: Lifetimes Basics ⏳

In Lesson 5, we learned about **References & Borrowing**. We established that one of Rust's golden rules is: **References must always be valid** (no dangling references).

To enforce this rule, Rust uses a concept called **Lifetimes**. A lifetime is the scope for which a reference is valid.

---

## What is a Lifetime?

Every reference in Rust has a lifetime. Most of the time, lifetimes are implicit and inferred—you don't have to think about them because the compiler automatically calculates them. 

However, when the relationship between references gets complex, the compiler needs our help. We use **explicit lifetime annotations** to explain how the lifetimes of different references relate to one another.

### The Problem: The Out-of-Scope Borrow
To understand lifetimes, let's look at what the compiler is trying to prevent:

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); // ❌ ERROR! |
}                         // ---------+
```
Here:
- `'a` is the lifetime of `r`.
- `'b` is the lifetime of `x`.
- At the `println!`, `x` is out of scope and has been dropped. But `r` is still trying to reference it!
- The compiler compares the size of the two lifetimes and sees that `'b` is shorter than `'a`. Since the reference `r` is trying to outlive its target `x`, the compiler rejects the code.

---

## Explicit Lifetime Annotations

Lifetime annotations don't change how long any of the references live. Rather, they **describe the relationships** between the lifetimes of multiple references.

Lifetime names start with an apostrophe (`'`) and are usually lowercase and very short (like `'a`, `'b`, or `'c`).

```rust
&i32        // A reference
&'a i32     // A reference with an explicit lifetime
&'a mut i32 // A mutable reference with an explicit lifetime
```

### The Classic Example: The `longest` Function

Suppose we want a function that takes two string slices and returns the longer one:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// ❌ THIS WILL NOT COMPILE!
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
If you try to compile this, Rust complains: **"missing lifetime specifier"**. 

Why? Because the return type is a reference (`&str`), but Rust doesn't know if the returned reference points to `x` or `y`. Since `x` and `y` might have different lifetimes, Rust cannot guarantee at compile time that the returned reference will remain valid.

#### The Fix: Annotating Lifetimes
We fix this by introducing a lifetime parameter `'a` to define the relationship:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
**What does this signature mean?**
- We declare a generic lifetime `'a` inside the angle brackets `<'a>`.
- We state that both inputs `x` and `y` are borrowed for **at least** the lifetime `'a`.
- We state that the returned reference will also live for **at least** the lifetime `'a`.
- In practice, `'a` will be equal to the **smaller** of the two lifetimes of `x` and `y`.

---

## Non-Lexical Lifetimes (NLL) and Scopes

Let's see how the compiler evaluates this when we call it:

```rust
fn main() {
    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("xyz");
        // string2 has a smaller scope than string1.
        // Therefore, the result (which borrows from them) can only live as long as string2!
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // ✅ Works! result is used while string2 is still valid.
    } 
    // string2 goes out of scope here.
}
```

If we try to use `result` *outside* the inner scope, it will fail:

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // ❌ Borrow checker error!
    } 
    // string2 is dropped here!
    println!("The longest string is {}", result); // result contains a reference to string2 which is dangling!
}
```

---

## Lifetime Elision (Implicit Lifetimes)

In early versions of Rust, every single reference required an explicit lifetime annotation. To make code cleaner, the Rust team analyzed patterns and added **Lifetime Elision Rules** to the compiler.

If your code fits these three rules, the compiler adds the lifetimes automatically:

1. **Rule 1 (Inputs):** Each parameter that is a reference gets its own lifetime parameter. 
   - `fn foo(x: &i32, y: &i32)` becomes `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`.
2. **Rule 2 (Single Output):** If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
   - `fn foo(x: &i32) -> &i32` becomes `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. **Rule 3 (Methods):** If there are multiple input lifetimes, but one of them is `&self` or `&mut self` (for methods), the lifetime of `self` is assigned to all output lifetimes.

---

## How this compares to other languages

### Python, Go, and TypeScript
In garbage-collected languages, lifetimes are not tracked by the compiler.
* **Python/TypeScript:** The runtime counts references or performs mark-and-sweep. If a variable is still referenced, it remains alive in memory. Once references hit zero, it is swept away.
* **Go:** Uses escape analysis. If you return a reference to a local variable, the compiler detects this and automatically allocates the variable on the **heap** instead of the stack.

### C and C++
C/C++ allows you to return pointers to variables that have gone out of scope, causing **dangling pointers** and undefined behavior.
```cpp
int* get_dangling_ptr() {
    int x = 42;
    return &x; // ❌ DANGEROUS! x is destroyed when function returns.
}
```
Rust's lifetimes completely eliminate this category of bugs by verifying scopes at compile time with **zero runtime cost**.

---

## Key Takeaways
1. **Lifetimes** ensure that references never point to invalid memory (dangling pointers).
2. **Lifetime Annotations (`'a`)** do not change how long a reference lives; they only declare the relationship between multiple reference lifetimes.
3. **The Borrow Checker** uses lifetimes to ensure a reference's lifetime is never longer than the owner's lifetime.
4. **Lifetime Elision** allows us to omit annotations in common, predictable patterns.

---

## Exercises

### Exercise 6-1: Return of the Longest Str
Implement the `longest` function from this lesson in `src/main.rs`. In `main`:
1. Call `longest` passing two strings of different lifetimes (one in `main`'s outer scope, one inside an inner `{}` block).
2. Make sure you use the returned result inside the inner block where both variables are still valid.
3. Observe what happens if you try to print the result *after* the inner block closes.

### Exercise 6-2: The First Word
Write a function `first_word(s: &str) -> &str` that returns a slice containing the first word of a sentence (up to the first space). If there is no space, it returns the whole string. 
- *Note:* Do you need explicit lifetime annotations for `first_word`? Explain why or why not based on the **Lifetime Elision Rules**.

### Exercise 6-3: Lifetimes in Structs (Intro)
Lifetimes are also required when a struct holds a reference!
Look at the following code, add the correct lifetime annotations so that the struct `Book` can hold a reference to a string slice, and implement it in `src/main.rs`:
```rust
// Todo: Add lifetime annotations to Book
struct Book {
    title: &str, 
    author: &str,
}

fn main() {
    let title = String::from("The Rust Book");
    let author = String::from("Steve Klabnik");
    
    let my_book = Book {
        title: &title,
        author: &author,
    };
    
    println!("Book: {} by {}", my_book.title, my_book.author);
}
```

---

## Reference
- [The Rust Book — Ch.10.3: Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
