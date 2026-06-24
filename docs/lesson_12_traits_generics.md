Lesson 12 — Traits & Generics

Concepts
- Traits: define shared behavior (interfaces). Use for polymorphism and abstraction.
- Generics: write code parametrized over types to avoid duplication.
- Trait bounds: constrain generic parameters to types that implement required traits.
- Where-clauses: improve readability for complex bounds.
- Associated types vs generics: associated types tie a type to a trait implementor.

Why Rust does this (brief)
Rust uses traits + generics to provide zero-cost abstraction. The compiler monomorphizes generics (specializes per concrete type), keeping performance like handwritten code while ensuring strong static guarantees.

Examples (short)
- Define a trait `Summary` with a default `summarize` method.
- Implement for structs and use it as a trait bound: `fn notify<T: Summary>(item: &T)`.
- Generic structs: `struct Pair<T> { a: T, b: T }` and implementations with trait bounds.

Comparison with other languages
- Python: duck typing at runtime vs Rust: explicit trait bounds at compile time.
- Go: interfaces are implicitly implemented; Rust requires explicit impls and traits can include default methods and associated types.
- TypeScript: structural typing with generics; Rust's generics are monomorphized and checked at compile time with ownership.

Exercises
- Exercise 12-1: Create a `Summary` trait with a default method and implement it for two types.
- Exercise 12-2: Write a generic `largest<T: PartialOrd + Copy>(items: &[T]) -> T` function.
- Exercise 12-3: Implement a generic `Pair<T>` with a method `fn cmp_display(&self) where T: PartialOrd + Display` that prints the larger value.
- Exercise 12-4 (advanced): Create a trait with an associated type and implement it for a concrete type.

Key takeaways
- Traits describe behavior; generics enable code reuse without runtime cost.
- Use trait bounds and where-clauses to express constraints clearly.

References
- The Rust Book: Traits and Generics

