#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod check_result;
pub mod step_result;

pub use check_result::CheckResult;
pub use step_result::StepResult;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data)
        .expect("Unable to parse text from STDIO. Output must be valid UTF-8 content.")
}
