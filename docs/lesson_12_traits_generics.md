Lesson 12 — Traits & Generics

Overview

This lesson explores two of Rust's most powerful abstraction mechanisms: traits (behavioral interfaces) and generics (type parameters). Together they let you write reusable, type-safe, zero-cost abstractions.

Goals
- Understand what traits are and when to use them
- Use generics for functions, structs, enums, and impl blocks
- Apply trait bounds and where-clauses to constrain generics
- Learn associated types and trait objects for dynamic dispatch
- Practice by solving progressively harder exercises

Traits (detailed)

What is a trait?
- A trait is a collection of methods that types can implement. Think of it as an interface describing behavior rather than data.
- Traits can provide default method implementations.

When to use traits
- When you need polymorphism across unrelated types that share behavior.
- To express abstractions: "anything that can be summarized" or "anything that can be drawn".

Example — basic trait with default method

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more)")
    }
}

struct Article { headline: String, content: String }
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}

struct Tweet { username: String, content: String }
impl Summary for Tweet {} // uses default
```

Associated types
- Traits can include associated types: placeholders that an implementer specifies. This is often more readable than large generic bounds.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Trait objects & dynamic dispatch
- Use Box<dyn Trait>, &dyn Trait to store values of different concrete types behind a pointer and call methods via dynamic dispatch.
- Use when runtime polymorphism is required; it incurs indirection and (usually) one virtual call per method.

Generics (detailed)

Why generics?
- Avoid code duplication while keeping compile-time type safety and performance.
- The compiler monomorphizes generics — it generates specialized code for each concrete used type, leading to performance similar to handwritten code.

Generic functions

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest { largest = item }
    }
    largest
}
```

Generic structs & enums

```rust
struct Pair<T> { a: T, b: T }
impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self { Pair { a, b } }
}

impl<T: PartialOrd + std::fmt::Display> Pair<T> {
    fn cmp_display(&self) { /* ... */ }
}
```

Trait bounds — concise and readable

- Use T: Trait to require a trait implementation
- When bounds get complex, prefer a where-clause

```rust
fn notify<T: Summary + Display>(item: &T) { }

fn complicated<T, U>(t: &T, u: &U)
where
    T: Iterator<Item = U>,
    U: Display,
{
}
```

Specialization and coherence
- Rust does not support ad-hoc specialization by default. Trait implementations must be unambiguous and obey coherence rules.

Examples and patterns

1) Returning types that implement a trait (impl Trait)

```rust
fn make_summary() -> impl Summary {
    Article { headline: String::from("x"), content: String::from("y") }
}
```

2) Using trait objects where appropriate

```rust
fn notify_dyn(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

Exercises (detailed)

Exercise 12-1 (easy) — Summary trait
- Create a `Summary` trait with a default `summarize` method that returns a String.
- Implement Summary for two types: `Article` (override summarize) and `Tweet` (use default).
- Write tests or a small `main()` that demonstrates both behaviors.

Hints: Keep the trait small; prefer &self receivers.

Exercise 12-2 (easy-medium) — Generic largest
- Implement `fn largest<T: PartialOrd + Copy>(items: &[T]) -> T` that returns the largest item.
- Explain why `Copy` is used here and how to avoid it (hint: return reference instead).

Exercise 12-3 (medium) — Pair<T>
- Implement a generic struct `Pair<T>` with a method `cmp_display` that prints the larger value.
- Use where-clauses for readability: `impl<T> Pair<T> where T: PartialOrd + Display`.

Exercise 12-4 (medium-hard) — Associated types
- Write a trait `Container` with an associated type `Item` and methods to add and remove items.
- Implement `Container` for a simple `struct Bucket<T> { items: Vec<T> }`.
- Discuss tradeoffs between associated types vs generic parameters on the trait.

Exercise 12-5 (advanced) — Trait objects and lifetimes
- Write a function that accepts a slice of boxed trait objects `&[Box<dyn Summary>]` and prints summaries.
- Extend to accept references `&[&dyn Summary]` and explain lifetime considerations.

Exercise 12-6 (challenge) — Generic bounds & where-clauses
- Create a function that takes two generic types and uses complex trait bounds (e.g., T: Iterator<Item = U>, U: Display) and demonstrate it with real types.

Solutions & references
- Short solutions are available in `src/answers/lesson_12_traits_generics.rs`.
- Read "Traits" and "Generics" chapters in The Rust Programming Language for deep coverage.

Key takeaways
- Traits express behavior; generics express parametrized code.
- Use trait bounds and where-clauses to keep APIs ergonomic and readable.
- Prefer static dispatch (generics/impl Trait) for performance; use trait objects for runtime polymorphism when necessary.

Further reading
- https://doc.rust-lang.org/book/ch10-02-traits.html
- https://doc.rust-lang.org/book/ch10-00-generics.html

