fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", add(1, 2));
    let a = if true { 10 } else { 20 };
    let b = { 10 * 2 };
    println!("{} {}", a, b);
}
