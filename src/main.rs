fn add_until(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    let mut temp = start;
    loop {
        sum += temp;
        if temp == end {
            break sum;
        }
        temp += 1;
    }
}

fn main() {
    let result = add_until(1, 3);
    println!("{}", result);
}
