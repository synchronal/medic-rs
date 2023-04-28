#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::std_to_string;
use medic_lib::StepResult::{self, StepError, StepOk};

use std::process::Command;

pub fn run_git_pull() -> StepResult {
    match Command::new("git").args(["pull", "--rebase"]).output() {
        Ok(cmd) => {
            if cmd.status.success() {
                StepOk
            } else {
                let stdout = std_to_string(cmd.stdout);
                let stderr = std_to_string(cmd.stderr);

                StepError("Git pull error".into(), Some(stdout), Some(stderr))
            }
        }
        Err(_err) => StepError(
            "Could not run git pull. Is `git` in PATH?".into(),
            None,
            None,
        ),
    }
}

pub fn run_git_push() -> StepResult {
    match Command::new("git")
        .args(["push", "origin", "HEAD"])
        .output()
    {
        Ok(cmd) => {
            if cmd.status.success() {
                StepOk
            } else {
                let stdout = std_to_string(cmd.stdout);
                let stderr = std_to_string(cmd.stderr);

                StepError("Git push error".into(), Some(stdout), Some(stderr))
            }
        }
        Err(_err) => StepError(
            "Could not run git pull. Is `git` in PATH?".into(),
            None,
            None,
        ),
    }
}
