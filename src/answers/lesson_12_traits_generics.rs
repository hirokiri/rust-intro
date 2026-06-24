// Lesson 12 — Traits & Generics
// Exercises: Summary trait, generic largest, Pair<T> with trait bounds
use std::fmt::Display;

pub fn run() {
    // Exercise 12-1: Trait with default method
    let article = News { headline: String::from("Rust 1.70 released"), location: String::from("Internet") };
    let tweet = Tweet { username: String::from("ferris"), content: String::from("I love Rust!") };

    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());

    // Exercise 12-2: generic largest
    let nums = vec![3, 7, 2, 9, 4];
    println!("Largest number: {}", largest(&nums));

    let chars = vec!['a', 'z', 'm'];
    println!("Largest char: {}", largest(&chars));

    // Exercise 12-3: Pair<T> with bounds
    let p = Pair { a: 3, b: 5 };
    p.cmp_display();
}

// Trait definition with default implementation
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more)")
    }
}

struct News {
    headline: String,
    location: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} - {}", self.headline, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {}

// Generic function using trait bounds. Using Copy to return by value for simplicity.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct and method with where clause
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self {
        Pair { a, b }
    }
}

impl<T> Pair<T>
where
    T: PartialOrd + Display,
{
    fn cmp_display(&self) {
        if self.a >= self.b {
            println!("The larger is a = {}", self.a);
        } else {
            println!("The larger is b = {}", self.b);
        }
    }
}

