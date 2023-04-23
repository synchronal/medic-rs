use medic_lib::std_to_string;
use medic_step::StepResult::{self, StepError, StepOk};
use medic_step_cargo::cli::{CliArgs, Command as Cmd};

use std::process::Command;

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Cmd::Clippy => run_clippy()?,
        Cmd::Test => run_tests()?,
    }
    StepOk
}

fn run_clippy() -> StepResult {
    match Command::new("cargo").args(["clippy"]).output() {
        Ok(which) => {
            if which.status.success() {
                StepOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                StepError("Cargo lint error".into(), Some(stdout), Some(stderr))
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
    match Command::new("cargo").args(["test"]).output() {
        Ok(which) => {
            if which.status.success() {
                StepOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                StepError("Cargo test failure".into(), Some(stdout), Some(stderr))
            }
        }
        Err(_err) => StepError(
            "Could not run cargo test. Is `cargo` in PATH?".into(),
            None,
            None,
        ),
    }
}
