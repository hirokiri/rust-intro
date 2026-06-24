use rust_intro::sandbox;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    sandbox::run_from_args(&args);
}
