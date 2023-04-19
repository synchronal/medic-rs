use medic_check::CheckResult::{self, CheckError, CheckOk};
use medic_check_npm::cli::{CliArgs, Command};
use medic_lib::std_to_string;

use std::process::Command as Cmd;

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::Exists => npm_exists()?,
        Command::PackagesInstalled(args) => packages_installed(args.cd, args.prefix)?,
    }
    CheckOk
}

fn npm_exists() -> CheckResult {
    match Cmd::new("which").args(["npm"]).output() {
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

fn packages_installed(cd: Option<String>, prefix: Option<String>) -> CheckResult {
    let mut check = Cmd::new("npm");
    let mut remedy = Cmd::new("npm");
    check.arg("ls").arg("--prefer-offline");
    remedy.arg("install");

    if let Some(path) = cd {
        let expanded = std::fs::canonicalize(path).unwrap();
        check.current_dir(&expanded);
        remedy.current_dir(&expanded);
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
