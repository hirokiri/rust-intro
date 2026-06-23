use std::fs;
use std::num::ParseIntError;

fn main() {
    // Exercise 10-1: Safe Highest Score
    let scores = vec![92, 81, 77, 100, 68];
    let empty_scores: Vec<i64> = vec![];

    print_highest_score(&scores);
    print_highest_score(&empty_scores);

    // Exercise 10-2: Parse a Score
    for input in ["95", "not a score"] {
        match parse_score(input) {
            Ok(score) => println!("Parsed score: {}", score),
            Err(error) => println!("Could not parse '{input}': {error}"),
        }
    }

    // Exercise 10-3: Parse and Validate a Percentage
    for input in ["85", "101", "abc"] {
        match parse_percentage(input) {
            Ok(percentage) => println!("Valid percentage: {}%", percentage),
            Err(message) => println!("Invalid percentage '{input}': {message}"),
        }
    }

    // Exercise 10-4: Use `?`
    for (left, right) in [("40", "2"), ("40", "oops")] {
        match add_scores(left, right) {
            Ok(total) => println!("{left} + {right} = {total}"),
            Err(error) => println!("Could not add '{left}' and '{right}': {error}"),
        }
    }

    // Exercise 10-5: File Reading Experiment
    match read_notes() {
        Ok(contents) => println!("notes.txt contents:\n{contents}"),
        Err(error) => println!("Could not read notes.txt: {error}"),
    }
}

fn highest_score(scores: &[i64]) -> Option<i64> {
    scores.iter().max().copied()
}

fn print_highest_score(scores: &[i64]) {
    match highest_score(scores) {
        Some(score) => println!("Highest score: {}", score),
        None => println!("No scores available"),
    }
}

fn parse_score(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn parse_percentage(input: &str) -> Result<u8, String> {
    let number = input
        .parse::<u16>()
        .map_err(|_| format!("'{input}' is not a valid number"))?;

    if number > 100 {
        Err(format!("{number} is greater than 100"))
    } else {
        Ok(number as u8)
    }
}

fn add_scores(left: &str, right: &str) -> Result<i32, ParseIntError> {
    let left_score = left.parse::<i32>()?;
    let right_score = right.parse::<i32>()?;

    Ok(left_score + right_score)
}

fn read_notes() -> Result<String, std::io::Error> {
    fs::read_to_string("notes.txt")
}
