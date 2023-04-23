use medic_step::StepResult::{self, StepError, StepOk};
use medic_step_cargo::cli::{CliArgs, Command as Cmd};

use std::process::{Command, Stdio};

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Cmd::Clippy => run_clippy()?,
        Cmd::Test => run_tests()?,
    }
    StepOk
}

fn run_clippy() -> StepResult {
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

fn run_tests() -> StepResult {
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
