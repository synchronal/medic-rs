#[cfg(test)]
mod step_config_test;

pub mod step_config;
pub use step_config::StepConfig;

use crate::cli::Flags;
use crate::config;
use crate::context::Context;
use crate::error::MedicError;
use crate::noop_config::NoopConfig;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::theme::current_theme;
use crate::{AppResult, Check};
use serde::Deserialize;
use std::fmt;
use std::process::Command;
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

  fn run(self, progress: &mut retrogress::ProgressBar, flags: &mut Flags, context: &Context) -> Recoverable<()> {
    match self {
      Step::Check(config) => config.run(progress, flags, context),
      Step::Doctor(config) => config.run(progress, flags, context),
      Step::Shell(config) => config.run(progress, flags, context),
      Step::Step(config) => config.run(progress, flags, context),
      Step::Steps(steps) => {
        if flags.parallel {
          run_parallel_steps(steps, progress, flags, context)
        } else {
          run_serial_steps(steps, progress, flags, context)
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
  fn run(self, progress: &mut retrogress::ProgressBar, flags: &mut Flags, context: &Context) -> Recoverable<()> {
    progress.print_inline(&format!("{} {self}", console::style("!").bright().green(),));

    match config::Manifest::new(&flags.config_path) {
      AppResult::Ok(manifest) => {
        if let Some(doctor) = manifest.doctor {
          for check in doctor.checks {
            crate::runnable::run(check, progress, flags, context);
          }
        }
        Recoverable::Ok(())
      }
      AppResult::Err(err) => Recoverable::Nonrecoverable(err.unwrap()),
      AppResult::Quit => Recoverable::Nonrecoverable("Unable to read manifest".into()),
    }
  }

  fn to_command(&self) -> Result<std::process::Command, MedicError> {
    panic!("DoctorConfig not be converted to a Command");
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

fn run_parallel_steps(
  steps: Vec<Step>,
  progress: &mut retrogress::ProgressBar,
  flags: &mut Flags,
  context: &Context,
) -> Recoverable<()> {
  let (tx, rx) = mpsc::channel();

  thread::scope(|s| {
    for step in steps {
      let mut progress = progress.clone();
      let tx = tx.clone();
      let mut flags = flags.clone();

      s.spawn(move || {
        let result = step.run(&mut progress, &mut flags, context);
        let _ = tx.send(result);
      });
    }

    drop(tx);
  });

  let mut failure = None;
  let mut manual = None;
  let mut nonrecoverable = None;
  let mut optional = None;

  while let Ok(result) = rx.recv() {
    match result {
      Recoverable::Err(_, _) => failure = Some(result),
      Recoverable::Manual(_, _) => manual = Some(result),
      Recoverable::Nonrecoverable(_) => nonrecoverable = Some(result),
      Recoverable::Ok(_) => {}
      Recoverable::Optional(_, _) => optional = Some(result),
    }
  }

  if let Some(failure) = nonrecoverable {
    return failure;
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

fn run_serial_steps(
  steps: Vec<Step>,
  progress: &mut retrogress::ProgressBar,
  flags: &mut Flags,
  context: &Context,
) -> Recoverable<()> {
  for step in steps {
    step.run(progress, flags, context)?;
  }
  Recoverable::Ok(())
}
