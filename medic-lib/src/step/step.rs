use crate::runnable::Runnable;
use crate::step::shell_config::ShellConfig;
use crate::step::step_config::StepConfig;
use crate::AppResult;
use serde::Deserialize;
use std::fmt;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Shell(ShellConfig),
    Step(StepConfig),
    Doctor(DoctorConfig),
}

#[derive(Debug, Deserialize)]
pub struct DoctorConfig {}

impl Runnable for Step {
    fn allow_failure(&self) -> bool {
        match self {
            Step::Shell(config) => config.allow_failure(),
            Step::Step(config) => config.allow_failure(),
            Step::Doctor(_) => false,
        }
    }

    fn run(self) -> AppResult<()> {
        match self {
            Step::Shell(config) => config.run(),
            Step::Step(config) => config.run(),
            Step::Doctor(_) => run_doctor(),
        }
    }

    fn to_command(self) -> Option<Command> {
        match self {
            Step::Shell(config) => config.to_command(),
            Step::Step(config) => config.to_command(),
            Step::Doctor(_) => doctor_command(),
        }
    }

    fn verbose(&self) -> bool {
        match self {
            Step::Shell(config) => config.verbose,
            Step::Step(config) => config.verbose,
            Step::Doctor(_) => true,
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Step::Shell(config) => config.fmt(f),
            Step::Step(config) => config.fmt(f),
            Step::Doctor(_) => write!(f, "\x1b[36m== Doctor ===\x1b[0m"),
        }
    }
}

fn run_doctor() -> AppResult<()> {
    if let Ok(result) = doctor_command()
        .unwrap()
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        if result.status.success() {
            println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
            Ok(())
        } else {
            println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
            Err("".into())
        }
    } else {
        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
        Err("Unable to run doctor".into())
    }
}

fn doctor_command() -> Option<Command> {
    let mut command = Command::new("medic");
    command.arg("doctor");
    Some(command)
}
