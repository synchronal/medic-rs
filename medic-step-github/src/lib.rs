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
                let mut origin = std_to_string(cmd.stdout);
                origin = origin.trim().to_owned();
                let origin_re = Regex::new(r"(git@|https://)([^:]+)[:/](.+)(?:\.git)$").unwrap();
                let caps = origin_re.captures(&origin).unwrap();
                let url = caps.get(2).unwrap().as_str();
                let repository = caps.get(3).unwrap().as_str();

                let github_url = format!("https://{}/{}", url, repository);

                println!("\x1b[93mCheck CI at \x1b[0;1m{github_url}/actions\x1b[0m");

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
