// Lesson 14 — Modules & Project Structure

// Inline module example
pub mod math {
    /// Add two integers (public)
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Nested module
    pub mod nested {
        /// Double a value
        pub fn double(x: i32) -> i32 {
            x * 2
        }
    }
}

// Public run entry called by the top-level dispatcher
pub fn run() {
    println!("Lesson 14 — Modules & Project Structure");

    // Fully-qualified path
    println!("2 + 3 = {}", crate::answers::lesson_14_modules_project_structure::math::add(2, 3));

    // Use a local import to shorten paths
    use crate::answers::lesson_14_modules_project_structure::math::nested::double;
    println!("double 4 = {}", double(4));

    // Demonstrate `use` bringing items into scope
    use crate::answers::lesson_14_modules_project_structure::math::add;
    println!("2 + 5 = {}", add(2, 5));

    println!("Module demo complete.");
}
