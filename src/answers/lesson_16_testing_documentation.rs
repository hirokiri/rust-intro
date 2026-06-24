// Lesson 16 — Testing & Documentation

/// Add two integers.
///
/// # Examples
///
/// ```
/// assert_eq!(rust_intro::answers::lesson_16_testing_documentation::add(1,2), 3);
/// ```
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

    #[test]
    #[should_panic]
    fn panic_example() {
        panic!("example panic");
    }
}
