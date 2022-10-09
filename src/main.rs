#[derive(PartialEq, Eq)]
struct Point {
    value: f64
}

fn main() {
    let p1 = Point {
        value: f64::NAN,
    };
    let nan = f64::NAN;
    assert!(nan != nan);
    assert!(p1 == p1);
}