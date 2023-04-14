use medic_check::std_to_string;
use medic_check::CheckResult::{self, CheckError, CheckOk};
use medic_check_hex::cli::{CliArgs, Command};

use std::fs;
use std::process::Command as Cmd;

fn main() -> CheckResult {
    let cli = CliArgs::new();
    mix_installed()?;

    match cli.command {
        Command::ArchiveInstalled(args) => archive_installed(args.name)?,
        Command::LocalHex => local_mix_installed()?,
        // Command::LocalRebar => local_rebar_installed()?,
        Command::PackagesCompiled(args) => packages_compiled(args.cd)?,
        Command::PackagesInstalled(args) => packages_installed(args.cd)?,
    }
    CheckOk
}

fn mix_installed() -> CheckResult {
    match Cmd::new("which").args(["mix"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find mix.".into(),
                    stdout,
                    stderr,
                    "asdf install elixir".into(),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for mix. Is `which` in your PATH?".into(),
            "".into(),
            "".into(),
            "".into(),
        ),
    }
}

fn archive_installed(archive_name: String) -> CheckResult {
    match Cmd::new("mix").args(["archive"]).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                let archive_substr = format!("* {}-", archive_name);
                if stdout.contains(&archive_substr) {
                    CheckOk
                } else {
                    CheckError(
                        format!("Mix archive is not installed."),
                        stdout,
                        stderr,
                        format!("mix archive.install hex {} --force", archive_name),
                    )
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    stdout,
                    stderr,
                    "asdf install elixir".into(),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix archives are installed.".into(),
            "".into(),
            "".into(),
            "asdf install elixir".into(),
        ),
    }
}

fn local_mix_installed() -> CheckResult {
    match Cmd::new("mix").args(["archive"]).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                let archive_substr = format!("* hex-");
                if stdout.contains(&archive_substr) {
                    CheckOk
                } else {
                    CheckError(
                        format!("Mix archive is not installed."),
                        stdout,
                        stderr,
                        format!("mix local.hex --force"),
                    )
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    stdout,
                    stderr,
                    "asdf install elixir".into(),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix archives are installed.".into(),
            "".into(),
            "".into(),
            "asdf install elixir".into(),
        ),
    }
}

// fn local_rebar_installed() -> CheckResult {
// }

fn packages_compiled(cd: String) -> CheckResult {
    let path = fs::canonicalize(&cd).unwrap();
    match Cmd::new("mix").args(["deps"]).current_dir(path).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                if stdout.contains("the dependency build is outdated") {
                    CheckError(
                        format!("Mix deps are not compiled."),
                        stdout,
                        stderr,
                        format!("(cd {} && mix deps.compile)", cd),
                    )
                } else {
                    CheckOk
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    stdout,
                    stderr,
                    "## No suggested remedy".into(),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix packages are installed.".into(),
            "".into(),
            "".into(),
            "asdf install elixir".into(),
        ),
    }
}

fn packages_installed(cd: String) -> CheckResult {
    let path = fs::canonicalize(&cd).unwrap();
    match Cmd::new("mix").args(["deps"]).current_dir(path).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                if stdout.contains("dependency is not available")
                    || stdout.contains("is out of date")
                {
                    CheckError(
                        format!("Mix deps are out of date."),
                        stdout,
                        stderr,
                        format!("(cd {} && mix deps.get)", cd),
                    )
                } else {
                    CheckOk
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    stdout,
                    stderr,
                    "## No suggested remedy".into(),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix packages are installed.".into(),
            "".into(),
            "".into(),
            "asdf install elixir".into(),
        ),
    }
}
