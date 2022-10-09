struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn description(&self) -> String {
        format!("user name is {}, age is {}", &self.name, &self.age)
    }
}

struct StraightLine(i32, i32);

fn main() {
    let mut user = User::new("Sato".to_string(), 30);
    println!("{}", user.description());
    user.rename(String::from("Kato"));
    println!("{}", user.description());
    let line: StraightLine = StraightLine(12, 12);
    println!("{} {}", line.0, line.1);
}
