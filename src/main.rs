fn hello_print(message: &str) {
    println!("hello, {}", message);
}

fn print(value: i32) {
    println!("{}", value);
}

fn main() {
    let world = String::from("world");
    hello_print(&world);
    println!("{}", world);
    let a = 999;
    let b = &a;
    print(*b);
}
