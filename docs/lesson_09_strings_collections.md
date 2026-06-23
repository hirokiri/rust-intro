# Lesson 9: Strings & Collections (`Vec`, `HashMap`)

So far, most of our values have been individual values: numbers, booleans, structs, enums, and references. Real programs also need growable data: user names, command arguments, lists of records, word counts, lookup tables, and parsed input.

This lesson covers three of Rust's most common standard-library data structures:

- `String`: owned, growable UTF-8 text
- `Vec<T>`: owned, growable list of values of the same type
- `HashMap<K, V>`: owned key-value lookup table

The important Rust idea is that collections are not magical containers outside the ownership system. They own their contents unless you deliberately store references.

---

## 1. `String` vs `&str`

Rust has two string types you will see constantly:

- `String`: an owned, growable string stored on the heap
- `&str`: a borrowed string slice, often pointing into a `String` or a string literal

```rust
let literal: &str = "hello";              // borrowed, fixed text
let mut owned = String::from("hello");    // owned, growable text

owned.push('!');
owned.push_str(" welcome");

println!("{}", literal);
println!("{}", owned);
```

You can think of `String` as similar to a Python `str` value you own and can build up, while `&str` is closer to a view into existing text. The comparison is not perfect because Python strings are immutable objects managed by a garbage collector, while Rust makes ownership and borrowing explicit.

### Why Two Types?

Rust separates ownership from borrowing:

```rust
fn print_label(label: &str) {
    println!("Label: {}", label);
}

let name = String::from("Ferris");

print_label("static text"); // string literal works
print_label(&name);         // String can be borrowed as &str
```

Taking `&str` is flexible because callers can pass string literals, borrowed `String`s, or slices of larger strings. This is a common API design habit in Rust: accept borrowed data when you only need to read it.

### Moving a `String`

Because `String` owns heap memory, assigning it to another variable moves ownership:

```rust
let first = String::from("rust");
let second = first;

// println!("{}", first); // Compile error: first was moved
println!("{}", second);
```

If you only need to inspect the text, borrow it:

```rust
let first = String::from("rust");
let second = &first;

println!("{}", first);
println!("{}", second);
```

This is the same ownership rule you learned earlier, now applied to text.

---

## 2. Updating Strings

Use `push` for one character and `push_str` for a string slice:

```rust
let mut message = String::from("Hello");

message.push(',');
message.push_str(" Rust!");

println!("{}", message);
```

You can also concatenate strings:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2;

// println!("{}", s1); // Compile error: s1 was moved by +
println!("{}", s2);    // s2 was only borrowed
println!("{}", s3);
```

The `+` operator takes ownership of the left-hand `String` and borrows the right-hand string. That can feel odd at first, but it avoids unnecessary allocation when possible.

For clearer formatting, prefer `format!`:

```rust
let language = "Rust";
let year = 2015;

let sentence = format!("{} reached 1.0 in {}", language, year);
println!("{}", sentence);
```

`format!` works like `println!`, but returns a `String` instead of printing.

---

## 3. String Indexing and UTF-8

Rust strings are UTF-8. That means one visible character may take more than one byte:

```rust
let hello = String::from("Здравствуйте");
```

In Rust, this does not compile:

```rust
let s = String::from("hello");
// let first = s[0]; // Compile error
```

Rust refuses direct string indexing because `s[0]` is ambiguous:

- Should it return a byte?
- Should it return a Unicode scalar value?
- Should it return a user-perceived character, which can be multiple Unicode scalars?

Instead, choose what you mean:

```rust
let word = "hello";

for byte in word.bytes() {
    println!("byte: {}", byte);
}

for ch in word.chars() {
    println!("char: {}", ch);
}
```

You can slice strings by byte range, but only on valid UTF-8 boundaries:

```rust
let s = "hello";
let slice = &s[0..2];

println!("{}", slice); // he
```

Be careful with non-ASCII text. If the byte range cuts through a multi-byte character, the program will panic at runtime.

---

## 4. `Vec<T>`: Growable Lists

A vector stores multiple values of the same type in insertion order:

```rust
let mut numbers: Vec<i32> = Vec::new();

numbers.push(10);
numbers.push(20);
numbers.push(30);

println!("{:?}", numbers);
```

More commonly, use the `vec!` macro:

```rust
let scores = vec![90, 85, 100];
```

The type is usually inferred, but every element must have the same type:

```rust
let names = vec![
    String::from("Ada"),
    String::from("Grace"),
    String::from("Linus"),
];
```

If you need mixed types, reach for an enum:

```rust
enum Cell {
    Int(i32),
    Text(String),
    Bool(bool),
}

let row = vec![
    Cell::Int(42),
    Cell::Text(String::from("answer")),
    Cell::Bool(true),
];
```

This is Rust's type system helping you model the possible shapes explicitly.

### Reading Values

There are two common ways to access vector elements:

```rust
let scores = vec![90, 85, 100];

let first = scores[0];
let maybe_fourth = scores.get(3);

println!("first: {}", first);
println!("fourth: {:?}", maybe_fourth);
```

Indexing with `[]` panics if the index is out of bounds. `get` returns `Option<&T>`, forcing you to handle the missing case:

```rust
match scores.get(3) {
    Some(score) => println!("score: {}", score),
    None => println!("no score at that index"),
}
```

This is the same safety pattern as Lesson 8: possible absence is represented in the type.

### Iterating Over Vectors

Borrow each value when you want to read without taking ownership:

```rust
let names = vec![
    String::from("Ada"),
    String::from("Grace"),
    String::from("Linus"),
];

for name in &names {
    println!("{}", name);
}

println!("still available: {:?}", names);
```

Use mutable references when you want to update in place:

```rust
let mut numbers = vec![1, 2, 3];

for number in &mut numbers {
    *number += 10;
}

println!("{:?}", numbers); // [11, 12, 13]
```

The `*number` syntax dereferences the mutable reference so you can change the value it points to.

### Borrowing Rules with Vectors

This will not compile:

```rust
let mut numbers = vec![1, 2, 3];
let first = &numbers[0];

numbers.push(4);

println!("{}", first);
```

Why so strict? `Vec<T>` may need to move its elements to a larger memory allocation when you push. If Rust allowed `first` to keep pointing into the old allocation, it could become a dangling reference. The borrow checker prevents that before the program runs.

---

## 5. `HashMap<K, V>`: Key-Value Lookup

`HashMap` is not imported by the prelude, so bring it into scope:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

println!("{:?}", scores);
```

Hash maps are useful when you want to look up a value by key instead of by numeric index.

### Ownership in Hash Maps

Inserting owned values moves them into the map:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// println!("{}", field_name); // Compile error: moved into map
```

If you insert references, the map does not own the data, and the referenced data must live long enough. For now, prefer owned keys and values unless you have a clear reason to borrow.

### Reading Values

`get` returns an `Option<&V>`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

let team = String::from("Blue");
let score = scores.get(&team);

match score {
    Some(value) => println!("score: {}", value),
    None => println!("team not found"),
}
```

Again, Rust uses `Option` to make missing data explicit.

### Updating Values

Calling `insert` with an existing key replaces the old value:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // {"Blue": 25}
```

Use `entry` when you only want to insert if the key does not exist:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(99);

println!("{:?}", scores); // Blue stays 10
```

`entry` returns a special enum-like value that represents either an occupied or vacant entry. `or_insert` gives you a mutable reference to the value in the map.

### Counting Words

A classic `HashMap` example is counting occurrences:

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut counts = HashMap::new();

for word in text.split_whitespace() {
    let count = counts.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", counts);
```

Here, the keys are `&str` slices borrowed from `text`. This is okay because `text` lives for the whole loop and the map is used while `text` is still valid. If the map needed to outlive the source text, use owned `String` keys instead:

```rust
for word in text.split_whitespace() {
    let count = counts.entry(word.to_string()).or_insert(0);
    *count += 1;
}
```

---

## How This Compares to Other Languages

### Python

Python's closest equivalents are:

- `str` for text
- `list` for `Vec<T>`
- `dict` for `HashMap<K, V>`

The big difference is that Python containers can freely mix types at runtime:

```python
items = [1, "two", True]
```

Rust vectors require one element type. If you want mixed shapes, model them with an enum. That extra ceremony buys compile-time guarantees: once your program compiles, you know every element is handled according to its declared shape.

### Go

Go has slices and maps:

```go
scores := []int{90, 85, 100}
counts := map[string]int{"rust": 1}
```

Go slices and Rust vectors are conceptually close, but Go relies on garbage collection and runtime checks for many memory concerns. Rust's borrow checker prevents references into a vector from surviving across mutations that might reallocate.

Go strings are byte sequences that conventionally contain UTF-8. Rust strings are guaranteed valid UTF-8, and Rust makes you choose between bytes and `chars` when iterating.

### TypeScript

TypeScript has `string`, arrays, objects, `Map`, and `Record`:

```typescript
const scores: number[] = [90, 85, 100];
const counts: Record<string, number> = { rust: 1 };
```

TypeScript checks types at compile time, but JavaScript arrays and objects are dynamic at runtime. Rust's `Vec<T>` and `HashMap<K, V>` are strongly typed all the way down to the compiled program.

---

## Key Takeaways

1. `String` owns growable UTF-8 text; `&str` borrows text.
2. Prefer function parameters like `&str` when you only need to read string data.
3. Rust does not allow direct string indexing because UTF-8 makes "character at index" ambiguous.
4. `Vec<T>` stores a growable list of one element type.
5. Vector indexing with `[]` can panic; `.get()` returns `Option<&T>`.
6. `HashMap<K, V>` stores key-value pairs and moves owned keys/values into the map.
7. The `entry(...).or_insert(...)` pattern is the standard way to initialize or update map values.

---

## Exercises

Open [main.rs](../src/main.rs) and implement the following exercises.

### Exercise 9-1: Name Formatter

Write a function:

```rust
fn format_name(first: &str, last: &str) -> String
```

It should return a string in `"Last, First"` format.

Test it with:

- `"Ada", "Lovelace"`
- `"Grace", "Hopper"`

Try implementing it once with `format!` and once by building a mutable `String` with `push_str`.

### Exercise 9-2: Score Summary with `Vec`

Create a vector of integer scores:

```rust
let scores = vec![92, 81, 77, 100, 68];
```

Write code that prints:

- each score
- the total score
- the average score as an `f64`
- the highest score

Use iteration instead of hard-coding indexes.

### Exercise 9-3: Safe Index Lookup

Write a function:

```rust
fn describe_score(scores: &Vec<i32>, index: usize) -> String
```

If a score exists at that index, return `"Score at index N is X"`.

If not, return `"No score at index N"`.

Use `.get()` and `match`, not direct indexing.

Bonus: change the parameter from `&Vec<i32>` to `&[i32]`. A slice is more flexible because callers can pass a vector, an array, or part of a vector.

### Exercise 9-4: Word Counter with `HashMap`

Write a function:

```rust
fn word_counts(text: &str) -> std::collections::HashMap<String, u32>
```

It should count how many times each whitespace-separated word appears.

Test it with:

```rust
let text = "rust is fast rust is safe rust is fun";
```

Expected counts:

- `"rust"` appears `3`
- `"is"` appears `3`
- `"fast"`, `"safe"`, and `"fun"` each appear `1`

### Exercise 9-5: Inventory Updates

Create a `HashMap<String, i32>` representing inventory counts:

- `"apples"` -> `3`
- `"bananas"` -> `2`

Then:

- add `5` more apples
- add `"oranges"` with count `4` if it does not already exist
- try to insert `"bananas"` with count `99` only if bananas are missing

Print the final map. Use the `entry(...).or_insert(...)` pattern for at least one of the updates.

---

## Reference

- [The Rust Book - Ch. 8: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
