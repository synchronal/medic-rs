pub mod step_config;

pub use step_config::StepConfig;

use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::{AppResult, Check};
use serde::Deserialize;
use std::fmt;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Check(Check),
    Shell(ShellConfig),
    Step(StepConfig),
    Doctor(DoctorConfig),
}

#[derive(Debug, Deserialize)]
pub struct NoopConfig {}

#[derive(Debug, Deserialize)]
pub struct DoctorConfig {
    pub doctor: NoopConfig,
}

impl Runnable for Step {
    fn allow_failure(&self) -> bool {
        match self {
            Step::Check(config) => config.allow_failure(),
            Step::Doctor(_) => false,
            Step::Shell(config) => config.allow_failure(),
            Step::Step(config) => config.allow_failure(),
        }
    }

    fn run(self) -> AppResult<()> {
        match self {
            Step::Check(config) => config.run(),
            Step::Doctor(_) => run_doctor(),
            Step::Shell(config) => config.run(),
            Step::Step(config) => config.run(),
        }
    }

    fn to_command(&self) -> Option<Command> {
        match self {
            Step::Check(config) => config.to_command(),
            Step::Doctor(_) => doctor_command(),
            Step::Shell(config) => config.to_command(),
            Step::Step(config) => config.to_command(),
        }
    }

    fn verbose(&self) -> bool {
        match self {
            Step::Check(config) => config.verbose,
            Step::Doctor(_) => true,
            Step::Shell(config) => config.verbose,
            Step::Step(config) => config.verbose,
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Step::Check(config) => config.fmt(f),
            Step::Doctor(_) => write!(f, "\x1b[36m== Doctor ===\x1b[0m"),
            Step::Shell(config) => config.fmt(f),
            Step::Step(config) => config.fmt(f),
        }
    }
}

fn run_doctor() -> AppResult<()> {
    print!("\x1b[32m! \x1b[0");
    println!("\x1b[36;1m==== Doctor ====\x1b[0m");
    if let Ok(result) = doctor_command()
        .unwrap()
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        if result.status.success() {
            AppResult::Ok(())
        } else {
            AppResult::Err(None)
        }
    } else {
        AppResult::Err(Some("Unable to run doctor".into()))
    }
}

fn doctor_command() -> Option<Command> {
    let mut command = Command::new("medic");
    command.arg("doctor");
    Some(command)
}
