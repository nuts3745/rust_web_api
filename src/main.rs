fn take<T>(_value: T) {
    
}

fn fizz(value: i32) -> String {
    let result = if value % 3 == 0 {
        String::from("fizz")
    } else {
        format!("{}", value)
    };
    let clone_result = result.clone();
    take(clone_result);
    result
}

fn main() {
    println!("{}", fizz(42));
}
