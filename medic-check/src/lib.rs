#![feature(try_trait_v2)]

pub mod result;
pub use result::CheckResult;

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}
