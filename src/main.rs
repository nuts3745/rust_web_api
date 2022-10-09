struct User {
    name: String,
    age: u32,
}

struct StraightLine(i32, i32);

fn main() {
    let user = User{
        name: "Sato".to_string(),
        age: 30,
    };
    println!("name is {}, age is {}", user.name, user.age);
    let line: StraightLine = StraightLine(12, 12);
    println!("{} {}", line.0, line.1);
}
