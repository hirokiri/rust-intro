# Lesson 7: Structs & Methods

In earlier lessons, you learned how Rust stores values, moves ownership, and borrows data through references. Structs let us group related values into a named type, and methods let us attach behavior to that type.

If you are coming from Python, TypeScript, or Go, structs will feel familiar: they are Rust's main way to model records, domain objects, and data with behavior. The important Rust difference is that ownership and borrowing still apply to every field and every method call.

---

## Structs: Named Data

A struct defines a new type with named fields:

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

You create an instance by providing every field:

```rust
let user = User {
    username: String::from("hiro"),
    email: String::from("hiro@example.com"),
    active: true,
    sign_in_count: 1,
};
```

Access fields with dot syntax:

```rust
println!("{}", user.email);
```

To mutate a field, the whole binding must be mutable:

```rust
let mut user = User {
    username: String::from("hiro"),
    email: String::from("hiro@example.com"),
    active: true,
    sign_in_count: 1,
};

user.email = String::from("new@example.com");
```

Rust does not allow marking only one field mutable. Mutability belongs to the binding, not the individual field.

---

## Field Init Shorthand

When variable names match field names, Rust lets you write the field once:

```rust
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
```

This is similar to JavaScript/TypeScript object shorthand:

```typescript
const user = { username, email };
```

The Rust version still moves owned values into the struct. After `username` is moved into `User`, the original variable cannot be used unless it was cloned.

---

## Struct Update Syntax

You can build one struct from another:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

This copies or moves the remaining fields from `user1`.

Important ownership detail:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

Fields like `active: bool` and `sign_in_count: u64` are copied because they implement `Copy`. Fields like `username: String` are moved. After this, you cannot use `user1.username`, because ownership moved into `user2`.

---

## Tuple Structs

Tuple structs give a tuple a distinct type name:

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Even though both contain three numbers, `Color` and `Point` are different types. This is useful when you want lightweight type safety without naming every field.

---

## Unit-Like Structs

A unit-like struct has no fields:

```rust
struct AlwaysEqual;
```

These are most useful later when we implement traits. For now, just know that Rust allows types with no stored data.

---

## Debug Printing

Structs do not automatically know how to print with `{}`:

```rust
println!("{}", user); // error
```

For development, add `#[derive(Debug)]` above the struct:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle {
    width: 30,
    height: 50,
};

println!("{:?}", rect);
println!("{:#?}", rect);
```

`{:?}` uses debug formatting. `{:#?}` pretty-prints over multiple lines.

---

## Methods with `impl`

Methods are functions attached to a type. They live inside an `impl` block:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

`&self` is shorthand for `self: &Self`.

This means `area` borrows the rectangle immutably. It can read fields, but it cannot mutate them and it does not take ownership.

```rust
let rect = Rectangle {
    width: 30,
    height: 50,
};

println!("area: {}", rect.area());
println!("{:?}", rect); // still usable because area only borrowed it
```

---

## Method Receivers

Rust methods commonly use one of three receiver forms:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn grow(&mut self, amount: u32) {
        self.width += amount;
        self.height += amount;
    }

    fn destroy(self) {
        println!("dropping rectangle: {:?}", self);
    }
}
```

Use `&self` when the method only needs to read.

Use `&mut self` when the method needs to mutate.

Use `self` when the method should consume the value.

That last form is less common for beginner code, but it matters. A method taking `self` owns the value, so the caller cannot use it afterward.

---

## Associated Functions

Functions in an `impl` block that do not take `self` are called associated functions:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let square = Rectangle::square(10);
```

You call associated functions with `::`, not dot syntax, because they are attached to the type rather than to a specific instance.

This is Rust's common constructor pattern. Rust does not have special constructor syntax like Python's `__init__`, TypeScript's `constructor`, or Go's common `NewType` convention.

---

## Structs and Ownership

A struct owns its fields unless those fields are references:

```rust
struct User {
    username: String,
}
```

This `User` owns its `String`. When the `User` is dropped, the `String` is dropped too.

If a struct stores references, it needs lifetimes:

```rust
struct UserView<'a> {
    username: &'a str,
}
```

This `UserView` does not own the string data. It only borrows it, so Rust needs to know how long the borrowed data must remain valid.

In application code, prefer owned fields like `String` unless you have a specific reason to store references. Owned structs are often easier to move around, return from functions, and store in collections.

---

## How This Compares to Other Languages

### Python

Python classes combine fields and methods, but field layout is flexible by default. You can often add attributes dynamically:

```python
user.name = "Hiro"
user.timezone = "Asia/Tokyo"
```

Rust structs are fixed at compile time. That makes them less flexible, but much easier for the compiler to optimize and verify.

### Go

Go structs are close to Rust structs:

```go
type Rectangle struct {
    Width  uint
    Height uint
}
```

Go methods also attach behavior to types. The big Rust difference is receiver borrowing:

```rust
fn area(&self) -> u32
fn grow(&mut self, amount: u32)
fn destroy(self)
```

Rust makes read, mutation, and ownership transfer explicit in the method signature.

### TypeScript

TypeScript object types are structural:

```typescript
type Rectangle = {
  width: number;
  height: number;
};
```

If two values have the same shape, TypeScript usually treats them as compatible. Rust is more nominal: a `Rectangle` and a `ScreenSize` are different types even if both contain `width` and `height`.

---

## Key Takeaways

1. Structs define custom data types with named fields.
2. Struct instances own their fields unless the fields are references.
3. `impl` blocks attach methods and associated functions to a type.
4. `&self`, `&mut self`, and `self` describe whether a method reads, mutates, or consumes the value.
5. `#[derive(Debug)]` lets you inspect structs with `{:?}` and `{:#?}`.
6. Prefer owned fields like `String` while learning, then use borrowed fields when lifetimes are truly useful.

---

## Exercises

### Exercise 7-1: Define a `Rectangle`

Create a `Rectangle` struct with `width: u32` and `height: u32`.

In `main`, create a rectangle and print it with debug formatting.

### Exercise 7-2: Add Methods

Add these methods to `Rectangle`:

```rust
fn area(&self) -> u32
fn can_hold(&self, other: &Rectangle) -> bool
```

`can_hold` should return `true` when `self` is wider and taller than `other`.

### Exercise 7-3: Add an Associated Function

Add:

```rust
fn square(size: u32) -> Rectangle
```

Call it with:

```rust
let square = Rectangle::square(25);
```

### Exercise 7-4: Mutating Method

Add:

```rust
fn scale(&mut self, factor: u32)
```

It should multiply both `width` and `height` by `factor`.

In `main`, create a mutable rectangle, call `scale`, and print the result.

### Exercise 7-5: Model a User

Create a `User` struct with owned fields:

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

Write a `build_user(username: String, email: String) -> User` function using field init shorthand.

Then create a second user with struct update syntax. Notice which fields move and which fields copy.

---

## Reference

- [The Rust Book - Ch.5: Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
