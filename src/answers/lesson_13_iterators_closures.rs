// Lesson 13 — Iterators & Closures

// More detailed examples covering common adapters, consumers, and closure traits.
use std::iter;

pub fn run() {
    iterator_basics();
    iterator_patterns();
    closure_variants();
    collecting_examples();
}

fn iterator_basics() {
    let v = vec![1, 2, 3, 4, 5];

    // iter() yields &i32; cloned() transforms &i32 -> i32
    let squared_evens: Vec<i32> = v.iter().map(|&x| x * x).filter(|x| x % 2 == 0).collect();
    println!("squared evens (iter+cloned): {:?}", squared_evens);

    // into_iter() consumes the collection
    let v2 = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v2.into_iter().map(|x| x * 2).collect();
    println!("doubled (into_iter): {:?}", doubled);

    // iter_mut() allows mutation in-place
    let mut names = vec![String::from("alice"), String::from("bob")];
    for s in names.iter_mut() { s.push('!'); }
    println!("names after iter_mut: {:?}", names);
}

fn iterator_patterns() {
    // enumerate, peekable, zip, flat_map
    let nums: Vec<i32> = (1..=5).collect();
    for (i, n) in nums.iter().enumerate() {
        println!("index {} => {}", i, n);
    }

    let mut it = (1..).map(|x| x * x).peekable();
    println!("peek: {:?}", it.peek());
    for n in it.take(5) { println!("square: {}", n); }

    let words = vec!["a b", "c d e"];
    let flattened: Vec<&str> = words.into_iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flattened: {:?}", flattened);

    let a = [1,2,3];
    let b = [4,5,6];
    let zipped: Vec<(i32,i32)> = a.iter().cloned().zip(b.iter().cloned()).collect();
    println!("zipped: {:?}", zipped);

    let sum_of_squares: i32 = (1..=10).map(|x| x*x).filter(|x| x % 2 == 0).sum();
    println!("sum_of_squares (even squares): {}", sum_of_squares);
}

fn closure_variants() {
    // Fn: immutable borrow
    let x = 5;
    let add_x = |n: i32| n + x; // implements Fn
    println!("add_x(3) = {}", add_x(3));

    // FnMut: mutating captured environment
    let mut counter = 0;
    {
        let mut inc = |n: i32| { counter += n; counter };
        println!("inc(2) = {}", inc(2));
        println!("inc(3) = {}", inc(3));
    }
    println!("counter after closure: {}", counter);

    // FnOnce: consumes captured value
    let s = String::from("consumed");
    let consume = move || {
        println!("inside consume: {}", s);
        // s is moved and dropped here
    };
    consume();
    // cannot use s here
}

fn collecting_examples() {
    // collect into different container types
    let nums = vec![1,2,3,4];
    let set: std::collections::HashSet<i32> = nums.iter().cloned().collect();
    println!("collected into HashSet: {:?}", set);

    // product using fold vs product()
    let nums2 = [2,3,4];
    let product_fold: i32 = nums2.iter().fold(1, |acc, &x| acc * x);
    let product_direct: i32 = nums2.iter().product();
    println!("product_fold = {}, product_direct = {}", product_fold, product_direct);

    // infinite iterators + take
    let ones: Vec<i32> = iter::repeat(1).take(5).collect();
    println!("ones: {:?}", ones);
}

