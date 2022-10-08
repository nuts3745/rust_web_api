fn always_error() -> Result<(), String> {
    Err("常にエラーが発生します".to_string())
}

fn might_fail() -> Result<(), String> {
    let _result = always_error()?;
    Ok(())
}

fn main() {
    let message = match might_fail() {
        Ok(_) => "処理に成功しました".to_string(),
        Err(cause_message) => cause_message,
    };
    println!("{}", message);
}
