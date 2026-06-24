use std::process;

/// Run the sandbox launcher using the provided args slice (argv style).
/// args[0] is program name. If args.len() > 1, args[1] is parsed as lesson number.
/// Default lesson is 1.
pub fn run_from_args(args: &[String]) {
    let lesson: usize = if args.len() > 1 {
        match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid lesson number: {}", args[1]);
                eprintln!("Usage: [lesson_number]");
                process::exit(1);
            }
        }
    } else {
        1
    };

    match lesson {
        0 => crate::answers::lesson_00_setup_hello_world::run(),
        1 => crate::answers::lesson_01_variables_mutability::run(),
        2 => crate::answers::lesson_02_data_types_functions::run(),
        3 => crate::answers::lesson_03_control_flow::run(),
        4 => crate::answers::lesson_04_ownership_basics::run(),
        5 => crate::answers::lesson_05_borrowing_references::run(),
        6 => crate::answers::lesson_06_lifetimes_basics::run(),
        7 => crate::answers::lesson_07_structs_methods::run(),
        8 => crate::answers::lesson_08_enums_pattern_matching::run(),
        9 => crate::answers::lesson_09_strings_collections::run(),
        10 => crate::answers::lesson_10_error_handling::run(),
        11 => crate::answers::lesson_11_smart_pointers::run(),
        12 => crate::answers::lesson_12_traits_generics::run(),
        13 => crate::answers::lesson_13_iterators_closures::run(),
        other => {
            eprintln!("Unknown lesson: {}", other);
            process::exit(1);
        }
    }
}
