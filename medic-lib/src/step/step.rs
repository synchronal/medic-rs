use crate::step::shell_config::ShellConfig;
use crate::step::step_config::StepConfig;
use serde::Deserialize;
use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Shell(ShellConfig),
    Step(StepConfig),
    Doctor(DoctorConfig),
}

#[derive(Debug, Deserialize)]
pub struct DoctorConfig {}

impl Step {
    pub fn to_command(self) -> Option<Command> {
        match self {
            Step::Shell(config) => config.to_command(),
            Step::Step(config) => config.to_command(),
            Step::Doctor(_) => doctor_command(),
        }
    }

    pub fn verbose(&self) -> bool {
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

fn doctor_command() -> Option<Command> {
    let mut command = Command::new("medic");
    command.arg("doctor");
    Some(command)
}
