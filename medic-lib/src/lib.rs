#![cfg_attr(feature = "strict", deny(warnings))]

pub mod config;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}
