use medic_lib::std_to_string;
use medic_lib::StepResult::{self, StepError, StepOk};
use medic_step_git::cli::{CliArgs, Command as Cmd};

use std::process::Command;

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Cmd::Pull => run_git_pull()?,
    }
    StepOk
}

fn run_git_pull() -> StepResult {
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
