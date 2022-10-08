fn fizz_buzz(value: i32) -> String {
    let result = if value % 15 == 0 {
        "fizz buzz".to_string()
    } else if value % 5 == 0 {
        "buzz".to_string()
    } else if value % 3 == 0 {
        "fizz".to_string()
    } else {
        value.to_string()
    };
    result
}

fn main() {
    println!("{}", fizz_buzz(1));
    println!("{}", fizz_buzz(3));
    println!("{}", fizz_buzz(5));
}
