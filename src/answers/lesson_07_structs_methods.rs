// Lesson 07 — Structs & Methods
// Exercises: Rectangle struct with methods (area, can_hold, square, scale) and User struct construction with field init shorthand and struct update syntax.
// Entry: contains impl blocks and examples for methods and associated functions
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Exercise 7-1: Rectangle methods (area, can_hold, square, scale)
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

pub fn run() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rectangle: {:?}", rectangle);
    println!("area: {}", rectangle.area());

    let smaller = Rectangle {
        width: 10,
        height: 20,
    };
    println!("can hold smaller: {}", rectangle.can_hold(&smaller));

    let square = Rectangle::square(25);
    println!("square: {:?}", square);

    let mut scalable = Rectangle {
        width: 2,
        height: 3,
    };
    scalable.scale(4);
    println!("scaled: {:?}", scalable);

    // Exercise 7-2: User struct construction and struct update syntax
    let user1 = build_user(
        String::from("ferris"),
        String::from("ferris@example.com"),
    );
    let user2 = User {
        email: String::from("new-ferris@example.com"),
        ..user1
    };

    println!("user2: {:?}", user2);
    println!(
        "user2 fields: username={}, email={}, active={}, sign_in_count={}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    run();
}
