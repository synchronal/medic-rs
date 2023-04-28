#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::CheckResult::{self, CheckError, CheckOk};
use medic_lib::std_to_string;
use std::path::Path;
use std::process::Command;

pub fn brewfile_exists() -> CheckResult {
    let filepath = Path::new("Brewfile");
    if filepath.exists() {
        CheckOk
    } else {
        CheckError(
            "Brewfile does not exist".into(),
            None,
            None,
            Some("touch Brewfile".into()),
        )
    }
}

pub fn bundled() -> CheckResult {
    homebrew_installed()?;
    brewfile_exists()?;
    let filepath = Path::new("Brewfile");
    match Command::new("brew")
        .args([
            "bundle",
            "check",
            "--file",
            filepath.to_path_buf().to_str().unwrap(),
        ])
        .env("HOMEBREW_NO_AUTO_UPDATE", "1")
        .output()
    {
        Ok(command) => match command.status.success() {
            true => CheckOk,
            false => {
                let stdout = std_to_string(command.stdout);
                let stderr = std_to_string(command.stderr);
                CheckError(
                    "Homebrew bundle is out of date.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some(format!("brew bundle --file {filepath:?}")),
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
