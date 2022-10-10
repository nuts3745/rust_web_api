use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("sato"),
        age: 30,
    };

    let serialized = serde_json::to_string(&user).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}