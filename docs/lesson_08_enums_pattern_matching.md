# Lesson 8: Enums & Pattern Matching

In Lesson 7, we learned how **Structs** let you group related fields together. In this lesson, we will explore **Enums (Enumerations)**. 

If you are coming from Python, TypeScript, or Go, you might think of enums as just lists of named constants (like `RED = 1`, `BLUE = 2`). In Rust, enums are far more powerful: they are **Algebraic Data Types (ADTs)**, which means they can hold data inside their variants, and they work hand-in-hand with Rust's pattern matching to provide compile-time safety.

---

## 1. Defining an Enum

An enum gives you a way of saying a value is one of a possible set of values. For example, we might want to express IP addresses:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

You can instantiate variants using double colons `::`:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### Attaching Data to Variants

In other languages, to associate data with an enum, you might need a separate class or struct. In Rust, you can put data **directly** inside the enum variants:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Each variant can even have **different types and amounts of data**:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // Tuple-like variant
    V6(String),          // Single string variant
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

This flexibility allows you to model complex, mutually exclusive domain states cleanly:

```rust
enum Message {
    Quit,                         // No data
    Move { x: i32, y: i32 },      // Anonymous struct
    Write(String),                // Single String
    ChangeColor(i32, i32, i32),   // Three i32s
}
```

---

## 2. Implementing Methods on Enums

Just like structs, you can define methods on enums using an `impl` block:

```rust
impl Message {
    fn call(&self) {
        // Method body details
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

Inside the method, you can use `self` to read the variant and its associated data.

---

## 3. The `Option` Enum (Rust's Solution to `null`)

Many languages have a `null` or `nil` value, representing the absence of a value. In 2009, Tony Hoare (the inventor of `null`) called it his **"billion-dollar mistake"** because trying to use a `null` value as a non-null value leads to runtime crashes (e.g., `NullPointerException`, `AttributeError: 'NoneType' object has no attribute...`, `Cannot read properties of null`).

Rust **does not have null**. Instead, the standard library defines an enum called `Option<T>`:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `<T>` is a **generic type parameter** (which we'll cover in Lesson 12). It means the `Some` variant can hold a value of any type `T`.

Because `Option<T>` is so common, its variants `Some` and `None` are imported by default. You don't need to write `Option::Some` or `Option::None`:

```rust
let some_number = Some(5);
let some_char = Some('e');
let absent_number: Option<i32> = None;
```

### Why is this safer?
In Rust, an `i32` and an `Option<i32>` are **completely different types**:

```rust
let x: i32 = 5;
let y: Option<i32> = Some(5);

let sum = x + y; // ❌ Compile Error! Cannot add i32 and Option<i32>
```

To use the value inside `y`, you are **forced** to handle the case where it might be `None`. You cannot accidentally use a missing value as if it were valid!

---

## 4. Pattern Matching with `match`

To extract values from an enum and run different code based on the variant, Rust uses the powerful `match` control flow construct:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Binding to Values

If a variant has associated data, you can bind to that data in the match pattern to extract it:

```rust
#[derive(Debug)]
enum UsState {
    Alaska,
    California,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Associated State data
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### Matches Are Exhaustive

In Rust, **matches must cover every single possibility**. If you forget a variant, the code will not compile:

```rust
// ❌ Compile Error! Pattern 'None' not covered
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

This ensures you can never forget to handle the `None` case or a new enum variant you add later.

### Catch-All Patterns (`_` and `other`)

If you don't want to list every variant, you can use a catch-all pattern:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // Binds the value to `other`
}

// Or if you want to ignore the value:
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // `_` matches anything, `()` is the unit value (do nothing)
}
```

---

## 5. Concise Control Flow with `if let`

If you only care about **one** variant and want to ignore the rest, `match` can feel verbose:

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

You can write this more concisely using `if let`:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

You can also add an `else` block:

```rust
if let Some(max) = config_max {
    println!("Max is {}", max);
} else {
    println!("No max configured.");
}
```

*Note:* `if let` sacrifices compile-time exhaustiveness checking for brevity. Use it when you intentionally only care about one outcome.

---

## How This Compares to Other Languages

### Python
Python 3.10 introduced **Structural Pattern Matching** (`match`/`case`), which is syntactically very similar to Rust's `match`:

```python
# Python 3.10+
match command.split():
    case ["quit"]:
        quit()
    case ["move", x, y]:
        move(x, y)
```

However:
- Python does not enforce exhaustiveness at compile-time. If no case matches, it simply falls through.
- Python enums (`enum.Enum`) are simple constants and do not easily hold arbitrary variant-specific typed structures.

### Go
Go does not have enums. Instead, Go developers use constants grouped with `iota`:

```go
type Command int
const (
    Quit Command = iota
    Move
)
```

To mimic associated data, you must use interfaces, structs, and runtime type assertions (`switch cmd := val.(type)`). Go's compiler cannot check if you handled every type variant in a switch, making runtime bugs more likely if you add new types.

### TypeScript
TypeScript uses **Discriminated Unions** (sometimes called tagged unions) to achieve something very similar to Rust's enums:

```typescript
type Message =
  | { type: 'quit' }
  | { type: 'move'; x: number; y: number }
  | { type: 'write'; text: string };

function handle(msg: Message) {
  switch (msg.type) {
    case 'quit': ...
    case 'move': ...
  }
}
```

TypeScript discriminated unions are highly expressive, but the safety is verified at the compiler level and compiles down to basic JavaScript objects. Rust enums are natively optimized at the binary level (using a "tag" byte to identify the variant in memory) with zero garbage collection.

---

## Key Takeaways

1. **Enums** allow you to define a type by enumerating its possible variants.
2. Variants can contain **associated data** (tuples, structs, or simple types), making them extremely expressive.
3. Rust has no `null`. Instead, it uses **`Option<T>`** (`Some(T)` or `None`) to represent the potential absence of a value.
4. **`match`** expressions are compile-time verified to be **exhaustive**; you must handle every variant.
5. **`if let`** is a concise shorthand when you only care about matching one variant.

---

## Exercises

Let's put this into practice! Open [main.rs](src/main.rs) and implement the following exercises.

### Exercise 8-1: Traffic Light Enum
Define an enum `TrafficLight` with variants: `Red`, `Yellow`, `Green`.
Implement a method `duration(&self) -> u32` on `TrafficLight` that returns:
- `30` for `Red`
- `5` for `Yellow`
- `45` for `Green`

In `main`, instantiate each light and print its duration.

### Exercise 8-2: Web Event Handler
Define an enum `WebEvent` with these variants:
- `PageLoad`
- `PageUnload`
- `KeyPress(char)`
- `Paste(String)`
- `Click { x: i64, y: i64 }`

Implement a function `inspect_event(event: WebEvent)` that uses a `match` expression to print a description of the event. For example, if it's a `Click`, print: `"Click event at x = 10, y = 20"`. If it's a `KeyPress`, print: `"Key pressed: 'a'"`.

In `main`, instantiate at least three different variants of `WebEvent` and pass them to `inspect_event`.

### Exercise 8-3: Option Math
Write a function `plus_one(x: Option<i32>) -> Option<i32>` that uses a `match` expression. If `x` is `Some(i)`, it returns `Some(i + 1)`. If `x` is `None`, it returns `None`.

Test it in `main` with:
- `Some(5)`
- `None`

### Exercise 8-4: Config Parser
Write a function `get_port(env_var: Option<&str>) -> u16`.
- If `env_var` is `Some(val)`, try to parse it into a `u16` using `val.parse::<u16>()`.
- If parsing succeeds, return that `u16` value.
- If `env_var` is `None` OR if the parsing fails (e.g., `"abc"` cannot be parsed to `u16`), return a default port of `8080`.
- *Hint:* You can chain matching or use `match` on the result of `val.parse()`, which returns a `Result` enum (`Ok(num)` or `Err(_)`).

Test it in `main` with:
- `Some("8000")`
- `Some("invalid")`
- `None`

---

## Reference
- [The Rust Book - Ch. 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
