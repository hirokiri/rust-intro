use std::collections::HashMap;

fn main() {
    // Excersise 9-1
    let first_name = "ada";
    let last_name = "lovelace";
    let formatted_name = format_name(first_name, last_name);
    println!("{}", formatted_name); // Output: "Ada, Lovelace"

    // Exercise 9-2
    let scores = vec![92, 81, 77, 100, 68];
    let total = total_score(&scores);
    let average = average_score(&scores);
    let highest = highest_score(&scores);
    println!("Total score: {}", total); // Output: "Total score: 418"
    println!("Average score: {}", average); // Output: "Average score: 83.6"
    println!("Highest score: {:?}", highest); // Output: "Highest score: Some(100)"

    // Exercise 9-3
    let scores_vec = vec![85, 90, 78, 92];
    let index = 2;
    let description = describe_score(&scores_vec, index);
    println!("{}", description); // Output: "Score at index 2 is 78"
    println!("{}", describe_score(&scores_vec, 5)); // Output: "No score at index 5"

    // Excersise 9-4
    let text = "rust is fast rust is safe rust is fun";
    let counts = word_counts(text);
    for (word, count) in &counts{
        println!("{}: {}", word, count);
    }
    println!("{:?}", counts);

    //Excersise 9-5
    let mut inventory = HashMap::new();
    inventory.insert(String::from("apples"), 3);
    inventory.insert(String::from("bananas"), 2);
    //add 5 more apples
    let count_apples =  inventory.entry(String::from("apples")).or_default();
    *count_apples += 5;

    // add "oranges" with count 4 if does not already exist
    inventory.entry(String::from("oranges")).or_insert(4);

    // try to insert "bananas" with count 99 only if bananas are missing
    inventory.entry(String::from("bananas")).or_insert(99);

    println!("{:?}", inventory);
}


fn format_name(first: &str, last: &str) -> String {
    let mut first_chars = first.chars();
    let mut last_chars = last.chars();
    let formatted_first = match first_chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + first_chars.as_str(),
    };
    let formatted_last = match last_chars.next() {
        None => String::new(),
        Some(l) => l.to_uppercase().collect::<String>() + last_chars.as_str(),
    };
    format!("{}, {}", formatted_first, formatted_last)
}

fn total_score(scores: &[i64]) -> i64 {
    scores.iter().sum()
}

fn average_score(scores: &[i64]) -> f64 {
    let total = total_score(scores);
    let count = scores.len() as f64;
    if count == 0.0 {
        0.0
    } else {
        total as f64 / count
    }
}

fn highest_score(scores: &[i64]) -> i64 {
    *scores.iter().max().unwrap()
}



fn describe_score(score: &Vec<i32>, index: usize) -> String {
    if let Some(value) = score.get(index) {
        format!("Score at index {} is {}", index, value)
    } else {
        format!("No score at index {}", index)
    }
}


fn word_counts(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(String::from(word)).or_insert(0);
        *count += 1;
    }
    counts
}