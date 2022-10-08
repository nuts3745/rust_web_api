fn fizz_buzz(value: i32) -> String {
    let result = match value {
        v if v % 15 == 0 => "fizz buzz".to_string(),
        v if v % 5 == 0 => "buzz".to_string(),
        v if v % 3 == 0 => "fizz".to_string(),
        _ => value.to_string(),
    };
    result
}

enum Color {
    Red,
    Blue,
    Green,
    Hex(String),
}

fn string_to_color_token(value: &str) -> Option<Color> {
    match value {
        "red" => Some(Color::Red),
        "blue" => Some(Color::Blue),
        "green" => Some(Color::Green),
        "white" => Some(Color::Hex("f8f7f2".to_string())),
        _ => None,
    }
}

fn main() {
    println!("{}", fizz_buzz(1));
    println!("{}", fizz_buzz(3));
    println!("{}", fizz_buzz(5));
    let result = string_to_color_token("red").unwrap();
}
