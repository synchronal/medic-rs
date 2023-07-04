use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use crate::{AppResult, Check};
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

impl Runnable for AuditStep {
    fn allow_failure(&self) -> bool {
        match self {
            AuditStep::Check(config) => config.allow_failure(),
            AuditStep::Shell(config) => config.allow_failure(),
            AuditStep::Step(config) => config.allow_failure(),
        }
    }

    fn run(self) -> AppResult<()> {
        match self {
            AuditStep::Check(config) => config.run(),
            AuditStep::Shell(config) => config.run(),
            AuditStep::Step(config) => config.run(),
        }
    }

    fn to_command(&self) -> Option<Command> {
        match self {
            AuditStep::Check(config) => config.to_command(),
            AuditStep::Shell(config) => config.to_command(),
            AuditStep::Step(config) => config.to_command(),
        }
    }

    fn verbose(&self) -> bool {
        match self {
            AuditStep::Check(config) => config.verbose(),
            AuditStep::Shell(config) => config.verbose(),
            AuditStep::Step(config) => config.verbose(),
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
