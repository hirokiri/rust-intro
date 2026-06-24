/// Sandbox runner replaced with a minimal echo app.
/// When main delegates to sandbox, this prints "hello world!!!".

pub fn run_from_args(_args: &[String]) {
    println!("Hello World!!!");
}
