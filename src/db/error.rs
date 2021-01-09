use std::fmt;

#[derive(Debug)]
pub enum Error {
    Operation,
    NoRecord,
    Convert(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Convert(e) => write!(f, "Conver Error: {}", e),
            Error::NoRecord => write!(f, "NoRecord"),
            Error::Operation => write!(f, "Operation Error"),
        }
    }
}
