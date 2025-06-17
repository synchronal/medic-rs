#[cfg(test)]
mod step_config_test;

pub mod step_config;
pub use step_config::StepConfig;

use crate::extra;
use crate::noop_config::NoopConfig;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::Check;
use console::style;
use retrogress::Progress;
use serde::Deserialize;
use std::fmt;
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Step {
  Check(Check),
  Shell(ShellConfig),
  Step(StepConfig),
  Doctor(DoctorConfig),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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

  fn platform(&self) -> &Option<Vec<String>> {
    match self {
      Step::Check(config) => config.platform(),
      Step::Doctor(_) => &None,
      Step::Shell(config) => config.platform(),
      Step::Step(config) => config.platform(),
    }
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    match self {
      Step::Check(config) => config.run(progress),
      Step::Doctor(_) => run_doctor(progress),
      Step::Shell(config) => config.run(progress),
      Step::Step(config) => config.run(progress),
    }
  }

  fn to_command(&self) -> Result<Command, Box<dyn std::error::Error>> {
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

fn run_doctor(progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
  let pb = progress.append("doctor");
  progress.println(
    pb,
    &format!("{} {}", style("!").bright().green(), style("==== Doctor ====").cyan()),
  );
  progress.hide(pb);
  if let Ok(result) = doctor_command()
    .unwrap()
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .stdin(Stdio::inherit())
    .output()
  {
    if result.status.success() {
      Recoverable::Ok(())
    } else if result.status.code() == Some(crate::USER_QUIT) {
      std::process::exit(crate::USER_QUIT);
    } else {
      Recoverable::Err(None, None)
    }
  } else {
    Recoverable::Err(Some("Unable to run doctor".into()), None)
  }
}

fn doctor_command() -> Result<Command, Box<dyn std::error::Error>> {
  let command = extra::command::from_string("medic doctor", &None);
  Ok(command)
}
