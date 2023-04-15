pub mod config;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
