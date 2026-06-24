// Lesson 13 — Iterators & Closures

pub fn run() {
    // Iterator adapters: map, filter, collect
    let v = vec![1, 2, 3, 4, 5];
    let squared_evens: Vec<i32> = v
        .iter()
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .cloned()
        .collect();
    println!("squared evens: {:?}", squared_evens);

    // Consumers: sum, product (fold)
    let sum: i32 = v.iter().sum();
    println!("sum: {}", sum);

    let product: i32 = v.iter().fold(1, |acc, &x| acc * x);
    println!("product (fold): {}", product);

    // Closures capturing environment (Fn)
    let factor = 3;
    let multiply = |x: i32| x * factor; // captures factor by reference (Copy here)
    let mapped: Vec<i32> = (1..=5).map(multiply).collect();
    println!("mapped with closure capturing factor: {:?}", mapped);

    // move closure: transfers ownership into closure
    let owned_vec = vec![10, 20, 30];
    let consume = move || {
        println!("owned_vec inside move closure: len={}", owned_vec.len());
        // owned_vec is moved and cannot be used after this point
    };
    consume();

    // Lazy evaluation: nothing computed until consumed
    let lazy = (1..)
        .map(|x| x * x)
        .take(5); // still lazy
    println!("squares (lazy take 5):");
    for n in lazy {
        println!("  {}", n);
    }

    // Iterator adapters are composed without allocations until collect/consume
}

fn main() {
    run();
}
