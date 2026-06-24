Lesson 13 — Iterators & Closures

Concepts

- Iterators: lazy adapters over sequences (iter, into_iter, iter_mut).
- Adapters: map, filter, take, enumerate, flat_map, peekable, rev, etc.
- Consumers: collect, sum, product, fold, for_each, find.
- Closures: anonymous functions that capture environment (Fn, FnMut, FnOnce). Use `move` to transfer ownership into a closure.

Examples

- Use iterators for efficient, readable pipelines: v.iter().map(|x| x*2).filter(|x| x%2==0).collect::<Vec<_>>().
- Closures behave like lightweight lambdas; types are inferred but can be annotated when helpful.

Why this matters (comparisons)

- Python: list comprehensions/eager; Rust iterators are lazy and zero-cost.
- Go: no generics-era iterators; use loops. Rust iterator chains are composable and expressive.
- TypeScript: arrays have map/filter but without Rust's ownership guarantees and performance.

Exercises

Exercise 13-1: Given Vec<i32>, produce a Vec<i32> of squared even numbers using iterator adapters.

Exercise 13-2: Implement a function that returns the product of elements using fold.

Exercise 13-3: Write a closure that captures a multiplier and use it in an iterator map (show both Fn and move closure variants).

Key takeaways

- Prefer iterator adapters over manual loops when transforming collections.
- Understand closure capture semantics to avoid borrow issues.
- Use move closures when spawning threads or transferring ownership.

References

- The Rust Book: Iterators and Closures
- Rust by Example: Closures and Iterators
