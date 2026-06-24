// Lesson 17 — Macros & Advanced Features

// Simple declarative macro example: adds `!!!` and uppercases the message
macro_rules! shout {
    ($msg:expr) => {
        println!("{}!!!", $msg.to_uppercase());
    };
}

// Variadic macro: prints debug of multiple expressions
macro_rules! dbg_print {
    ( $( $x:expr ),* ) => {
        $( println!("{} = {:?}", stringify!($x), $x); )*
    };
}

// Small helper macro that generates a getter method for a struct field
macro_rules! make_getter {
    ($name:ident, $field:ident, $ty:ty) => {
        impl $name {
            pub fn $field(&self) -> $ty {
                self.$field
            }
        }
    };
}

pub fn run() {
    println!("Lesson 17 — Macros & Advanced Features");

    shout!("hello macros");

    // Use variadic dbg_print
    let a = 1; let b = "two"; let c = vec![3];
    dbg_print!(a, b, c);

    // Demonstrate make_getter macro
    struct Point { x: i32, y: i32 }
    make_getter!(Point, x, i32);
    let p = Point { x: 4, y: 5 };
    println!("point.x() = {}", p.x());

    // Procedural macros would be covered separately (derive, attribute); here show iterators/closures
    let nums = vec![1, 2, 3];
    let doubled: Vec<_> = nums.iter().map(|n| n * 2).collect();
    println!("doubled = {:?}", doubled);
}
