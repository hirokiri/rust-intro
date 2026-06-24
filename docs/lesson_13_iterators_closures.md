Lesson 13 — Iterators & Closures

Overview

Iterators provide lazy, composable pipelines for processing sequences without intermediate allocations until a consumer (like collect, sum, or fold) runs. Closures are anonymous functions that can capture their environment; their behavior is characterized by the Fn, FnMut, and FnOnce traits.

Iterator basics

- iter(): borrows each element (&T)
- into_iter(): consumes the collection (T)
- iter_mut(): yields &mut T

Common adapters

- map, filter, take, skip, enumerate, peekable, flat_map, zip, rev

Common consumers

- collect, sum, product, fold, find, any, all, for_each

Closure capture semantics

- Fn: closure that only borrows immutably
- FnMut: closure that mutates captured environment
- FnOnce: closure that takes ownership of captured values (consumed when called)
- `move` forces capture by value (useful for threads or when transferring ownership)

Examples

1) Basic adapters and consumers

```rust
let v = vec![1, 2, 3, 4, 5];
let squared_evens: Vec<i32> = v.iter()
    .map(|x| x * x)
    .filter(|x| x % 2 == 0)
    .cloned()
    .collect();
// squared_evens == [4, 16]
```

2) into_iter vs iter vs iter_mut

```rust
let mut names = vec![String::from("alice"), String::from("bob")];
for s in names.iter() { println!("ref: {}", s); } // &String
for s in names.iter_mut() { s.push_str("!"); } // &mut String
for s in names.into_iter() { println!("owned: {}", s); } // String, names consumed
```

3) Closures: Fn, FnMut, FnOnce

```rust
let x = 5;
let add_x = |n: i32| n + x; // Fn: borrows x immutably

let mut counter = 0;
let mut inc = |n: i32| { counter += n; counter }; // FnMut: mutates counter

let s = String::from("hello");
let consume = move || drop(s); // FnOnce: s moved into closure and consumed
```

4) Useful iterator patterns

```rust
let sum_of_squares: i32 = (1..=10).map(|x| x*x).filter(|x| x % 2 == 0).sum();
let words = vec!["a b", "c d e"];
let flattened: Vec<&str> = words.into_iter().flat_map(|s| s.split_whitespace()).collect();
```

Tips and pitfalls

- Iterator chains are lazy; nothing happens until a consumer runs.
- Watch capture modes: borrowing a collection inside a closure can cause borrow errors if the collection is used later.
- Prefer iterator adapters for clarity and potential performance gains over manual loops.

Exercises (expanded)

- Exercise 13-1: Given Vec<i32>, produce a Vec<i32> of squared even numbers using iterator adapters. Show both iter() + cloned() and into_iter() variants.
- Exercise 13-2: Implement product using fold and compare to product() when available.
- Exercise 13-3: Demonstrate Fn, FnMut, FnOnce by writing three closures that capture/modify/consume an outer variable.

References

- The Rust Book — Iterators: https://doc.rust-lang.org/book/ch13-02-iterators.html
- The Rust Book — Closures: https://doc.rust-lang.org/book/ch13-01-closures.html
- Rust by Example — Iterators & Closures
