#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;
pub mod deps;

use cli::app::MixArgs;
use medic_lib::std_to_string;
use medic_lib::StepResult::{self, StepError, StepOk};
use std::fs;
use std::process::{Command, Stdio};

pub fn mix_installed() -> StepResult {
    match Command::new("which").args(["mix"]).output() {
        Ok(which) => {
            if which.status.success() {
                StepOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                StepError("Unable to find mix.".into(), Some(stdout), Some(stderr))
            }
        }
        Err(_err) => StepError(
            "Unable to search for mix. Is `which` in your PATH?".into(),
            None,
            None,
        ),
    }
}

pub fn mix_project_exists(path: &String) -> StepResult {
    if let Ok(expanded) = fs::canonicalize(path) {
        let mix_exs = expanded.join("mix.exs");
        if mix_exs.exists() {
            StepOk
        } else {
            StepError(
                "Could not find mix project. Please run from a directory with a mix.exs file."
                    .into(),
                None,
                Some(format!("Expected file: {mix_exs:?}")),
            )
        }
    } else {
        StepError(
            "Could not find mix project. Path does not exist.".into(),
            None,
            Some(format!("Expected path: `{path}/mix.exs`")),
        )
    }
}

pub fn compile_deps(args: MixArgs) -> StepResult {
    mix_installed()?;
    mix_project_exists(&args.cd)?;
    match parse_deps(&args) {
        Ok(deps) => {
            let outdated: Vec<&deps::Dep> = deps
                .iter()
                .filter(|d| d.status == deps::DepStatus::Outdated)
                .collect();

            if outdated.is_empty() {
                StepOk
            } else {
                let path = fs::canonicalize(&args.cd).unwrap();
                let mut command = Command::new("mix");
                command
                    .arg("deps.compile")
                    .current_dir(path)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());

                if let Some(mix_env) = &args.mix_env {
                    command.env("MIX_ENV", mix_env);
                };

                for dep in outdated {
                    command.arg(dep.name.clone());
                }

                match command.status() {
                    Ok(status) => {
                        if status.success() {
                            StepOk
                        } else {
                            StepError("Unable to compile deps.".into(), None, None)
                        }
                    }
                    Err(_) => StepError("Unable to compile deps.".into(), None, None),
                }
            }
        }
        Err(err) => StepError(
            "Unable to check for uncompiled deps.".into(),
            None,
            Some(format!("{}", err)),
        ),
    }
}

fn parse_deps(args: &MixArgs) -> Result<Vec<deps::Dep>, Box<dyn std::error::Error>> {
    let path = fs::canonicalize(&args.cd).unwrap();

    let mut command = Command::new("mix");
    command.arg("deps").current_dir(path);
    if let Some(mix_env) = &args.mix_env {
        command.env("MIX_ENV", mix_env);
    };

    let output = command.output()?;
    if output.status.success() {
        let stdout = std_to_string(output.stdout);
        deps::Dep::from_deps_output(stdout)
    } else {
        Err(std_to_string(output.stderr).into())
    }
}

pub fn get_deps(args: MixArgs) -> StepResult {
    mix_installed()?;
    mix_project_exists(&args.cd)?;
    let path = fs::canonicalize(&args.cd).unwrap();
    match Command::new("mix")
        .args(["deps.get"])
        .current_dir(&path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                StepOk
            } else {
                StepError(
                    "Mix was unable get deps.".into(),
                    Some(stdout),
                    Some(stderr),
                )
            }
        }
        Err(_err) => StepError("Unable to get deps.".into(), None, None),
    }
}

pub fn run_credo(args: MixArgs) -> StepResult {
    mix_installed()?;
    mix_project_exists(&args.cd)?;
    let path = fs::canonicalize(&args.cd).unwrap();
    match Command::new("mix")
        .args(["credo", "--strict"])
        .current_dir(&path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                StepOk
            } else {
                StepError(
                    "Credo has detected errors.".into(),
                    Some(stdout),
                    Some(stderr),
                )
            }
        }
        Err(_err) => StepError("Unable to run credo.".into(), None, None),
    }
}

pub fn run_dialyzer(args: MixArgs) -> StepResult {
    mix_installed()?;
    mix_project_exists(&args.cd)?;
    let path = fs::canonicalize(&args.cd).unwrap();
    match Command::new("mix")
        .args(["dialyzer"])
        .current_dir(&path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                StepOk
            } else {
                StepError(
                    "Dialyzer has detected errors.".into(),
                    Some(stdout),
                    Some(stderr),
                )
            }
        }
        Err(_err) => StepError("Unable to run dialyzer.".into(), None, None),
    }
}

pub fn run_mix_audit(args: MixArgs) -> StepResult {
    mix_installed()?;
    mix_project_exists(&args.cd)?;
    let path = fs::canonicalize(&args.cd).unwrap();
    match Command::new("mix")
        .args(["deps.audit"])
        .current_dir(&path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() {
                StepOk
            } else {
                StepError(
                    "Vulnerabilities have been detected in deps.".into(),
                    Some(stdout),
                    Some(stderr),
                )
            }
        }
        Err(_err) => StepError("Unable to run deps.audit.".into(), None, None),
    }
}
