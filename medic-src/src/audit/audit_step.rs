use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use crate::Check;
use serde::Deserialize;

use std::fmt;
use std::process::Command;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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

  fn platform(&self) -> &Option<Vec<String>> {
    match self {
      AuditStep::Check(config) => config.platform(),
      AuditStep::Shell(config) => config.platform(),
      AuditStep::Step(config) => config.platform(),
    }
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    match self {
      AuditStep::Check(config) => config.run(progress),
      AuditStep::Shell(config) => config.run(progress),
      AuditStep::Step(config) => config.run(progress),
    }
  }

  fn to_command(&self) -> Result<Command, Box<dyn std::error::Error>> {
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
