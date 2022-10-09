async fn something() -> Result<String, String> {
    Err("something failure...".to_string())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let r = something().await?;
    println!("{}", r);
    Ok(())
}
