#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::std_to_string;
use medic_lib::CheckResult::{self, CheckError, CheckOk};

use std::fs;
use std::path::Path;
use std::process::Command;

pub fn mix_installed() -> CheckResult {
    match Command::new("which").args(["mix"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find mix.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install elixir".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for mix. Is `which` in your PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn mix_project_exists(path: &Path) -> CheckResult {
    let mix_exs = path.join("mix.exs");
    if mix_exs.exists() {
        CheckOk
    } else {
        CheckError(
            "Could not find mix project. Please run from a directory with a mix.exs file".into(),
            Some(format!("Expected file: {mix_exs:?}")),
            None,
            None,
        )
    }
}

pub fn check_unused_deps(cd: String) -> CheckResult {
    mix_installed()?;
    if let Ok(path) = fs::canonicalize(cd) {
        mix_project_exists(&path)?;
        match Command::new("mix")
            .args(["deps.unlock", "--check-unused"])
            .current_dir(&path)
            .output()
        {
            Ok(output) => {
                let stdout = std_to_string(output.stdout);
                let stderr = std_to_string(output.stderr);
                if output.status.success() {
                    CheckOk
                } else {
                    CheckError(
                        "Unused dependencies detected.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some(format!("(cd {path:?} && mix deps.unlock --unused)")),
                    )
                }
            }
            Err(_err) => CheckError("Unable to check for unused deps.".into(), None, None, None),
        }
    } else {
        CheckError(
            "Given a `cd` param to a directory that does not exist.".into(),
            None,
            None,
            None,
        )
    }
}
