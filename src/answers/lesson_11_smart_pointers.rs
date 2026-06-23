// Lesson 11 — Smart Pointers (Box, Rc, RefCell)
// Exercises: Box usage, recursive list with Box, Rc cloning & strong_count, RefCell interior mutability and shared mutable state examples.
// Entry: demonstrates Box, Rc, RefCell patterns
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    // Exercise 11-1: Box usage and dereference
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);
    println!("Dereferenced value: {}", *boxed);
    println!("Doubled boxed value: {}", double_boxed(boxed));

    // Exercise 11-2: recursive list with Box
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil))),
        )),
    );
    println!("List sum: {}", list_sum(&list));

    // Exercise 11-3: Rc cloning and checking strong_count
    let shared_name = Rc::new(String::from("Ferris"));
    println!("Initial strong count: {}", Rc::strong_count(&shared_name));

    let first = Rc::clone(&shared_name);
    println!("After first clone: {}", Rc::strong_count(&shared_name));

    {
        let second = Rc::clone(&shared_name);
        println!("After second clone: {}", Rc::strong_count(&shared_name));
        println!("First clone: {}", first);
        println!("Second clone: {}", second);
    }

    println!(
        "After second clone drops: {}",
        Rc::strong_count(&shared_name)
    );

    // Exercise 11-4: RefCell interior mutability example
    let counter = RefCell::new(0);
    *counter.borrow_mut() += 1;
    *counter.borrow_mut() += 1;
    *counter.borrow_mut() += 1;
    println!("Counter value: {}", counter.borrow());

    // Exercise 11-5: shared mutable counter using Rc<RefCell>
    let shared_counter = Rc::new(RefCell::new(0));
    println!(
        "Shared counter initial strong count: {}",
        Rc::strong_count(&shared_counter)
    );

    let add_five = Rc::clone(&shared_counter);
    let add_ten = Rc::clone(&shared_counter);

    *add_five.borrow_mut() += 5;
    *add_ten.borrow_mut() += 10;

    println!(
        "Shared counter final strong count: {}",
        Rc::strong_count(&shared_counter)
    );
    println!("Shared counter value: {}", shared_counter.borrow());
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn double_boxed(value: Box<i32>) -> i32 {
    *value * 2
}

fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + list_sum(next),
        List::Nil => 0,
    }
}

fn main() {
    run();
}
