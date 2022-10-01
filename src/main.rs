fn main() {
    let message = if true { "hello, foo" } else { "hello, bar" };
    println!("{}", message);

    let mut scores = std::collections::HashMap::new();
    scores.insert("Sato", 100);
    scores.insert("Tanaka", 90);
    scores.entry("Tanaka").or_insert(100);
    println!("{:?}", scores);

    let solar_distance = std::collections::HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    println!("{:?}", solar_distance);
}
