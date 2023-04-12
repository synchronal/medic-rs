#![cfg_attr(feature = "strict", deny(warnings))]

use std::error;

pub mod cli;
pub mod config;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
