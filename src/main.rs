use std::env;

mod answers;

fn usage() {
    eprintln!("Usage:");
    eprintln!("  --lesson N         Run lesson N (e.g. --lesson 3)");
    eprintln!("  [N]                Positional lesson number (e.g. cargo run -- 3)");
    eprintln!("If neither is provided the program runs the sandbox default (lesson 1).");
}

fn run_lesson(n: usize) {
    match n {
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
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse flags and positional: prefer explicit --lesson, otherwise first positional numeric arg
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
            arg if !arg.starts_with('-') && lesson.is_none() => {
                // positional numeric lesson
                lesson = arg.parse().ok();
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    match lesson {
        Some(n) => run_lesson(n),
        None => {
            // unified sandbox behavior when no lesson specified: default to lesson 1
            run_lesson(1);
        }
    }
}
