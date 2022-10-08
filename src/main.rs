fn add(x: i32, y: i32) -> i32 {
    x + y
}

enum Color {
    Red,
    Blue,
    Green,
    Hex(String),
}

fn main() {
    println!("{}", add(1, 2));
    let a = if true { 10 } else { 20 };
    let b = { 10 * 2 };
    println!("{} {}", a, b);
    let red = Color::Red;
    let hex = Color::Hex("ffffff".to_string());
}
