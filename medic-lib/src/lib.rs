#![cfg_attr(feature = "strict", deny(warnings))]

pub mod audit_step;
pub mod config;

pub use audit_step::AuditStep;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}
