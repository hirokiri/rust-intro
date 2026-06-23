# Rust Learning Journey 🦀

Welcome to your Rust learning repository! This project is a guided exploration of the Rust programming language, led by your dedicated mentor, focusing on a strong foundation before building a CLI project.

## Curriculum Overview

| Lesson | Topic | Status | Document |
| :--- | :--- | :--- | :--- |
| **0** | Setup & "Hello, World!" | ✅ Completed | [View Doc](docs/lesson_00_setup_hello_world.md) |
| **1** | Variables & Mutability | ✅ Completed | [View Doc](docs/lesson_01_variables_mutability.md) |
| **2** | Data Types & Functions | ✅ Completed | [View Doc](docs/lesson_02_data_types_functions.md) |
| **3** | Control Flow (if, loop, while, for) | ✅ Completed | [View Doc](docs/lesson_03_control_flow.md) |
| **4** | Ownership Basics | ✅ Completed | [View Doc](docs/lesson_04_ownership_basics.md) |
| **5** | Borrowing & References | ✅ Completed | [View Doc](docs/lesson_05_borrowing_references.md) |
| **6** | Lifetimes Basics | ✅ Completed | [View Doc](docs/lesson_06_lifetimes_basics.md) |
| **7** | Structs & Methods | ✅ Completed | [View Doc](docs/lesson_07_structs_methods.md) |
| **8** | Enums & Pattern Matching | ✅ Completed | [View Doc](docs/lesson_08_enums_pattern_matching.md) |
| **9** | Strings & Collections (Vec, HashMap) | ⏳ Pending | - |
| **10** | Error Handling | ⏳ Pending | - |
| **11** | Smart Pointers (Box, Rc, RefCell) | ⏳ Pending | - |
| **12** | Traits & Generics | ⏳ Pending | - |
| **13** | Iterators & Closures | ⏳ Pending | - |
| **14** | Modules & Project Structure | ⏳ Pending | - |
| **15** | Concurrency (Threads, Channels, Mutex) | ⏳ Pending | - |
| **16** | Testing & Documentation | ⏳ Pending | - |
| **17** | Macros & Advanced Features | ⏳ Pending | - |

## Project Structure

- `src/main.rs`: Our primary playground where we write and run code for each lesson.
- `docs/`: Contains reference documents, concept explanations, and exercises for every completed lesson.
- `Cargo.toml`: The manifest file for our project and its dependencies.
- `AGENTS.md` / `GEMINI.md`: Mission control files that track the mentorship style, exact goals, and curriculum state.

## How to Run

To run the current code in `src/main.rs`, use:
```bash
cargo run
```

To check for compilation errors without building (much faster):
```bash
cargo check
```

Run individual lesson answer files in `src/answers/` (each file is a small standalone example with its own main()):

```bash
# Compile & run a single answer file (example):
rustc src/answers/lesson_01_variables_mutability.rs -o /tmp/lesson_01 && /tmp/lesson_01

# Compile & run all answer files (Linux/macOS):
for f in src/answers/*.rs; do
  echo "Running $f"
  rustc "$f" -o "/tmp/$(basename "${f%.*}")" && "/tmp/$(basename "${f%.*}")"
done
```

Note: These `rustc` commands compile each file as a standalone program. Use the loop to run every example in order; `cargo run` still builds the main crate.
