#![feature(try_trait_v2)]

use medic_check_homebrew::cli::CliArgs;

use std::ops::{ControlFlow, FromResidual, Try};
use std::process::{Command, Stdio};
use CheckResult::{CheckError, CheckOk};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum CheckResult {
    #[default]
    CheckOk,
    CheckError(String, String),
}

impl std::process::Termination for CheckResult {
    fn report(self) -> std::process::ExitCode {
        match self {
            CheckOk => std::process::ExitCode::from(0),
            CheckError(output, remedy) => {
                eprintln!("HomeBrew bundle check failed!");
                eprintln!("{output}\n");
                eprintln!("Possible remedy:");
                eprintln!("{remedy}\n");

                std::process::ExitCode::from(1)
            }
        }
    }
}

pub struct ResultCodeResidual(String, String);

impl Try for CheckResult {
    type Output = ();
    type Residual = ResultCodeResidual;

    fn branch(self) -> ControlFlow<Self::Residual> {
        match self {
            CheckError(output, remedy) => ControlFlow::Break(ResultCodeResidual(output, remedy)),
            CheckOk => ControlFlow::Continue(()),
        }
    }
    fn from_output((): ()) -> Self {
        CheckOk
    }
}

impl FromResidual for CheckResult {
    fn from_residual(r: ResultCodeResidual) -> Self {
        Self::CheckError(r.0, r.1)
    }
}

// This should return a CheckResult, which should exit 0 when all checks pass.
// When checks fail, some output should be printed to stderr and the remedy
// should be printed to stdout and put into the clipboard.

fn main() -> CheckResult {
    let _cli_args = CliArgs::new();
    homebrew_installed()?;
    brewfile_exists()?;
    bundled()?;
    CheckOk
}

fn brewfile_exists() -> CheckResult {
    CheckOk
}

fn bundled() -> CheckResult {
    CheckOk
}

fn homebrew_installed() -> CheckResult {
    if let Ok(which) = Command::new("which")
        .args(["brew"])
        .stdout(Stdio::null())
        .status()
    {
        if which.success() {
            CheckOk
        } else {
            CheckError("Unable to find homebrew".into(),
            "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"".into())
        }
    } else {
        CheckError("Unable to search for homebrew".into(), "".into())
    }
}
