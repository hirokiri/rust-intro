Lesson 14 — Modules & Project Structure

Welcome — this page explains modules in plain language with small examples you can copy-paste.

What is a module?
- A module groups related code (functions, types) together. Think of it as a folder inside your program that holds related tools.
- The crate root (src/main.rs or src/lib.rs) is the top-level module.

Why modules matter
- They keep code organized and prevent name clashes.
- They control visibility: you decide which items are public (accessible from other modules) and which stay private (for internal use).

Key keywords
- mod: declare a module
- pub: make an item visible outside its module
- pub(crate): visible anywhere inside this crate but hidden from other crates
- use: bring a path into scope to shorten names
- pub use: re-export an item so callers see a simpler public API

Short examples

1) Private vs public
```rust
mod math {
    fn internal_mul(a: i32, b: i32) -> i32 { a * b } // private
    pub fn add(a: i32, b: i32) -> i32 { a + b }       // public
}
// Call with: crate::math::add(1, 2)
```

2) Nested module and re-export
```rust
pub mod math {
    pub fn triple(x: i32) -> i32 { x * 3 }
    pub mod nested {
        pub fn double(x: i32) -> i32 { x * 2 }
        // re-export triple so callers can use math::nested::triple()
        pub use super::triple;
    }
}
```

How files map to modules
- `mod foo;` in a file looks for `foo.rs` or `foo/mod.rs` beside it.
- Prefer one module per file for larger modules (cleaner Git diffs, easier navigation).

Practical tips
- Use `pub(crate)` for helpers you want across your crate but not in public API.
- Keep small helpers private; expose only the functions you want users to rely on.
- Use `pub use` in a top-level module to present a stable, small surface area.

Exercises (easy, with increasing steps)
1. Move `math` into `src/math.rs`, add `mod math;` in main, and call `math::add(2,3)`.
2. Make a nested private helper and try to call it from another module — watch the compiler error and then make it `pub(crate)`.
3. Re-export a commonly used function from a nested module with `pub use` so callers use a flat path.

If anything is confusing, paste a small example and the compiler error — will explain the exact fix.