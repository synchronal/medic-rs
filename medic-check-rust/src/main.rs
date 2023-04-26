use medic_check_rust::cli::{CliArgs, Command as Cmd};
use medic_lib::std_to_string;
use medic_lib::CheckResult::{self, CheckError, CheckOk};

use regex::Regex;
use std::process::Command;

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Cmd::CrateInstalled(args) => crate_installed(args.name)?,
        Cmd::FormatCheck(_) => check_formatting()?,
        Cmd::TargetInstalled(args) => target_installed(args.target)?,
    }
    CheckOk
}

fn cargo_exists() -> CheckResult {
    match Command::new("which").args(["cargo"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find cargo in PATH.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install rust".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for cargo. Is `which` in your PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

fn check_formatting() -> CheckResult {
    cargo_exists()?;
    match Command::new("cargo").args(["fmt", "--check"]).output() {
        Ok(command) => match command.status.success() {
            true => CheckOk,
            false => CheckError(
                "Rust project is not correctly formatted".into(),
                Some(std_to_string(command.stdout)),
                Some(std_to_string(command.stderr)),
                Some("cargo fmt".into()),
            ),
        },
        Err(_err) => CheckError(
            "Unable to check for rust formatting. Is `cargo` in PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

fn crate_installed(name: String) -> CheckResult {
    cargo_exists()?;
    match Command::new("cargo").args(["install", "--list"]).output() {
        Ok(command) => match command.status.success() {
            true => {
                let pattern = Regex::new(&format!("(?m)^{} v", regex::escape(&name))).unwrap();
                let stdout = std_to_string(command.stdout);
                if pattern.is_match(&stdout) {
                    CheckOk
                } else {
                    CheckError(
                        format!("Rust crate `{name}` does not appear to be installed"),
                        Some(stdout),
                        Some(std_to_string(command.stderr)),
                        Some(format!("cargo install {name}")),
                    )
                }
            }
            false => CheckError(
                "Unable to check for installed crates. Is cargo in PATH?".into(),
                Some(std_to_string(command.stdout)),
                Some(std_to_string(command.stderr)),
                None,
            ),
        },
        Err(_err) => CheckError(
            "Unable to check for installed crates. Is cargo in PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

fn rustup_exists() -> CheckResult {
    match Command::new("which").args(["rustup"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find rustup in PATH.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install rust".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for rustup. Is `which` in your PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

fn target_installed(target: String) -> CheckResult {
    rustup_exists()?;
    match Command::new("rustup").args(["target", "list"]).output() {
        Ok(command) => match command.status.success() {
            true => {
                let pattern =
                    Regex::new(&format!("(?m)^{} \\(installed\\)", regex::escape(&target)))
                        .unwrap();
                let stdout = std_to_string(command.stdout);
                if pattern.is_match(&stdout) {
                    CheckOk
                } else {
                    CheckError(
                        format!("Rust target `{target}` does not appear to be installed"),
                        Some(stdout),
                        Some(std_to_string(command.stderr)),
                        Some(format!("rustup target install {target}")),
                    )
                }
            }
            false => CheckError(
                "Unable to check for installed crates. Is cargo in PATH?".into(),
                Some(std_to_string(command.stdout)),
                Some(std_to_string(command.stderr)),
                None,
            ),
        },
        Err(_err) => CheckError(
            "Unable to check for installed crates. Is cargo in PATH?".into(),
            None,
            None,
            None,
        ),
    }
}
