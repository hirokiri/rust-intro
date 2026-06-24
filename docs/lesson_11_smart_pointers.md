# Lesson 11: Smart Pointers (`Box`, `Rc`, `RefCell`)

So far, most values have had one clear owner. That simple model is Rust's default, and it is one reason Rust can be both fast and memory safe.

But some programs need more flexible ownership patterns:

- put data on the heap even when the value has a known owner
- let multiple parts of the program share ownership
- mutate data even when the outer value is not declared `mut`

Rust supports these patterns with smart pointers. A smart pointer is a type that acts like a pointer but also carries extra behavior.

This lesson focuses on three common smart pointers:

- `Box<T>`: single ownership of heap data
- `Rc<T>`: multiple ownership in single-threaded code
- `RefCell<T>`: borrow checking at runtime instead of compile time

The important theme: smart pointers do not remove Rust's ownership rules. They move some ownership decisions into carefully designed library types.

---

## 1. What Is a Smart Pointer?

A normal reference, like `&T`, borrows a value owned somewhere else:

```rust
let value = 10;
let reference = &value;

println!("{}", reference);
```

A smart pointer is usually a struct that owns or manages some value:

```rust
let value = Box::new(10);

println!("{}", value);
```

`Box::new(10)` stores `10` on the heap, while the `Box` itself lives on the stack. When the `Box` goes out of scope, Rust automatically frees the heap allocation.

This automatic cleanup comes from the `Drop` trait. You rarely call `drop` directly; Rust calls it for you at the end of a value's lifetime.

---

## 2. `Box<T>`: Put a Value on the Heap

`Box<T>` gives you owned heap allocation.

```rust
fn main() {
    let number = Box::new(5);

    println!("number: {}", number);
}
```

For small values like `i32`, this is not useful by itself. The box adds an allocation. `Box<T>` matters when:

- you need a type with a known size
- you want to transfer ownership of a large value without copying it
- you want trait objects later, such as `Box<dyn Display>`

### Recursive Types Need `Box`

Rust must know the size of every type at compile time. This enum does not compile:

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

Why? `List::Cons` contains another `List`, which may contain another `List`, forever. The compiler cannot compute a finite size.

Use `Box<List>` to break the infinite size:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

Now `Cons` contains an `i32` plus a `Box<List>`. A box has a known size because it stores a pointer to heap data.

This is similar to linked structures in Go, where a struct can contain a pointer to the same struct type:

```go
type Node struct {
    Value int
    Next  *Node
}
```

Rust makes the pointer explicit with `Box`.

---

## 3. Dereferencing Smart Pointers

`Box<T>` implements `Deref`, so you can use `*` to access the value inside:

```rust
let boxed = Box::new(10);
let value = *boxed;

println!("{}", value);
```

Rust also performs deref coercion in many places. That means a `&Box<String>` can often behave like a `&String`, and a `&String` can behave like a `&str`.

```rust
fn print_label(label: &str) {
    println!("{}", label);
}

let boxed_name = Box::new(String::from("Ada"));

print_label(&boxed_name);
```

The compiler follows the dereference chain for you. This is ergonomic, but the ownership is still precise: the `Box` owns the `String`.

---

## 4. `Rc<T>`: Multiple Owners

Sometimes one value logically belongs to more than one owner. Rust's default ownership model does not allow that:

```rust
let name = String::from("shared");
let a = name;
let b = name; // compile error: name was moved
```

`Rc<T>` means reference-counted pointer. It lets multiple owners point to the same value in single-threaded code.

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("shared value"));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    println!("{}", a);
    println!("{}", b);
    println!("owners: {}", Rc::strong_count(&shared));
}
```

Important: `Rc::clone(&shared)` does not clone the inner `String`. It increments a reference count. The heap data is freed when the last `Rc` owner is dropped.

This is conceptually closer to Python or TypeScript object references, where multiple variables can point at the same object. The difference is that Rust makes shared ownership explicit in the type: `Rc<T>`.

### When to Use `Rc<T>`

Use `Rc<T>` when:

- several parts of single-threaded code need to own the same value
- the value should be freed only after the final owner is gone
- immutable sharing is enough

Do not use `Rc<T>` for multithreaded sharing. Later, in the concurrency lesson, we will meet `Arc<T>`, the atomic reference-counted pointer for threaded code.

---

## 5. `RefCell<T>`: Runtime Borrow Checking

Normal Rust borrowing is checked at compile time:

```rust
let mut value = 10;
let reference = &mut value;

*reference += 1;
```

Sometimes the compiler cannot prove that your borrowing pattern is safe, even though you know it is safe by program logic. `RefCell<T>` moves the borrow check from compile time to runtime.

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);

    *value.borrow_mut() += 5;

    println!("{}", value.borrow());
}
```

Notice that `value` itself is not declared `mut`. The mutation happens through the `RefCell`.

This pattern is called interior mutability: the outer value looks immutable, but it contains a type that permits controlled mutation inside.

### Runtime Borrow Rules

`RefCell<T>` enforces the same core borrowing rule:

- any number of immutable borrows, or
- exactly one mutable borrow

But violations are detected at runtime and cause a panic:

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);

    let first = value.borrow_mut();
    let second = value.borrow_mut(); // panic at runtime

    println!("{} {}", first, second);
}
```

That is a tradeoff. `RefCell<T>` gives flexibility, but you lose some compile-time protection.

---

## 6. `Rc<RefCell<T>>`: Shared and Mutable

`Rc<T>` gives shared ownership, but only immutable access. `RefCell<T>` gives interior mutability, but only one owner by itself.

Together, `Rc<RefCell<T>>` means:

- many owners can point to the same value
- those owners can mutate the value through runtime borrow checks

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_count = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared_count);
    let b = Rc::clone(&shared_count);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 10;

    println!("count: {}", shared_count.borrow());
}
```

This pattern is useful, but it should make you cautious. If you reach for `Rc<RefCell<T>>` constantly, it may mean your ownership design is too tangled.

In Python and TypeScript, shared mutable objects are normal. Rust makes you spell out the cost: `Rc<RefCell<T>>`.

---

## 7. Choosing the Right Tool

Use normal ownership and borrowing first.

Use `Box<T>` when:

- one owner is enough
- you need heap allocation
- you need to make a recursive type possible

Use `Rc<T>` when:

- multiple owners need shared read-only access
- the code is single-threaded

Use `RefCell<T>` when:

- you need mutation through an immutable outer value
- the compiler cannot verify the borrowing pattern
- you are comfortable with possible runtime borrow panics

Use `Rc<RefCell<T>>` when:

- multiple owners need to mutate shared state
- the code is single-threaded
- a simpler ownership design does not fit

---

## Key Takeaways

1. Smart pointers are ordinary Rust types with pointer-like behavior and extra guarantees.
2. `Box<T>` owns heap data and is useful for recursive types.
3. `Rc<T>` enables multiple ownership in single-threaded programs.
4. `Rc::clone` increments a reference count; it usually does not deep-copy the inner value.
5. `RefCell<T>` checks borrow rules at runtime instead of compile time.
6. `RefCell<T>` enables interior mutability.
7. `Rc<RefCell<T>>` is powerful but should be used deliberately.

---

## Exercises

Open [main.rs](../src/main.rs) and implement the following exercises.

### Exercise 11-1: Boxed Number

Create a `Box<i32>` containing `42`.

Print:

- the boxed value
- the dereferenced value using `*`

Then write a function:

```rust
fn double_boxed(value: Box<i32>) -> i32
```

It should take ownership of the box and return double the inner value.

### Exercise 11-2: Recursive List with `Box`

Define this enum:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Create a list containing `1 -> 2 -> 3 -> Nil`.

Write a function:

```rust
fn list_sum(list: &List) -> i32
```

Use `match` to recursively sum the list.

### Exercise 11-3: Shared Name with `Rc`

Create one shared `String` using `Rc<String>`.

Clone it into two variables with `Rc::clone`.

Print:

- both cloned values
- the strong count after each clone
- the strong count after one clone goes out of scope

Use an inner block to make one clone drop early.

### Exercise 11-4: Counter with `RefCell`

Create:

```rust
let counter = RefCell::new(0);
```

Use `borrow_mut()` to increment it three times.

Print the final value using `borrow()`.

### Exercise 11-5: Shared Mutable Counter

Create an `Rc<RefCell<i32>>`.

Clone it into two variables:

- one should add `5`
- one should add `10`

Print the final value from the original owner.

Bonus: print `Rc::strong_count` before and after the clones.

---

## Reference

- [The Rust Book - Ch. 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
