fn main() {
    println!("{}", add(12, 21));
}

#[derive(Debug, PartialEq, Eq)]
struct Book {
    name: String
}
/// xとyを足し合わせます。
/// 
/// # Example
/// 
/// ```
/// use chapter_02::utils::add;
/// 
/// let r = add(1, 10);
/// assert_eq!(11, r);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(2, add(1, 1));
    }

    #[test]
    fn book_test() {
        let book1 = Book {
            name: "book name".to_string()
        };
        let book2 = Book {
            name: "book name".to_string()
        };
        assert_eq!(book1, book2);
    }

    #[test]
    fn return_result_test() -> Result<(), ()> {
        Ok(())
    }
}
