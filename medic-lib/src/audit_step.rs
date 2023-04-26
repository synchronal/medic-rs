use crate::step::{ShellConfig, StepConfig};
use crate::Check;
use serde::Deserialize;

use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AuditStep {
    Check(Check),
    Shell(ShellConfig),
    Step(StepConfig),
}

impl AuditStep {
    pub fn to_command(self) -> Option<Command> {
        match self {
            AuditStep::Check(config) => Some(config.to_command()),
            AuditStep::Shell(config) => config.to_command(),
            AuditStep::Step(config) => config.to_command(),
        }
    }

    pub fn verbose(&self) -> bool {
        match self {
            AuditStep::Check(config) => config.verbose,
            AuditStep::Shell(config) => config.verbose,
            AuditStep::Step(config) => config.verbose,
        }
    }
}

impl fmt::Display for AuditStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuditStep::Check(config) => config.fmt(f),
            AuditStep::Shell(config) => config.fmt(f),
            AuditStep::Step(config) => config.fmt(f),
        }
    }
}
