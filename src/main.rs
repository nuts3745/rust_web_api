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
    if let Some(color) = string_to_color_token("red") {
        println!("red");
    }
}
