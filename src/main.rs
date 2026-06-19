#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
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
    println!("--- Lesson 7: Structs & Methods ---");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1: {:#?}", rect1);
    println!("rect1 area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(25);
    println!("square: {:?}, area: {}", square, square.area());

    let mut resizable = Rectangle {
        width: 4,
        height: 8,
    };
    println!("before scale: {:?}", resizable);
    resizable.scale(3);
    println!("after scale: {:?}", resizable);

    let user1 = build_user(
        String::from("hiro"),
        String::from("hiro@example.com"),
    );

    let user2 = User {
        email: String::from("new-hiro@example.com"),
        ..user1
    };

    println!("user2: {:#?}", user2);
    println!("user2 is active: {}", user2.active);
    println!("user2 sign-in count: {}", user2.sign_in_count);
    println!("user2 username: {}", user2.username);
    println!("user2 email: {}", user2.email);
}
