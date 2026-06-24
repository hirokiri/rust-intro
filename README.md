# Rust Learning Journey 🦀

Welcome to your Rust learning repository! This project is a guided exploration of the Rust programming language, led by your dedicated mentor.

## Curriculum Overview

(see docs/ for per-lesson notes)

## Project Structure

- `src/main.rs`: CLI dispatcher — accepts `--lesson N` to run a specific lesson.
- `src/bin/sandbox.rs` and `src/sandbox.rs`: A sandbox runner that can run lessons directly (default lesson is 1).
- `src/answers/`: Per-lesson example modules exposing `pub fn run()` (no longer contain `main`).
- `docs/`: Lesson documents, exercises and explanations.
- `Cargo.toml`: Project manifest.

## How to Run

- Run a specific lesson from the main binary:

```bash
# runs lesson 3 via the main crate
cargo run -- --lesson 3
# or the built binary:
./target/debug/rust-intro --lesson 3
```

- If `--lesson` is not provided, `src/main.rs` delegates to the sandbox runner. Run the sandbox directly:

```bash
# default runs lesson 1 when no arg is given
cargo run
# or explicitly run the sandbox binary with an optional lesson arg
cargo run --bin sandbox -- 2
# or the compiled sandbox binary
./target/debug/sandbox 2
```

Notes

- Answer files under `src/answers/` expose `pub fn run()` and are invoked by the dispatcher/sandbox.
- Use `cargo build` / `cargo check` to compile and verify the project.

Enjoy learning Rust — experiment, break things, and ask questions!