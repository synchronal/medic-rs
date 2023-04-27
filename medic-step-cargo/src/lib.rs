#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::StepResult::{self, StepError, StepOk};
use std::process::{Command, Stdio};

pub fn run_clippy() -> StepResult {
    match Command::new("cargo")
        .args(["clippy"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(which) => {
            if which.status.success() {
                StepOk
            } else {
                StepError("Cargo lint error".into(), None, None)
            }
        }
        Err(_err) => StepError(
            "Could not run cargo clippy. Is `cargo` in PATH?".into(),
            None,
            None,
        ),
    }
}

pub fn run_tests() -> StepResult {
    match Command::new("cargo")
        .args(["test"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(which) => {
            if which.status.success() {
                StepOk
            } else {
                StepError("Cargo test failure".into(), None, None)
            }
        }
        Err(_err) => StepError(
            "Could not run cargo test. Is `cargo` in PATH?".into(),
            None,
            None,
        ),
    }
}
