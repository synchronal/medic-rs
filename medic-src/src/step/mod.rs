#[cfg(test)]
mod step_config_test;

pub mod step_config;
pub use step_config::StepConfig;

use crate::cli::Flags;
use crate::error::MedicError;
use crate::extra;
use crate::noop_config::NoopConfig;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::theme::current_theme;
use crate::Check;
use console::style;
use serde::Deserialize;
use std::fmt;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Step {
  Check(Check),
  Shell(ShellConfig),
  Step(StepConfig),
  Doctor(DoctorConfig),
  Steps(Vec<Step>),
}

impl Runnable for Step {
  fn allow_failure(&self) -> bool {
    match self {
      Step::Check(config) => config.allow_failure(),
      Step::Doctor(_) => false,
      Step::Shell(config) => config.allow_failure(),
      Step::Step(config) => config.allow_failure(),
      Step::Steps(_) => false,
    }
  }

  fn platform(&self) -> &Option<Vec<String>> {
    match self {
      Step::Check(config) => config.platform(),
      Step::Doctor(_) => &None,
      Step::Shell(config) => config.platform(),
      Step::Step(config) => config.platform(),
      Step::Steps(_) => &None,
    }
  }

  fn run(self, progress: &mut retrogress::ProgressBar, flags: &Flags) -> Recoverable<()> {
    match self {
      Step::Check(config) => config.run(progress, flags),
      Step::Doctor(config) => config.run(progress, flags),
      Step::Shell(config) => config.run(progress, flags),
      Step::Step(config) => config.run(progress, flags),
      Step::Steps(steps) => {
        if flags.parallel {
          run_parallel_steps(steps, progress, flags)
        } else {
          run_serial_steps(steps, progress, flags)
        }
      }
    }
  }

  fn to_command(&self) -> Result<Command, MedicError> {
    match self {
      Step::Check(config) => config.to_command(),
      Step::Doctor(config) => config.to_command(),
      Step::Shell(config) => config.to_command(),
      Step::Step(config) => config.to_command(),
      Step::Steps(_) => Err(MedicError::Message(
        "Steps cannot be converted to a single command".to_string(),
      )),
    }
  }

  fn verbose(&self) -> bool {
    match self {
      Step::Check(config) => config.verbose,
      Step::Doctor(_) => true,
      Step::Shell(config) => config.verbose,
      Step::Step(config) => config.verbose,
      Step::Steps(_) => false,
    }
  }
}

impl fmt::Display for Step {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Step::Check(config) => config.fmt(f),
      Step::Doctor(config) => config.fmt(f),
      Step::Shell(config) => config.fmt(f),
      Step::Step(config) => config.fmt(f),
      Step::Steps(_) => write!(
        f,
        "{}",
        OptionalStyled::new("== Nested Steps ==", current_theme().text_style.clone())
      ),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DoctorConfig {
  pub doctor: NoopConfig,
}

impl Runnable for DoctorConfig {
  fn run(self, progress: &mut retrogress::ProgressBar, _flags: &Flags) -> Recoverable<()> {
    let pb = progress.append("doctor");
    progress.println(
      pb,
      &format!(
        "{} {}",
        style("!").bright().green(),
        OptionalStyled::new("== Doctor ==", current_theme().text_style.clone())
      ),
    );
    progress.hide(pb);
    if let Ok(result) = self
      .to_command()
      .unwrap()
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .stdin(Stdio::inherit())
      .output()
    {
      if result.status.success() {
        Recoverable::Ok(())
      } else if result.status.code() == Some(crate::QUIT_STATUS_CODE) {
        std::process::exit(crate::QUIT_STATUS_CODE);
      } else {
        Recoverable::Err(None, None)
      }
    } else {
      Recoverable::Err(Some("Unable to run doctor".into()), None)
    }
  }

  fn to_command(&self) -> Result<std::process::Command, MedicError> {
    let command = extra::command::from_string("medic doctor", &None);
    Ok(command)
  }
}

impl std::fmt::Display for DoctorConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      OptionalStyled::new("== Doctor ==", current_theme().text_style.clone())
    )
  }
}

fn run_parallel_steps(steps: Vec<Step>, progress: &mut retrogress::ProgressBar, flags: &Flags) -> Recoverable<()> {
  let (tx, rx) = mpsc::channel();

  thread::scope(|s| {
    for step in steps {
      let mut progress = progress.clone();
      let tx = tx.clone();

      s.spawn(move || {
        let result = step.run(&mut progress, flags);
        let _ = tx.send(result);
      });
    }

    drop(tx);
  });

  let mut failure = None;
  let mut manual = None;
  let mut optional = None;

  while let Ok(result) = rx.recv() {
    match result {
      Recoverable::Manual(_, _) => manual = Some(result),
      Recoverable::Ok(_) => {}
      Recoverable::Optional(_, _) => optional = Some(result),
      Recoverable::Err(_, _) => failure = Some(result),
    }
  }

  if let Some(failure) = failure {
    return failure;
  }
  if let Some(manual) = manual {
    return manual;
  }
  if let Some(optional) = optional {
    return optional;
  }
  Recoverable::Ok(())
}

fn run_serial_steps(steps: Vec<Step>, progress: &mut retrogress::ProgressBar, flags: &Flags) -> Recoverable<()> {
  for step in steps {
    step.run(progress, flags)?;
  }
  Recoverable::Ok(())
}
