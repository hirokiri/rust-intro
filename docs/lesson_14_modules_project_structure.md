Lesson 14 — Modules & Project Structure

Overview

Rust uses modules to organize code into namespaces and control visibility. Modules can be declared inline (mod name { ... }) or as separate files. The crate root (src/lib.rs or src/main.rs) is the top-level module.

Key concepts

- mod: declare a module
- pub: make items public outside their module
- pub(crate): visible within the current crate
- use: import paths into scope
- file system mapping: `mod foo;` looks for `foo.rs` or `foo/mod.rs`

Examples

- Inline modules for small helpers
- File modules for larger components

Exercises

Exercise 14-1: Create a `math` module in `src/answers/lesson_14_modules_project_structure.rs` with `add` and `nested::double` functions. Call them from `run()`.

Exercise 14-2: Split a module into a separate file in `src/` (create a `utils.rs`) and import it with `mod utils; use crate::utils::...`.

Key takeaways

- Use modules to structure code and control visibility
- Prefer separate files for larger modules; inline modules are good for small helpers
- `use` reduces verbosity when accessing deep paths
