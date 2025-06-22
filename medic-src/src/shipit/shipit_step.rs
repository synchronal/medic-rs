use crate::extra;
use crate::noop_config::NoopConfig;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use crate::theme::current_theme;
use crate::Check;
use console::style;
use retrogress::Progress;
use serde::Deserialize;

use std::fmt;
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum ShipitStep {
  Check(Check),
  Shell(ShellConfig),
  Step(StepConfig),
  Audit(AuditConfig),
  Test(TestConfig),
  Update(UpdateConfig),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AuditConfig {
  pub audit: NoopConfig,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TestConfig {
  pub test: NoopConfig,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateConfig {
  pub update: NoopConfig,
}

impl Runnable for ShipitStep {
  fn allow_failure(&self) -> bool {
    match self {
      ShipitStep::Check(config) => config.allow_failure(),
      ShipitStep::Shell(config) => config.allow_failure(),
      ShipitStep::Step(config) => config.allow_failure(),
      ShipitStep::Audit(_) => false,
      ShipitStep::Test(_) => false,
      ShipitStep::Update(_) => false,
    }
  }

  fn platform(&self) -> &Option<Vec<String>> {
    match self {
      ShipitStep::Check(config) => config.platform(),
      ShipitStep::Shell(config) => config.platform(),
      ShipitStep::Step(config) => config.platform(),
      ShipitStep::Audit(_) => &None,
      ShipitStep::Test(_) => &None,
      ShipitStep::Update(_) => &None,
    }
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    match self {
      ShipitStep::Check(config) => config.run(progress),
      ShipitStep::Shell(config) => config.run(progress),
      ShipitStep::Step(config) => config.run(progress),
      ShipitStep::Audit(_) => run_audit(progress),
      ShipitStep::Test(_) => run_test(progress),
      ShipitStep::Update(_) => run_update(progress),
    }
  }

  fn to_command(&self) -> Result<Command, Box<dyn std::error::Error>> {
    match self {
      ShipitStep::Check(config) => config.to_command(),
      ShipitStep::Shell(config) => config.to_command(),
      ShipitStep::Step(config) => config.to_command(),
      ShipitStep::Audit(_) => audit_cmd(),
      ShipitStep::Test(_) => test_cmd(),
      ShipitStep::Update(_) => update_cmd(),
    }
  }

  fn verbose(&self) -> bool {
    match self {
      ShipitStep::Check(config) => config.verbose(),
      ShipitStep::Shell(config) => config.verbose(),
      ShipitStep::Step(config) => config.verbose(),
      ShipitStep::Audit(_) => true,
      ShipitStep::Test(_) => true,
      ShipitStep::Update(_) => true,
    }
  }
}

impl fmt::Display for ShipitStep {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ShipitStep::Check(config) => config.fmt(f),
      ShipitStep::Shell(config) => config.fmt(f),
      ShipitStep::Step(config) => config.fmt(f),
      ShipitStep::Audit(_) => write!(
        f,
        "{}",
        OptionalStyled::new("== Audit ==", current_theme().text_style.clone())
      ),
      ShipitStep::Test(_) => write!(
        f,
        "{}",
        OptionalStyled::new("== Test ==", current_theme().text_style.clone())
      ),
      ShipitStep::Update(_) => write!(
        f,
        "{}",
        OptionalStyled::new("== Update ==", current_theme().text_style.clone())
      ),
    }
  }
}

fn run_audit(progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
  let pb = progress.append("doctor");
  progress.println(
    pb,
    &format!(
      "{} {}",
      style("!").bright().green(),
      OptionalStyled::new("== Audit ==", current_theme().text_style.clone())
    ),
  );
  progress.hide(pb);
  if let Ok(result) = audit_cmd()
    .unwrap()
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
  {
    if result.status.success() {
      Recoverable::Ok(())
    } else if result.status.code() == Some(crate::USER_QUIT) {
      std::process::exit(crate::USER_QUIT);
    } else {
      Recoverable::Err(Some("Audit failure".into()), None)
    }
  } else {
    Recoverable::Err(Some("Unable to run medic audit".into()), None)
  }
}

fn run_test(progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
  let pb = progress.append("doctor");
  progress.println(
    pb,
    &format!(
      "{} {}",
      style("!").bright().green(),
      OptionalStyled::new("== Test ==", current_theme().text_style.clone())
    ),
  );
  progress.hide(pb);
  if let Ok(result) = test_cmd()
    .unwrap()
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
  {
    if result.status.success() {
      Recoverable::Ok(())
    } else if result.status.code() == Some(crate::USER_QUIT) {
      std::process::exit(crate::USER_QUIT);
    } else {
      Recoverable::Err(Some("Test failure".into()), None)
    }
  } else {
    Recoverable::Err(Some("Unable to run medic test".into()), None)
  }
}

fn run_update(progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
  let pb = progress.append("doctor");
  progress.println(
    pb,
    &format!(
      "{} {}",
      style("!").bright().green(),
      OptionalStyled::new("== Update ==", current_theme().text_style.clone())
    ),
  );
  progress.hide(pb);
  if let Ok(result) = update_cmd()
    .unwrap()
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
  {
    if result.status.success() {
      Recoverable::Ok(())
    } else if result.status.code() == Some(crate::USER_QUIT) {
      std::process::exit(crate::USER_QUIT);
    } else {
      Recoverable::Err(Some("Unable to update project".into()), None)
    }
  } else {
    Recoverable::Err(Some("Unable to run medic update".into()), None)
  }
}

fn audit_cmd() -> Result<Command, Box<dyn std::error::Error>> {
  let mut command = extra::command::new("medic", &None);
  command.arg("audit");
  Ok(command)
}

fn test_cmd() -> Result<Command, Box<dyn std::error::Error>> {
  let mut command = extra::command::new("medic", &None);
  command.arg("test");
  Ok(command)
}

fn update_cmd() -> Result<Command, Box<dyn std::error::Error>> {
  let mut command = extra::command::new("medic", &None);
  command.arg("update");
  Ok(command)
}
