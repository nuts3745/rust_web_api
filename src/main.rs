/// xとyを足し合わせます。
///
/// # Examples
///
/// ```
/// use chapter_01::utils::add;
///
/// let r = add(1, 10);
/// assert_eq!(11, r);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", add(1, 2))
}
