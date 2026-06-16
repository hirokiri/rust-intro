# Lesson 2: Data Types & Functions 🧮

## Scalar Types

Rust has 4 scalar (single-value) types:

### Integers

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` (default) | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

- **Default integer type is `i32`** — fastest on most CPUs.
- **`usize`** is used for indexing into collections (like array indices). It matches your CPU's pointer size (64-bit on modern systems).
- Use `_` as a visual separator: `1_000_000` = `1000000`.

### Floats

- `f64` (default) — 64-bit, double precision
- `f32` — 32-bit, single precision

### Boolean

- `bool` — `true` or `false`

### Character

- `char` — **4 bytes**, full Unicode scalar value
- Single quotes: `'A'`, `'🦀'`, `'漢'`

---

## Compound Types

### Tuples — Fixed size, mixed types

```rust
let person: (&str, u32, bool) = ("Alice", 30, true);

// Access by index (dot notation!)
person.0  // "Alice"
person.1  // 30

// Destructuring
let (name, age, active) = person;
```

**Comparisons:**
- **Python:** Similar to Python tuples, but typed.
- **Go:** No built-in tuples — Go uses structs or multiple return values.
- **TypeScript:** `[string, number, boolean]` — same concept.

### Arrays — Fixed size, same type

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // [type; length]
let zeros = [0; 3];                         // [0, 0, 0]
```

- **Fixed size at compile time** (unlike `Vec<T>`, which is Rust's growable list — covered later).
- **Bounds-checked at runtime** — accessing out-of-bounds panics (crashes safely), unlike C/C++ which silently reads garbage memory.

> **Note:** Use `{:?}` (Debug format) to print arrays and tuples: `println!("{:?}", numbers);`

---

## Functions

### Syntax

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
}
```

### Key Rules

1. **Parameter types are ALWAYS required** — no inference for function signatures.
2. **Return type** is specified with `->`. Omit it for functions returning nothing (they return `()`, the unit type).
3. **`snake_case`** naming convention for functions (enforced by the compiler as a warning).

### Implicit Return (Expression-Based)

The last expression in a function (without `;`) is automatically returned:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = implicit return
}
```

Use `return` keyword only for **early returns**:

```rust
fn absolute_value(n: i32) -> i32 {
    if n < 0 {
        return -n;  // early return
    }
    n  // implicit return
}
```

### Multiple Return Values (Tuples)

```rust
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

let (quotient, remainder) = divide(17, 5);
```

This is similar to Go's multiple return values, but Rust uses tuples.

---

## Expressions vs Statements ⭐

This is one of Rust's most distinctive features.

| | Statement | Expression |
|---|---|---|
| Returns a value? | ❌ No | ✅ Yes |
| Ends with `;`? | Yes | **No** (adding `;` discards the value) |
| Example | `let x = 5;` | `x + 1`, `{ ... }`, `if ... else` |

### Block Expressions

A `{}` block is an expression — it returns its last value:

```rust
let y = {
    let x = 3;
    x + 1     // ← No semicolon = returns 4
};
// y = 4
```

### The Semicolon Trap

```rust
let y = {
    let x = 3;
    x + 1;    // ← Semicolon! Returns () instead of 4
};
// y = ()  — probably NOT what you wanted
```

### `if` as an Expression

```rust
let status = if age >= 18 { "adult" } else { "minor" };
```

This is like Python's ternary `"adult" if age >= 18 else "minor"` or JS's `age >= 18 ? "adult" : "minor"`.

---

## Comparison with Other Languages

| Feature | Python | Go | TypeScript | Rust |
|---|---|---|---|---|
| Type system | Dynamic | Static | Static (with `any`) | Static, inferred |
| Integer types | Unlimited precision | `int`, `int32`, etc. | `number` | `i8`..`i128`, `u8`..`u128` |
| Float types | `float` (64-bit) | `float32`, `float64` | `number` | `f32`, `f64` |
| Char type | str (1-char) | `rune` (int32) | `string` | `char` (4 bytes, Unicode) |
| Tuples | ✅ Built-in | ❌ | ✅ (typed arrays) | ✅ Built-in |
| Fixed arrays | ❌ (use list) | `[5]int` | ❌ | `[i32; 5]` |
| Expression-based | Partially | ❌ | Partially | ✅ Fully |
| Implicit return | ❌ | ❌ | ✅ (arrow functions) | ✅ (all functions) |

---

## Key Takeaways

1. **Rust has specific integer sizes** (`i32`, `u8`, etc.) — no unlimited-precision integers like Python.
2. **`char` is 4 bytes** and supports full Unicode — `'🦀'` is a valid char.
3. **Tuples** are fixed-size, mixed-type containers. Arrays are fixed-size, same-type.
4. **Function parameter types are always required** — return types too (unless `()`).
5. **Expressions vs statements** is the key insight: the last expression without `;` is the return value.
6. **Semicolons change meaning** — `x + 1` returns a value, `x + 1;` does not.

---

## Exercises

### Exercise 2-1: Type Explorer
Declare one variable of each scalar type and print them all:
- An `i64` integer
- A `f32` float
- A `bool`
- A `char` (use an emoji!)

### Exercise 2-2: Tuple Swap
Write a function `swap` that takes two `i32` values and returns them in reversed order as a tuple:

```rust
fn swap(a: i32, b: i32) -> (i32, i32) {
    // your code here
}

fn main() {
    let (x, y) = swap(10, 20);
    println!("x = {}, y = {}", x, y); // Should print: x = 20, y = 10
}
```

### Exercise 2-3: Fibonacci
Write a function `fibonacci(n: u32) -> u32` that returns the n-th Fibonacci number. Use a `mut` variable and a loop:

```rust
// fibonacci(0) = 0
// fibonacci(1) = 1
// fibonacci(7) = 13
```

Hint: You'll need `for i in 2..=n { ... }` — range syntax!

### Exercise 2-4: Expression Block
Use a block expression to calculate the area of a circle and bind it to a variable, **without** calling a separate function:

```rust
let radius = 5.0_f64;
let area = {
    // calculate pi * r * r here using an expression (no semicolon at the end!)
};
println!("Area = {}", area); // Should print: Area = 78.53981633974483
```

Hint: Use `std::f64::consts::PI` or just `3.14159265358979`.

---

## Reference

- [The Rust Book — Ch.3.2: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [The Rust Book — Ch.3.3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust by Example — Primitives](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)

