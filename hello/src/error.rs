use std::io::Error;

pub type HelloResult<T> = Result<T, HelloError>;

pub enum HelloError {
    ClientConnectionClosed,
}

impl From<Error> for HelloError {
    fn from(_: Error) -> Self {
        Self::ClientConnectionClosed
    }
}
