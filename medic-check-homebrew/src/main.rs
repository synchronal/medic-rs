#![feature(try_trait_v2)]

use medic_check_homebrew::cli::CliArgs;

use std::ops::{ControlFlow, FromResidual, Try};
use std::path::Path;
use std::process::{Command, Stdio};
use CheckResult::{CheckError, CheckOk};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum CheckResult {
    #[default]
    CheckOk,
    CheckError(String, String, String, String),
}

impl std::process::Termination for CheckResult {
    fn report(self) -> std::process::ExitCode {
        match self {
            CheckOk => std::process::ExitCode::from(0),
            CheckError(msg, stdout, stderr, remedy) => {
                eprintln!("{msg}\r\n");
                eprintln!("stdout:\r\n{stdout}");
                eprintln!("stderr:\r\n{stderr}");
                eprintln!("Possible remedy:\r\n{remedy}\r\n");

                std::process::ExitCode::from(1)
            }
        }
    }
}

impl CheckResult {
    pub fn from_std(data: Vec<u8>) -> String {
        String::from_utf8(data).unwrap()
    }
}

pub struct ResultCodeResidual(String, String, String, String);

impl Try for CheckResult {
    type Output = ();
    type Residual = ResultCodeResidual;

    fn branch(self) -> ControlFlow<Self::Residual> {
        match self {
            CheckError(msg, stdout, stderr, remedy) => {
                ControlFlow::Break(ResultCodeResidual(msg, stdout, stderr, remedy))
            }
            CheckOk => ControlFlow::Continue(()),
        }
    }
    fn from_output((): ()) -> Self {
        CheckOk
    }
}

impl FromResidual for CheckResult {
    fn from_residual(r: ResultCodeResidual) -> Self {
        Self::CheckError(r.0, r.1, r.2, r.3)
    }
}

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
