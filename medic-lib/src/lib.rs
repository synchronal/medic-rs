#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod audit_step;
pub mod check;
pub mod config;
pub mod step;

pub use audit_step::AuditStep;
pub use check::Check;
pub use check::CheckResult;
pub use step::step::Step;
pub use step::StepResult;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}
