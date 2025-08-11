use crate::error::MedicError;
use crate::recoverable::Recoverable;
pub use crate::shell::ShellConfig;
pub use crate::step::step_config::StepConfig;

use crate::runnable::Runnable;
use crate::Check;
use serde::Deserialize;
use std::fmt;
use std::process::Command;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum DoctorStep {
  Check(Check),
  Shell(ShellConfig),
  Step(StepConfig),
}

impl Runnable for DoctorStep {
  fn allow_failure(&self) -> bool {
    match self {
      DoctorStep::Check(config) => config.allow_failure(),
      DoctorStep::Shell(config) => config.allow_failure(),
      DoctorStep::Step(config) => config.allow_failure(),
    }
  }

  fn platform(&self) -> &Option<Vec<String>> {
    match self {
      DoctorStep::Check(config) => config.platform(),
      DoctorStep::Shell(config) => config.platform(),
      DoctorStep::Step(config) => config.platform(),
    }
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    match self {
      DoctorStep::Check(config) => config.run(progress),
      DoctorStep::Shell(config) => config.run(progress),
      DoctorStep::Step(config) => config.run(progress),
    }
  }

  fn to_command(&self) -> Result<Command, MedicError> {
    match self {
      DoctorStep::Check(config) => config.to_command(),
      DoctorStep::Shell(config) => config.to_command(),
      DoctorStep::Step(config) => config.to_command(),
    }
  }

  fn verbose(&self) -> bool {
    match self {
      DoctorStep::Check(config) => config.verbose,
      DoctorStep::Shell(config) => config.verbose,
      DoctorStep::Step(config) => config.verbose,
    }
  }
}

impl fmt::Display for DoctorStep {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DoctorStep::Check(config) => config.fmt(f),
      DoctorStep::Shell(config) => config.fmt(f),
      DoctorStep::Step(config) => config.fmt(f),
    }
  }
}
