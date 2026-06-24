Lesson 17 — Macros & Advanced Features

Overview

This lesson focuses on `macro_rules!` declarative macros and advanced idioms. Procedural macros (derive/attribute) are powerful but require a separate crate and are covered conceptually here.

Key ideas

- `macro_rules!` defines pattern-based macros (declarative)
- `stringify!`, repetition `$( ... )*`, and metavariables like `$ident`, `$expr`
- Macros can generate impls, functions, and repetitive code to reduce boilerplate

Code examples

1) Variadic debug-print macro

```rust
macro_rules! dbg_print { ( $( $x:expr ),* ) => { $( println!("{} = {:?}", stringify!($x), $x); )* } }
```

2) Macro generating code (getter example)

```rust
macro_rules! make_getter { ($name:ident, $field:ident, $ty:ty) => { impl $name { pub fn $field(&self) -> $ty { self.$field } } } }
```

Exercises

1. Write a `vec_of_strings!("a","b","c")` macro that expands into `vec![String::from("a"), ...]`.
2. Read about procedural macros and try a small `derive` macro in a separate crate (advanced).

Notes

- Macros run at compile time and can complicate error messages; use them judiciously.
- Prefer functions when possible; use macros when the code generation reduces duplication significantly.
