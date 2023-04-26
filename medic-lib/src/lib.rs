#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod audit_step;
pub mod config;
pub mod step;
pub mod step_result;

pub use audit_step::AuditStep;
pub use step::Step;
pub use step_result::StepResult;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}
