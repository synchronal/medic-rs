#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2, try_trait_v2_residual)]

pub mod check_result;
pub mod step_result;

pub use check_result::CheckResult;
pub use step_result::StepResult;

pub fn std_to_string(data: Vec<u8>) -> String {
  String::from_utf8_lossy(&data).into_owned()
}
