fn main() {
    let input: Result<&str, &str> = Ok("test");
    let input = input.unwrap();
    println!("{:?}", input);
}
