fn main() {
    let some: Result<&str, &str> = Ok("ok");
    println!("{:?}", some);
    let err: Result<&str, &str> = Err("err");
    println!("{:?}", err);
}
