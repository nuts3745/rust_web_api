use std::fmt::{self, Display};

struct User {
    name: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user name is {}, age is {}", &self.name, &self.age)
    }
}

impl User {
    fn new(name: String, age: u32) -> Self {
        User { name, age }
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

struct StraightLine(i32, i32);

fn main() {
    let mut user = User::new("Sato".to_string(), 30);
    println!("{}", user);
    user.rename(String::from("Kato"));
    println!("{}", user);
    let line: StraightLine = StraightLine(12, 12);
    println!("{} {}", line.0, line.1);
}
