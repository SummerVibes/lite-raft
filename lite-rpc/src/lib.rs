pub mod client;
pub mod server;
pub mod error;

/// all functions use this custom Result
pub use error::Result;

pub use error::Error;

pub type Id = u32;