#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod check;
pub mod check_result;

pub use check::Check;
pub use check_result::CheckResult;
