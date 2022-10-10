use std::{fmt, error, fs::File};

#[derive(Debug)]
enum ApiError {
    InternalServerError(String),
    NotFound,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ApiError]")
    }
}

impl error::Error for ApiError {
    
}

fn fetch_api() -> Result<(), ApiError> {
    Err(ApiError::InternalServerError("[always_my_error]".to_string()))
}

fn maybe_fail() -> Result<(), Box<dyn error::Error>> {
    let _r = fetch_api()?;
    let _f = File::open("hoge.txt")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let _l = maybe_fail()?;
    Ok(())
}