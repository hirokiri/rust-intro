# Lesson 10: Error Handling

Rust treats errors as ordinary values whenever possible. Instead of relying mostly on exceptions, Rust asks you to say, in the type system, whether an operation might fail.

This lesson focuses on three tools:

- `panic!`: stop the program because something unrecoverable happened
- `Option<T>`: represent a value that may be present or absent
- `Result<T, E>`: represent success or failure with error information

The big mindset shift is this: Rust does not want hidden control flow for expected failures. If a file might be missing, a string might not parse, or a lookup might fail, the function signature should usually reveal that.

---

## 1. Panic: Unrecoverable Failure

`panic!` immediately stops the current thread. In a simple program, that usually means the whole program exits.

```rust
fn main() {
    panic!("something went very wrong");
}
```

You have already seen panics indirectly:

```rust
let numbers = vec![10, 20, 30];
println!("{}", numbers[99]); // panic: index out of bounds
```

Indexing with `[]` says, "I am certain this index exists." If you are wrong, Rust panics.

Use panic for bugs, broken invariants, and situations where continuing would be misleading or dangerous. Do not use panic for ordinary user mistakes like "file not found" or "invalid number in input."

### `unwrap` and `expect`

`unwrap` extracts the value from `Some` or `Ok`, but panics on `None` or `Err`.

```rust
let text = "42";
let number: i32 = text.parse().unwrap();

println!("{}", number);
```

This works because `"42"` parses successfully. But this panics:

```rust
let text = "not a number";
let number: i32 = text.parse().unwrap();

println!("{}", number);
```

`expect` is like `unwrap`, but lets you provide the panic message:

```rust
let number: i32 = "42"
    .parse()
    .expect("expected a valid integer");
```

During quick experiments, `unwrap` and `expect` are fine. In production-style code, they should make you pause and ask: "Is this failure actually impossible, or should I handle it?"

---

## 2. `Option<T>`: Maybe There Is a Value

`Option<T>` represents absence:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

You used this with `Vec::get` and `HashMap::get`:

```rust
let scores = vec![92, 81, 77];

match scores.get(1) {
    Some(score) => println!("score: {}", score),
    None => println!("no score at that index"),
}
```

This is Rust replacing a common source of runtime bugs: null or undefined values.

### Compared to Python, Go, and TypeScript

Python often uses `None`:

```python
def find_user(id):
    return None
```

TypeScript has `undefined`, `null`, and union types:

```typescript
function findUser(id: string): User | undefined {
    return undefined;
}
```

Go often returns a zero value plus a boolean:

```go
value, ok := scores["blue"]
```

Rust's `Option<T>` is closest to TypeScript's explicit union style, but stricter at runtime. You cannot accidentally use an `Option<i32>` as an `i32`; you must unwrap, match, or transform it.

### Useful `Option` Patterns

Use `if let` when you only care about the present case:

```rust
let maybe_name = Some("Ada");

if let Some(name) = maybe_name {
    println!("hello, {}", name);
}
```

Use `unwrap_or` to provide a default:

```rust
let maybe_score: Option<i32> = None;
let score = maybe_score.unwrap_or(0);

println!("{}", score); // 0
```

Use `map` to transform the value only if it exists:

```rust
let maybe_name = Some("ada");
let uppercase = maybe_name.map(|name| name.to_uppercase());

println!("{:?}", uppercase); // Some("ADA")
```

The `|name| ...` syntax is a closure. We will study closures more deeply later, but for now you can read it as a small inline function.

---

## 3. `Result<T, E>`: Success or Error

`Result<T, E>` represents an operation that can either succeed with a value of type `T` or fail with an error of type `E`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Parsing a string into a number returns a `Result`:

```rust
fn main() {
    let input = "42";
    let parsed = input.parse::<i32>();

    match parsed {
        Ok(number) => println!("number: {}", number),
        Err(error) => println!("could not parse input: {}", error),
    }
}
```

The type is:

```rust
Result<i32, std::num::ParseIntError>
```

This means:

- success: an `i32`
- failure: a `ParseIntError`

Notice what Rust has done: the function signature tells you the operation may fail, and the compiler forces you to decide what to do.

---

## 4. Returning `Result` from Your Own Functions

Suppose you want to parse an age:

```rust
fn parse_age(input: &str) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(age) => Ok(age),
        Err(_) => Err(format!("'{}' is not a valid age", input)),
    }
}
```

Now the caller must handle both paths:

```rust
fn main() {
    match parse_age("34") {
        Ok(age) => println!("age: {}", age),
        Err(message) => println!("error: {}", message),
    }
}
```

Why return `String` as the error type? Because it is simple for learning. In larger Rust programs, error types often become enums so callers can distinguish different failure cases.

```rust
enum AgeError {
    Empty,
    InvalidNumber,
    TooLarge,
}
```

That is the same design instinct you practiced with enums: make possible states explicit.

---

## 5. Propagating Errors with `?`

Handling every error with a full `match` can get noisy. The `?` operator means:

- if the result is `Ok(value)`, unwrap it and continue
- if the result is `Err(error)`, return that error from the current function

Here is a verbose version:

```rust
fn parse_and_double(input: &str) -> Result<i32, std::num::ParseIntError> {
    let number = match input.parse::<i32>() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(number * 2)
}
```

Here is the same function with `?`:

```rust
fn parse_and_double(input: &str) -> Result<i32, std::num::ParseIntError> {
    let number = input.parse::<i32>()?;

    Ok(number * 2)
}
```

This is one of Rust's most important ergonomics features. It keeps error handling explicit in the type system while avoiding repetitive boilerplate.

### `?` Needs a Compatible Return Type

This does not compile:

```rust
fn parse_and_print(input: &str) {
    let number = input.parse::<i32>()?;
    println!("{}", number);
}
```

The function returns `()`, but `?` needs somewhere to return the error. Fix it by returning `Result`:

```rust
fn parse_and_print(input: &str) -> Result<(), std::num::ParseIntError> {
    let number = input.parse::<i32>()?;
    println!("{}", number);

    Ok(())
}
```

`Result<(), E>` means "success has no meaningful value, but failure has error information."

---

## 6. `main` Can Return `Result`

Rust lets `main` return `Result`, which is useful for small command-line programs:

```rust
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("notes.txt")?;
    println!("{}", contents);

    Ok(())
}
```

If reading the file fails, `?` returns the error from `main`, and Rust prints it before exiting with a failure status.

This will matter when we build a CLI project later. CLI programs deal with files, command-line arguments, parsing, environment variables, and user input. All of those are error-prone in normal, expected ways.

---

## 7. Choosing Between `Option`, `Result`, and Panic

Use `Option<T>` when absence is normal and no extra explanation is needed:

```rust
fn first_score(scores: &[i32]) -> Option<i32> {
    scores.first().copied()
}
```

Use `Result<T, E>` when failure is normal and the caller needs to know why:

```rust
fn parse_score(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}
```

Use panic when the program has hit a bug or invalid invariant:

```rust
fn percentage(score: i32, max: i32) -> i32 {
    assert!(max > 0, "max score must be positive");
    score * 100 / max
}
```

The line between these choices is a design decision:

- user typed bad input: usually `Result`
- lookup found nothing: usually `Option`
- programmer violated a requirement: often panic or `assert!`

---

## 8. Fixing the Lesson 9 `unwrap`

Your current `highest_score` function uses:

```rust
fn highest_score(scores: &[i64]) -> i64 {
    *scores.iter().max().unwrap()
}
```

This panics if `scores` is empty. A more honest signature returns `Option<i64>`:

```rust
fn highest_score(scores: &[i64]) -> Option<i64> {
    scores.iter().max().copied()
}
```

Now the caller handles both cases:

```rust
match highest_score(&scores) {
    Some(score) => println!("Highest score: {}", score),
    None => println!("No scores available"),
}
```

This is a perfect example of Rust's style: make the edge case visible at the boundary of the function.

---

## Key Takeaways

1. Rust separates unrecoverable bugs from recoverable errors.
2. `panic!`, `unwrap`, and `expect` stop the program when failure happens.
3. `Option<T>` represents a value that may be absent.
4. `Result<T, E>` represents success or failure with error information.
5. `match` and `if let` are basic tools for handling `Option` and `Result`.
6. The `?` operator propagates errors while keeping function signatures explicit.
7. Prefer returning `Option` or `Result` when failure is part of normal program behavior.

---

## Exercises

Open [main.rs](../src/main.rs) and implement the following exercises.

### Exercise 10-1: Safe Highest Score

Change your `highest_score` function from Lesson 9 so it returns:

```rust
fn highest_score(scores: &[i64]) -> Option<i64>
```

Test it with:

```rust
let scores = vec![92, 81, 77, 100, 68];
let empty_scores: Vec<i64> = vec![];
```

Print a helpful message for both `Some(score)` and `None`.

### Exercise 10-2: Parse a Score

Write a function:

```rust
fn parse_score(input: &str) -> Result<i32, std::num::ParseIntError>
```

It should parse the string into an `i32` and return the original parse result.

Test it with:

- `"95"`
- `"not a score"`

Use `match` in `main` to print either the parsed score or the error.

### Exercise 10-3: Parse and Validate a Percentage

Write a function:

```rust
fn parse_percentage(input: &str) -> Result<u8, String>
```

Rules:

- parse the input as a number
- return an error if parsing fails
- return an error if the number is greater than `100`
- return `Ok(number)` if it is between `0` and `100`

Test it with:

- `"85"`
- `"101"`
- `"abc"`

Hint: parse as `u8` first if you want `"101"` to parse successfully, or parse as `u16` if you want to clearly separate "valid number" from "out of range percentage."

### Exercise 10-4: Use `?`

Write a function:

```rust
fn add_scores(left: &str, right: &str) -> Result<i32, std::num::ParseIntError>
```

It should parse both inputs and return their sum. Use the `?` operator instead of `match`.

Test it with:

- `"40", "2"`
- `"40", "oops"`

### Exercise 10-5: File Reading Experiment

Create a small text file in the project root named `notes.txt`. Then write:

```rust
use std::fs;

fn read_notes() -> Result<String, std::io::Error> {
    fs::read_to_string("notes.txt")
}
```

Call it from `main` and handle both success and failure with `match`.

After it works, temporarily rename or delete `notes.txt` and run the program again to see the error path. Restore the file afterward if you want to keep the success case.

---

## Reference

- [The Rust Book - Ch. 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
