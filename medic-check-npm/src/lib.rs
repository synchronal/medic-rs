#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::std_to_string;
use medic_lib::CheckResult::{self, CheckError, CheckOk};
use std::process::Command;

pub fn npm_exists() -> CheckResult {
    match Command::new("which").args(["npm"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find npm.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install nodejs".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for npm. Is `which` in your PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn packages_installed(cd: Option<String>, prefix: Option<String>) -> CheckResult {
    let mut check = Command::new("npm");
    let mut remedy = Command::new("npm");
    check.arg("ls").arg("--prefer-offline");
    remedy.arg("install");

    if let Some(path) = cd {
        if let Ok(expanded) = std::fs::canonicalize(path) {
            check.current_dir(&expanded);
            remedy.current_dir(&expanded);
        } else {
            return CheckError(
                "Given a `cd` param to a directory that does not exist.".into(),
                None,
                None,
                None,
            );
        }
    }
    if let Some(path) = prefix {
        check.arg("--prefix").arg(&path);
        remedy.arg("--prefix").arg(&path);
    }

    match check.output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() && !stdout.contains("UNMET DEPENDENCY") {
                CheckOk
            } else {
                CheckError(
                    "NPM dependencies out of date.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some(format!("({remedy:?})")),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which NPM packages are installed.".into(),
            None,
            None,
            Some("asdf install nodejs".into()),
        ),
    }
}
