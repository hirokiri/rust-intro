fn main() {
    // =========================================================================
    // 1. The Lifetime Problem: Out-of-scope borrow
    // =========================================================================
    // Try to understand what this commented block is doing. 
    // It would fail to compile because `x` goes out of scope before `r` is printed.
    /*
    let r;
    {
        let x = 5;
        r = &x; // ❌ Compile Error: `x` does not live long enough!
    }
    println!("r: {}", r);
    */

    // =========================================================================
    // 2. The `longest` Function & Explicit Lifetimes
    // =========================================================================
    let string1 = String::from("Rust rules!");
    let string2 = "Go";

    // `longest` compares string1 and string2, and returns a reference.
    // The compiler uses the lifetime parameter `'a` to know that the return
    // value lives as long as the SHORTER of string1 and string2.
    let result = longest(string1.as_str(), string2);
    println!("2. Longest string: {}", result);

    // =========================================================================
    // 3. Different Scopes & Lifetimes
    // =========================================================================
    let string_a = String::from("Systems programming");
    {
        let string_b = String::from("Python");
        // Because string_b is in a smaller inner scope, the returned reference
        // `result_inner` cannot outlive string_b!
        let result_inner = longest(string_a.as_str(), string_b.as_str());
        println!("3. Longest in inner scope: {}", result_inner); // ✅ Works!
    }
    // println!("result_inner is no longer valid here!"); // ❌ If printed here, it would fail!

    // =========================================================================
    // 4. Your Exercises Start Here!
    // =========================================================================
    println!("\n--- Running Exercises ---");

    // Exercise 6-1: Call `longest` with variables of different scopes
    // and demonstrate both the working case (using the result inside the scope)
    // and a commented-out failing case (using it outside).

    let string1 = String::from("Systems programming");
    {
        let string2 = String::from("Python");
        // Because string_b is in a smaller inner scope, the returned reference
        // `result_inner` cannot outlive string_b!
        let result_inner = longest(string1.as_str(), string2.as_str());
        println!("3. Longest in inner scope: {}", result_inner); // ✅ Works!
    }
    // println!("3. Longest in inner scope: {}", result_inner); // ❌ No longer compile

    // Exercise 6-2: Test your `first_word` function here.
    let sentence = String::from("Hello Rust world");
    let first = first_word(&sentence);
    println!("Ex 6-2 - First word: {}", first);

    // Exercise 6-3: Test your `Book` struct with lifetime annotations here.
    let title = String::from("The Rust Book");
    let author = String::from("Steve Klabnik");
    let my_book = Book {
        title: &title,
        author: &author,
    };
    println!("Book: {} by {}", my_book.title, my_book.author);
}

// The `longest` function takes two string slices that live for at least `'a`
// and returns a string slice that also lives for at least `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// TODO: Implement Exercise 6-2: first_word
fn first_word(s: &str) -> &str {
    let s_slice: Vec<&str> = s.split(' ').collect();
    s_slice[0]
}

// TODO: Implement Exercise 6-3: Book struct
struct Book<'a> {
    title: &'a str,
    author: &'a str,

}