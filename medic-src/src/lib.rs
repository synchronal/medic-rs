#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod app_result;
pub mod audit_step;
pub mod check;
pub mod config;
pub mod doctor_step;
pub mod noop_config;
pub mod outdated;
pub mod runnable;
pub mod shell;
pub mod shipit_step;
pub mod step;
pub mod util;

mod optional_styled;

pub use app_result::AppResult;
pub use audit_step::AuditStep;
pub use check::Check;
pub use doctor_step::DoctorStep;
pub use outdated::OutdatedCheck;
pub use shipit_step::ShipitStep;
pub use step::Step;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data)
        .expect("Unable to parse text from STDIO. Output must be valid UTF-8 content.")
}
