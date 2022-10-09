fn hello() -> &'static str {
    "hello"
}

fn print_hello<'a>(name: &'a str) {
    println!("hello, {}", name);
}

fn main() {
    let a: &'static str = "some text";
    println!("{}", hello());
    print_hello(a);
}
