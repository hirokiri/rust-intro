fn get_port(env_var: Option<&str>) -> u16 {
    let default = 8080;

    match env_var {
        Some(port) => port.parse::<u16>().unwrap_or(default),
        None => default,
    }
}

fn main() {
    let port = get_port(None);
    println!("Port: {}", port);

    let port2 = get_port(Some("8081"));
    println!("Port2: {}", port2);

    let port3 = get_port(Some("8082"));
    println!("Port3: {}", port3);

    let port4 = get_port(Some("8083"));
    println!("Port4: {}", port4);

    let port5 = get_port(Some("8084"));
    println!("Port5: {}", port5);

    let port6 = get_port(Some("8085"));
    println!("Port6: {}", port6);

    let port_invalid = get_port(Some("invalid"));
    println!("Port (invalid input): {}", port_invalid);
}