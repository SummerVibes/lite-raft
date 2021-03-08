pub type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;

/// custom error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Client disconnected from server")]
    ServerDisconnected,
    #[error("Method not found: {0}")]
    NoSuchMethod(String),
    #[error("unknown error")]
    Unknown,
    #[error(transparent)]
    AllError(#[from] std::io::Error)
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    #[test]
    fn test_error() {
        println!("{:?}", Error::ServerDisconnected)
    }
}