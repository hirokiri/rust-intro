Lesson 16 — Testing & Documentation

Overview

This lesson covers writing unit tests, doctests, and integration tests, plus writing effective documentation comments. Use `cargo test` to run tests and `cargo test <filter>` to run selected tests.

Doc tests

- Place examples in `///` comments and verify with `cargo test`.
- The example in `add()` will run as a doctest and must compile.

Unit tests

- Use `#[cfg(test)]` and `mod tests` in the same file for unit tests.
- Use `#[should_panic]` for tests that expect failure.

Integration tests

- Put tests in the `tests/` directory that use the crate as a library: `use mycrate::...`.

Examples & exercises

1. Run `cargo test` to execute unit tests and doctests.
2. Create an integration test under `tests/` that asserts `add(3,4) == 7`.
3. Add edge-case tests (overflow, negatives) and run `cargo test -- --nocapture` to see printed output.
