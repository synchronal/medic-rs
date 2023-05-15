#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::std_to_string;
use medic_lib::CheckResult::{self, CheckError, CheckOk};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn brewfile_exists(path: &Path) -> CheckResult {
    let filepath = &path.join("Brewfile");
    if filepath.exists() {
        CheckOk
    } else {
        let mut remedy = Command::new("touch");
        remedy.arg("Brewfile").current_dir(path);
        CheckError(
            "Brewfile does not exist".into(),
            None,
            None,
            Some(format!("({remedy:?})").replace('"', "")),
        )
    }
}

pub fn bundled(cd: Option<String>) -> CheckResult {
    homebrew_installed()?;

    let mut command = Command::new("brew");
    let mut remedy = Command::new("brew");
    command
        .args(["bundle", "check"])
        .env("HOMEBREW_NO_AUTO_UPDATE", "1");
    remedy.args(["bundle"]);

    if let Some(path) = cd {
        if let Ok(expanded) = fs::canonicalize(path) {
            command.current_dir(&expanded);
            remedy.current_dir(&expanded);
            brewfile_exists(&expanded)?;
        } else {
            return CheckError(
                "Given a `cd` param to a directory that does not exist.".into(),
                None,
                None,
                None,
            );
        }
    } else {
        brewfile_exists(&std::env::current_dir().unwrap())?;
    }

    match command.env("HOMEBREW_NO_AUTO_UPDATE", "1").output() {
        Ok(command) => match command.status.success() {
            true => CheckOk,
            false => {
                let stdout = std_to_string(command.stdout);
                let stderr = std_to_string(command.stderr);
                CheckError(
                    "Homebrew bundle is out of date.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some(format!("({remedy:?})").replace('"', "")),
                )
            }
        },
        Err(err) => {
            let msg =
                format!("Unable to determine if Brewfile is up to date.\r\nOutput:\r\n{err:?}");
            CheckError(msg, None, None, None)
        }
    }
}

pub fn homebrew_installed() -> CheckResult {
    match Command::new("which").args(["brew"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find homebrew".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"".into())
                )
            }
        }
        Err(_err) => CheckError("Unable to search for homebrew".into(), None, None, None),
    }
}
