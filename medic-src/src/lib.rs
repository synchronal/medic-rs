#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod app_result;
pub mod audit;
pub mod check;
pub mod cli;
pub mod config;
pub mod context;
pub mod doctor;
pub mod error;
pub mod extra;
pub mod noop_config;
pub mod outdated;
pub mod recoverable;
pub mod runnable;
pub mod shell;
pub mod shipit;
pub mod step;
pub mod theme;
pub mod util;

mod optional_styled;

pub use app_result::AppResult;
pub use audit::AuditStep;
pub use check::Check;
pub use doctor::DoctorStep;
pub use outdated::OutdatedCheck;
pub use shipit::ShipitStep;
pub use step::Step;

pub const QUIT_STATUS_CODE: i32 = 75;

pub fn std_to_string(data: Vec<u8>) -> String {
  String::from_utf8(data).expect("Unable to parse text from STDIO. Output must be valid UTF-8 content.")
}
