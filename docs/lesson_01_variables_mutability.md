# Lesson 1: Variables & Mutability 🔒

## Core Concept: Immutability by Default

In Rust, variables are **immutable by default**. This is a deliberate design choice:
- Easier to reason about code
- Compiler can optimize more aggressively
- Immutable data is inherently thread-safe — no locks needed

```rust
let x = 5;
x = 6; // ❌ ERROR: cannot assign twice to immutable variable
```

Use `mut` to explicitly opt in to mutability:

```rust
let mut x = 5;
x = 6; // ✅ Works
```

---

## 3 Ways to Bind Values

### `let` — Immutable by default
```rust
let x = 5;    // can't reassign
```

### `let mut` — Opt-in mutability
```rust
let mut x = 5;
x = 6;        // ✅ allowed, but type must stay the same
```

### `const` — Compile-time constant
```rust
const MAX_POINTS: u32 = 100_000;
```

| Feature | `let` | `let mut` | `const` |
|---|---|---|---|
| Reassignable? | ❌ | ✅ | ❌ |
| Type annotation required? | Optional | Optional | **Required** |
| Can shadow? | ✅ | ✅ | ❌ |
| Evaluated at | Runtime | Runtime | **Compile time** |
| Naming convention | `snake_case` | `snake_case` | `SCREAMING_SNAKE_CASE` |

> **Tip:** `100_000` — Rust lets you use `_` as a visual separator in numbers. `100_000` = `100000`.

---

## Shadowing

Shadowing means re-declaring a variable with `let`. It creates a **brand new** variable with the same name:

```rust
let z = 5;
let z = z + 1;   // new z = 6
let z = z * 2;   // new z = 12
```

### Shadowing Can Change Types!

```rust
let spaces = "   ";          // &str (string slice)
let spaces = spaces.len();   // usize (integer)
```

This is NOT possible with `mut`:

```rust
let mut spaces = "   ";
spaces = spaces.len(); // ❌ ERROR: expected `&str`, found `usize`
```

### Shadowing vs `mut`

| | Shadowing (`let`) | Mutation (`mut`) |
|---|---|---|
| Creates new variable? | ✅ Yes | ❌ No |
| Can change type? | ✅ Yes | ❌ No |
| Previous value? | Dropped | Overwritten |

### Practical Use: Parsing Pipelines

Shadowing is great for transforming values through stages:

```rust
let input = "   42   ";                    // &str
let input = input.trim();                  // &str (trimmed)
let input: u32 = input.parse().unwrap();   // u32
```

---

## Comparison with Other Languages

| Concept | Python | Go | TypeScript | Rust |
|---|---|---|---|---|
| Immutable binding | ❌ (convention) | ❌ | `const` (shallow) | `let` (default!) |
| Mutable binding | Everything | Everything | `let` | `let mut` |
| Shadowing | ❌ | `:=` in inner scope | ❌ | `let` re-declaration |
| True constant | ❌ | `const` | `const` (kinda) | `const` |

### Python
Variables are always mutable. Immutability is only by convention (e.g., `UPPER_CASE` names).

### Go
Variables are always reassignable. `const` exists but only for compile-time values.

### TypeScript
`const` prevents reassignment but not deep mutation (`const arr = []; arr.push(1)` works). Rust's `let` is truly immutable.

---

## Key Takeaways

1. **`let` is immutable by default** — Rust makes safety the default.
2. **`let mut` opts in to mutability** — mutation is an explicit, visible choice.
3. **Shadowing** creates a new variable — it can even change types.
4. **`const`** is for compile-time constants and requires a type annotation.
5. **`_` in numbers** is a visual separator: `1_000_000` = `1000000`.

---

## Exercises

### Exercise 1-1: Immutability Error
Write code that declares an immutable variable and tries to reassign it. Run `cargo run` and **read the compiler error carefully**. What does the compiler suggest?

### Exercise 1-2: Temperature Converter
Write a program using `mut` that:
1. Starts with a temperature in Celsius (e.g., `let mut temp = 100.0;`)
2. Prints the Celsius value
3. Converts it to Fahrenheit (`F = C * 9/5 + 32`) and reassigns it to the same variable
4. Prints the Fahrenheit value

### Exercise 1-3: Shadowing Chain
Use shadowing to transform a value through 3 stages:

```rust
let data = "  42  ";       // Step 1: a string with spaces
let data = ???;             // Step 2: trim the spaces (use .trim())
let data: i32 = ???;        // Step 3: parse to an integer (use .parse().unwrap())
println!("Parsed: {}", data);  // Should print: Parsed: 42
```

### Exercise 1-4: Shadowing vs Mut
Predict the output of this code **without running it**, then verify:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner x = {}", x);
    }
    println!("outer x = {}", x);
}
```

---

## Reference

- [The Rust Book — Ch.3.1: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust by Example — Variable Bindings](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings.html)

