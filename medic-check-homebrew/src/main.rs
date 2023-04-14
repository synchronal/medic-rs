use medic_check::CheckResult::{self, CheckError, CheckOk};
use medic_check_homebrew::cli::CliArgs;

use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> CheckResult {
    let _cli_args = CliArgs::new();
    homebrew_installed()?;
    brewfile_exists()?;
    bundled()?;
    CheckOk
}

fn brewfile_exists() -> CheckResult {
    let filepath = Path::new("Brewfile");
    if filepath.exists() {
        CheckOk
    } else {
        CheckError(
            "Brewfile does not exist".into(),
            "".into(),
            "".into(),
            "touch Brewfile".into(),
        )
    }
}

fn bundled() -> CheckResult {
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
        Ok(status) => match status.status.success() {
            true => CheckOk,
            false => {
                let stdout = CheckResult::from_std(status.stdout);
                let stderr = CheckResult::from_std(status.stderr);
                let remedy = format!("brew bundle --file {filepath:?}");
                CheckError(
                    "Homebrew bundle is out of date.".into(),
                    stdout,
                    stderr,
                    remedy,
                )
            }
        },
        Err(err) => {
            let msg =
                format!("Unable to determine if Brewfile is up to date.\r\nOutput:\r\n{err:?}");
            CheckError(msg, "".into(), "".into(), "".into())
        }
    }
}

fn homebrew_installed() -> CheckResult {
    match Command::new("which")
        .args(["brew"])
        .stdout(Stdio::null())
        .output()
    {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = CheckResult::from_std(which.stdout);
                let stderr = CheckResult::from_std(which.stderr);
                CheckError("Unable to find homebrew".into(),
                stdout,
                stderr,
            "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"".into()
                            )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for homebrew".into(),
            "".into(),
            "".into(),
            "".into(),
        ),
    }
}
