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
        Command::LocalRebar => local_rebar_installed()?,
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
                        "Mix archive is not installed.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some(format!("mix archive.install hex {archive_name} --force")),
                    )
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install elixir".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix archives are installed.".into(),
            None,
            None,
            Some("asdf install elixir".into()),
        ),
    }
}

fn local_mix_installed() -> CheckResult {
    match Cmd::new("mix").args(["archive"]).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                if stdout.contains("* hex-") {
                    CheckOk
                } else {
                    CheckError(
                        "Mix hex archive is not installed.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some("mix local.hex --force".into()),
                    )
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install elixir".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix archives are installed.".into(),
            None,
            None,
            Some("asdf install elixir".into()),
        ),
    }
}

fn local_rebar_installed() -> CheckResult {
    match Cmd::new("mix")
        .args(["local.rebar", "--if-missing"])
        .output()
    {
        Ok(_) => CheckOk,
        Err(_) => CheckError(
            "Unable to install local rebar.".into(),
            None,
            None,
            Some("mix local.rebar".into()),
        ),
    }
}

fn packages_compiled(cd: String) -> CheckResult {
    let path = fs::canonicalize(&cd).unwrap();
    match Cmd::new("mix").args(["deps"]).current_dir(path).output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                if stdout.contains("the dependency build is outdated") {
                    CheckError(
                        "Mix deps are not compiled.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some(format!("(cd {cd} && mix deps.compile)")),
                    )
                } else {
                    CheckOk
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    Some(stdout),
                    Some(stderr),
                    None,
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix packages are installed.".into(),
            None,
            None,
            Some("asdf install elixir".into()),
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
                        "Mix deps are out of date.".into(),
                        Some(stdout),
                        Some(stderr),
                        Some(format!("(cd {cd} && mix deps.get)")),
                    )
                } else {
                    CheckOk
                }
            } else {
                CheckError(
                    "Unable to determine which mix packages are installed.".into(),
                    Some(stdout),
                    Some(stderr),
                    None,
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which mix packages are installed.".into(),
            None,
            None,
            Some("asdf install elixir".into()),
        ),
    }
}
