// Lesson 06 — Lifetimes Basics
// Exercises: longest() with lifetime annotations, first_word() string slicing, and a Book struct demonstrating lifetime parameters.
// Entry: pub fn run()
pub fn run() {
    // Exercise 6-1: lifetime annotation with longest()
    let outer = String::from("long-lived string");

    {
        let inner = String::from("short");
        let result = longest(outer.as_str(), inner.as_str());
        println!("Longest inside block: {}", result);
    }

    // Exercise 6-2: string slicing first_word()
    let sentence = "Rust lifetimes make borrowing explicit";
    println!("First word: {}", first_word(sentence));

    // Exercise 6-3: structs with lifetime parameters (Book)
    let title = String::from("The Rust Book");
    let author = String::from("Steve Klabnik");

    let my_book = Book {
        title: &title,
        author: &author,
    };

    println!("Book: {} by {}", my_book.title, my_book.author);
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

