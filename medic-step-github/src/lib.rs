#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use cli::app::GithubActionsArgs;
use medic_lib::std_to_string;
use medic_lib::StepResult::{self, StepError, StepOk};
use regex::Regex;

use std::process::Command;

pub fn link_to_actions(args: GithubActionsArgs) -> StepResult {
    match Command::new("git")
        .args(["remote", "get-url", &args.remote])
        .output()
    {
        Ok(cmd) => {
            if cmd.status.success() {
                let mut stdout = std_to_string(cmd.stdout);
                stdout = stdout.trim().replace(':', "/").replace("git@", "https://");
                let re = Regex::new(r"\.git$").unwrap();
                let origin = re.replace_all(&stdout, "");
                println!("\x1b[93mCheck CI at \x1b[0;1m{origin}/actions\x1b[0m");

                StepOk
            } else {
                let stdout = std_to_string(cmd.stdout);
                let stderr = std_to_string(cmd.stderr);

                StepError("Git remote error".into(), Some(stdout), Some(stderr))
            }
        }
        Err(_err) => StepError(
            "Could not run `git remote get-url`. Is `git` in PATH?".into(),
            None,
            None,
        ),
    }
}
