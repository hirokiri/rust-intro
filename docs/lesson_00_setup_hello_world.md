# Lesson 0: Setup & "Hello, World!" 🦀

## Why Rust?

Rust exists to solve a long-standing problem in systems programming: **how do you get C/C++ speed without sacrificing memory safety?**

| Language | Memory Management | Speed | Safety |
|---|---|---|---|
| **Python/TS** | Garbage Collector (GC) | Slower | Safe (runtime crashes) |
| **Go** | Garbage Collector (GC) | Fast | Safe (runtime crashes) |
| **C/C++** | Manual (you do it) | Fastest | **Unsafe** (segfaults, bugs) |
| **Rust** | **Ownership system** (compile-time) | Fastest | **Safe** (compile-time checks) |

Rust achieves memory safety **at compile time** through its ownership system — no garbage collector, no runtime cost. The trade-off is a strict compiler, but that strictness prevents entire categories of bugs before your code ever runs.

---

## The Simplest Rust Program

```rust
fn main() {
    println!("Hello, World! 🦀");
}
```

### `fn main()` — The Entry Point

Every Rust program starts execution from `fn main()`. Comparisons:

- **Python:** `if __name__ == "__main__":`
- **Go:** `func main()`
- **TypeScript/Node:** top-level code or explicit entry

### `println!()` — It's a Macro, Not a Function

The `!` after `println` indicates it's a **macro**, not a regular function.

- `println!` can accept a variable number of arguments with format strings (like `printf` in C).
- Regular Rust functions cannot accept variadic arguments.
- Rule of thumb: **`!` = macro**.

#### Format Placeholders

```rust
let greeting = "Hello";
let name = "Rustacean";
println!("{}, {}! 🦀", greeting, name);
// Output: Hello, Rustacean! 🦀
```

`{}` are format placeholders — similar to `%s` in C/Go, or `{}` in Python's `.format()`.

### Semicolons Matter

Rust uses `;` to end statements. Unlike JavaScript, omitting a semicolon can change your code's meaning — this relates to Rust's **expression-based** design (covered later).

---

## Cargo: Rust's Build Tool & Package Manager

`cargo` is Rust's all-in-one tool, similar to `npm` (JS) or `go` (Go).

| Command | What it does |
|---|---|
| `cargo run` | Compile **and** execute |
| `cargo build` | Compile only |
| `cargo check` | Check for errors without building (fastest!) |
| `cargo test` | Run tests |
| `cargo new <name>` | Create a new project |

### Project Structure

```
rust-intro/
├── Cargo.toml    # Project manifest (like package.json or go.mod)
├── Cargo.lock    # Exact dependency versions (like package-lock.json)
└── src/
    └── main.rs   # Entry point
```

- **`Cargo.toml`** — Declares your project name, version, and dependencies.
- **`Cargo.lock`** — Auto-generated; locks exact dependency versions.
- **`src/main.rs`** — Where `fn main()` lives.

---

## Key Takeaways

1. **`fn main()`** is the entry point of every Rust program.
2. **`println!`** is a macro (note the `!`), not a regular function.
3. **`cargo`** is your best friend — use `cargo run` to build & execute.
4. **Semicolons** are required and meaningful in Rust.
5. Rust guarantees **memory safety at compile time** with zero runtime cost.

---

## Exercises

### Exercise 0-1: Personalized Greeting
Modify the `main` function to print your name and a fun fact about yourself using format placeholders `{}`.

```rust
fn main() {
    let name = "Your Name";
    let language_count = 3;
    println!("Hi, I'm {} and I know {} programming languages!", name, language_count);
}
```

### Exercise 0-2: Multiple Lines
Print a 3-line ASCII art crab using `println!`:

```
Expected output:
  🦀
 /||\
/ || \
```

### Exercise 0-3: Cargo Commands
Try running each of these commands and observe the difference:
1. `cargo check` — How fast is it compared to `cargo build`?
2. `cargo build` — Where does the compiled binary end up? (Hint: look in `target/`)
3. `cargo build --release` — What's different about this build?

---

## Reference

- [The Rust Book — Ch.1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Rust by Example — Hello World](https://doc.rust-lang.org/stable/rust-by-example/hello.html)

