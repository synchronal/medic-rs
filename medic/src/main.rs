#![cfg_attr(feature = "strict", deny(warnings))]

use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> AppResult<()> {
    println!("Hello, world!");

    Ok(())
}
