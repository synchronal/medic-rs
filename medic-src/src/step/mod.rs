#[cfg(test)]
mod step_config_test;

pub mod step_config;
pub use step_config::StepConfig;

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

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    match self {
      Step::Check(config) => config.run(progress),
      Step::Doctor(_) => run_doctor(progress),
      Step::Shell(config) => config.run(progress),
      Step::Step(config) => config.run(progress),
      Step::Steps(steps) => run_parallel_steps(steps, progress),
    }
  }

  fn to_command(&self) -> Result<Command, MedicError> {
    match self {
      Step::Check(config) => config.to_command(),
      Step::Doctor(_) => doctor_command(),
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
      Step::Doctor(_) => write!(
        f,
        "{}",
        OptionalStyled::new("== Doctor ==", current_theme().text_style.clone())
      ),
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

fn run_doctor(progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
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
  if let Ok(result) = doctor_command()
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

fn doctor_command() -> Result<Command, MedicError> {
  let command = extra::command::from_string("medic doctor", &None);
  Ok(command)
}

fn run_parallel_steps(steps: Vec<Step>, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
  let (tx, rx) = mpsc::channel();
  let mut handles = vec![];

  for step in steps {
    let mut progress = progress.clone();
    let tx = tx.clone();

    let handle = thread::spawn(move || {
      let result = step.run(&mut progress);
      let _ = tx.send(result);
    });

    handles.push(handle);
  }

  drop(tx);

  for handle in handles {
    handle.join().expect("Unable to wait for internal thread");
  }

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
