use crate::HelloError::{DataError, UnknownError};
use rand::random;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum HelloError {
    UnknownError,
    DataError(String),
}
impl fmt::Display for HelloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnknownError => write!(f, "UnknownError"),
            DataError(s) => write!(f, "DataError: {}", s),
        }
    }
}
impl Error for HelloError {}

fn main() -> Result<(), Box<dyn Error>> {
    if random::<bool>() {
        return Err(Box::new(DataError("Test Error".into())));
    }
    println!("Hello");
    Ok(())
}
