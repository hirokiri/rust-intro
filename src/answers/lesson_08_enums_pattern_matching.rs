// Lesson 08 — Enums & Pattern Matching
// Exercises: enums with methods (TrafficLight::duration), complex enums with data (WebEvent), pattern matching, Option handling (plus_one), and parsing with fallback (get_port).
// Entry: enum and functions demonstrating match and pattern matching
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Exercise 8-1: enum methods (TrafficLight::duration)
impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// Exercise 8-2: WebEvent pattern matching and Option handling
pub fn run() {
    for light in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green] {
        println!("light duration: {}", light.duration());
    }

    inspect_event(WebEvent::PageLoad);
    inspect_event(WebEvent::PageUnload);
    inspect_event(WebEvent::KeyPress('a'));
    inspect_event(WebEvent::Paste(String::from("hello")));
    inspect_event(WebEvent::Click { x: 10, y: 20 });

    println!("Some(5) plus one: {:?}", plus_one(Some(5)));
    println!("None plus one: {:?}", plus_one(None));

    println!("port: {}", get_port(Some("8000")));
    println!("port: {}", get_port(Some("invalid")));
    println!("port: {}", get_port(None));
}

fn inspect_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(key) => println!("Key pressed: '{}'", key),
        WebEvent::Paste(text) => println!("Pasted: {}", text),
        WebEvent::Click { x, y } => println!("Click event at x = {}, y = {}", x, y),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}

fn get_port(env_var: Option<&str>) -> u16 {
    match env_var {
        Some(value) => match value.parse::<u16>() {
            Ok(port) => port,
            Err(_) => 8080,
        },
        None => 8080,
    }
}

fn main() {
    run();
}
