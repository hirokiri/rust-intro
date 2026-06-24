// Lesson 16 — Testing & Documentation

/// Demonstrates a simple function, its doc comment, and a unit test.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Entry point for the lesson demo
pub fn run() {
    println!("Lesson 16 — Testing & Documentation");
    println!("add(2,3) = {}", add(2,3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
