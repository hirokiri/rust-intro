// Lesson 14 — Modules & Project Structure

// Inline module example with private, pub(crate) and pub items
pub mod math {
    // private helper, not visible outside this module
    fn internal_mul(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Add two integers (public)
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Visible everywhere in the current crate
    pub fn triple(x: i32) -> i32 {
        internal_mul(x, 3)
    }

    // Nested module demonstrating relative paths and re-export
    pub mod nested {
        /// Double a value
        pub fn double(x: i32) -> i32 {
            x * 2
        }

        // Re-export parent's triple so consumers can access math::nested::triple()
        pub use super::triple;
    }
}

// Public run entry called by the top-level dispatcher
pub fn run() {
    println!("Lesson 14 — Modules & Project Structure");

    // Fully-qualified path
    println!(
        "2 + 3 = {}",
        crate::answers::lesson_14_modules_project_structure::math::add(2, 3)
    );

    // Use a local import to shorten paths
    use crate::answers::lesson_14_modules_project_structure::math::nested::double;
    println!("double 4 = {}", double(4));

    // pub(crate) item accessed from same crate
    println!(
        "triple 3 = {}",
        crate::answers::lesson_14_modules_project_structure::math::triple(3)
    );

    // Demonstrate re-export (nested::triple)
    println!(
        "nested::triple 4 = {}",
        crate::answers::lesson_14_modules_project_structure::math::nested::triple(4)
    );

    // Demonstrate `use` bringing items into scope
    use crate::answers::lesson_14_modules_project_structure::math::add;
    println!("2 + 5 = {}", add(2, 5));

    println!("Module demo complete.");
}
