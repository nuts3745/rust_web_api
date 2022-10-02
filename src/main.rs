fn void() {
    
}

fn return_void() {
    return ();
}

fn main() {
    let a = void();
    println!("{:?}", a);
    let a = return_void();
    println!("{:?}", a);
}
