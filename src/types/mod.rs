use crate::utility::Error;

pub mod cache;
pub mod context;
pub mod database;
pub type Result<T> = std::result::Result<T, Error>;
