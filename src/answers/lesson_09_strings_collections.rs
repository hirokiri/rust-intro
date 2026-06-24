// Lesson 09 — Strings & Collections
// Exercises: string formatting/manipulation, Vec iteration and aggregation, Option/Index handling, HashMap word counts and inventory updates.
// Entry: uses HashMap and collection helpers
use std::collections::HashMap;

// Exercise 9-1: string formatting and name helpers
pub fn run() {
    println!("{}", format_name("Ada", "Lovelace"));
    println!("{}", format_name_with_push("Grace", "Hopper"));

    // Exercise 9-2: Vec iteration and aggregation helpers (total, average, highest)
    let scores = vec![92, 81, 77, 100, 68];
    for score in &scores {
        println!("score: {}", score);
    }
    println!("total: {}", total_score(&scores));
    println!("average: {}", average_score(&scores));
    println!("highest: {:?}", highest_score(&scores));

    println!("{}", describe_score(&scores, 2));
    println!("{}", describe_score(&scores, 99));

    // Exercise 9-3: HashMap word count example
    let counts = word_counts("rust is fast rust is safe rust is fun");
    println!("word counts: {:?}", counts);

    // Exercise 9-4: Inventory updates using HashMap
    let inventory = inventory_updates();
    println!("inventory: {:?}", inventory);
}

fn format_name(first: &str, last: &str) -> String {
    format!("{}, {}", last, first)
}

fn format_name_with_push(first: &str, last: &str) -> String {
    let mut name = String::new();
    name.push_str(last);
    name.push_str(", ");
    name.push_str(first);
    name
}

fn total_score(scores: &[i32]) -> i32 {
    scores.iter().sum()
}

fn average_score(scores: &[i32]) -> f64 {
    if scores.is_empty() {
        0.0
    } else {
        total_score(scores) as f64 / scores.len() as f64
    }
}

fn highest_score(scores: &[i32]) -> Option<i32> {
    scores.iter().max().copied()
}

fn describe_score(scores: &[i32], index: usize) -> String {
    match scores.get(index) {
        Some(score) => format!("Score at index {} is {}", index, score),
        None => format!("No score at index {}", index),
    }
}

fn word_counts(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    counts
}

fn inventory_updates() -> HashMap<String, i32> {
    let mut inventory = HashMap::new();
    inventory.insert(String::from("apples"), 3);
    inventory.insert(String::from("bananas"), 2);

    *inventory.entry(String::from("apples")).or_insert(0) += 5;
    inventory.entry(String::from("oranges")).or_insert(4);
    inventory.entry(String::from("bananas")).or_insert(99);

    inventory
}

