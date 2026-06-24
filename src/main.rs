use rust_intro::answers;
use rust_intro::sandbox;
use std::env;

fn usage() {
    eprintln!("Usage: --lesson N   (e.g. --lesson 1)");
    eprintln!("Available lessons: 0..13");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lesson: Option<usize> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--lesson" => {
                if i + 1 < args.len() {
                    lesson = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    usage();
                    std::process::exit(1);
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    match lesson {
        Some(n) => match n {
            0 => answers::lesson_00_setup_hello_world::run(),
            1 => answers::lesson_01_variables_mutability::run(),
            2 => answers::lesson_02_data_types_functions::run(),
            3 => answers::lesson_03_control_flow::run(),
            4 => answers::lesson_04_ownership_basics::run(),
            5 => answers::lesson_05_borrowing_references::run(),
            6 => answers::lesson_06_lifetimes_basics::run(),
            7 => answers::lesson_07_structs_methods::run(),
            8 => answers::lesson_08_enums_pattern_matching::run(),
            9 => answers::lesson_09_strings_collections::run(),
            10 => answers::lesson_10_error_handling::run(),
            11 => answers::lesson_11_smart_pointers::run(),
            12 => answers::lesson_12_traits_generics::run(),
            13 => answers::lesson_13_iterators_closures::run(),
            other => {
                eprintln!("Unknown lesson: {}", other);
                usage();
                std::process::exit(1);
            }
        },
        None => {
            // No --lesson provided: delegate to sandbox runner (default behavior)
            sandbox::run_from_args(&args);
        }
    }
}
