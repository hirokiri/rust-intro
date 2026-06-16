# Lesson 3: Control Flow 🔀

## `if` Expressions

In Rust, `if` statements don't require parentheses around the condition. 

Crucially, `if` is an **expression**, meaning it returns a value. Because of this, Rust doesn't have a ternary operator (`condition ? true_val : false_val`). You just use `if`:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

**Important Rule:** The values returned from each arm of the `if`/`else` block must have the exact same type.

```rust
// ❌ ERROR: expected integer, found `&str`
let error_num = if condition { 5 } else { "six" }; 
```

## Loops

Rust has three kinds of loops: `loop`, `while`, and `for`.

### 1. `loop` (Infinite loop)

The `loop` keyword tells Rust to execute a block of code forever or until you explicitly tell it to stop with `break`.

```rust
loop {
    println!("Again!");
    break; // Stops the loop
}
```

**Returning Values from loops:**
You can return a value from a `loop` by putting it after the `break` keyword. This is incredibly useful for retry logic.

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // the loop expression evaluates to 20
    }
};
```

### 2. `while` loop

Standard conditional loop. It runs as long as the condition evaluates to `true`.

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

### 3. `for` loop

The `for` loop is the most common and safest loop in Rust. It iterates over a collection (like an array) or an iterator. It eliminates the risk of infinite loops and out-of-bounds access.

**Iterating over a collection:**
```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {}", element);
}
```

**Iterating over a range:**
Rust has built-in syntax for generating ranges of numbers using `..` (exclusive end) or `..=` (inclusive end).

```rust
for number in 1..4 {
    println!("{}!", number); // prints 1, 2, 3
}

for number in (1..4).rev() {
    println!("{}!", number); // prints 3, 2, 1
}
```

---

## Comparison with Other Languages

| Concept | Python | Go | TypeScript | Rust |
|---|---|---|---|---|
| Parentheses around condition? | No | No | Yes | No |
| Ternary Operator? | `"a" if cond else "b"` | N/A | `cond ? "a" : "b"` | `if cond { "a" } else { "b" }` |
| Infinite Loop | `while True:` | `for { }` | `while (true)` | `loop { }` |
| Range loop | `for i in range(1,4):` | `for i := 1; i < 4; i++`| `for (let i=1; i<4; i++)` | `for i in 1..4 { }` |

---

## Key Takeaways

1. **`if` is an expression**: You can assign its result to a variable.
2. **`loop` is for infinite loops**, and it can return values via `break value;`.
3. **`for` loops with ranges (`1..4`)** are the idiomatic way to loop a specific number of times.

---

## Exercises

### Exercise 3-1: The Temperature Check
Write an `if` expression that checks a `temperature` variable. If it's above 30, assign the string "Hot" to a variable called `weather`. If it's below 10, assign "Cold". Otherwise, assign "Nice". Print the result.

### Exercise 3-2: FizzBuzz
Write the classic FizzBuzz program using a `for` loop and ranges from 1 to 20 (inclusive).
- If divisible by 3, print "Fizz"
- If divisible by 5, print "Buzz"
- If both, print "FizzBuzz"
- Otherwise, print the number.

### Exercise 3-3: The Retry Simulator
Create a `loop` that increments a `mut attempts` variable. If `attempts` reaches 5, `break` the loop and return the string "Connection established". Assign the result of the loop to a variable and print it.

---

## Reference

- [The Rust Book — Ch.3.5: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
