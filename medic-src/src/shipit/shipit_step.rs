use crate::cli::Flags;
use crate::config;
use crate::context::Context;
use crate::error::MedicError;
use crate::noop_config::NoopConfig;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use crate::theme::current_theme;
use crate::{AppResult, Check};
use serde::Deserialize;

use std::fmt;
use std::process::Command;

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

  fn run(&self, progress: &mut retrogress::ProgressBar, flags: &mut Flags, ctx: &Context) -> Recoverable<()> {
    match self {
      ShipitStep::Check(config) => config.run(progress, flags, ctx),
      ShipitStep::Shell(config) => config.run(progress, flags, ctx),
      ShipitStep::Step(config) => config.run(progress, flags, ctx),
      ShipitStep::Audit(config) => config.run(progress, flags, ctx),
      ShipitStep::Test(config) => config.run(progress, flags, ctx),
      ShipitStep::Update(config) => config.run(progress, flags, ctx),
    }
  }

  fn to_command(&self) -> Result<Command, MedicError> {
    match self {
      ShipitStep::Check(config) => config.to_command(),
      ShipitStep::Shell(config) => config.to_command(),
      ShipitStep::Step(config) => config.to_command(),
      ShipitStep::Audit(config) => config.to_command(),
      ShipitStep::Test(config) => config.to_command(),
      ShipitStep::Update(config) => config.to_command(),
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

impl Runnable for AuditConfig {
  fn run(&self, progress: &mut retrogress::ProgressBar, flags: &mut Flags, context: &Context) -> Recoverable<()> {
    progress.print_inline(&format!("{} {self}", console::style("!").bright().green(),));

    match config::Manifest::new(&flags.config_path) {
      AppResult::Ok(manifest) => {
        if let Some(config) = manifest.audit {
          for check in config.checks {
            if let AppResult::Quit = crate::runnable::run(check, progress, flags, context) {
              return Recoverable::Quit;
            }
          }
        }
        Recoverable::Ok(())
      }
      AppResult::Err(err) => Recoverable::Nonrecoverable(err.unwrap()),
      AppResult::Quit => Recoverable::Nonrecoverable("Unable to read manifest".into()),
    }
  }

  fn to_command(&self) -> Result<std::process::Command, MedicError> {
    Err(MedicError::Message(
      "AuditConfig cannot be converted to a Command".to_string(),
    ))
  }
}

impl fmt::Display for AuditConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      OptionalStyled::new("== Audit ==", current_theme().text_style.clone())
    )
  }
}

impl Runnable for TestConfig {
  fn run(&self, progress: &mut retrogress::ProgressBar, flags: &mut Flags, context: &Context) -> Recoverable<()> {
    progress.print_inline(&format!("{} {self}", console::style("!").bright().green(),));

    match config::Manifest::new(&flags.config_path) {
      AppResult::Ok(manifest) => {
        if let Some(config) = manifest.test {
          for check in config.checks {
            if let AppResult::Quit = crate::runnable::run(check, progress, flags, context) {
              return Recoverable::Quit;
            }
          }
        }
        Recoverable::Ok(())
      }
      AppResult::Err(err) => Recoverable::Nonrecoverable(err.unwrap()),
      AppResult::Quit => Recoverable::Nonrecoverable("Unable to read manifest".into()),
    }
  }

  fn to_command(&self) -> Result<std::process::Command, MedicError> {
    Err(MedicError::Message(
      "TestConfig cannot be converted to a Command".to_string(),
    ))
  }
}

impl fmt::Display for TestConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      OptionalStyled::new("== Test ==", current_theme().text_style.clone())
    )
  }
}

impl Runnable for UpdateConfig {
  fn run(&self, progress: &mut retrogress::ProgressBar, flags: &mut Flags, context: &Context) -> Recoverable<()> {
    progress.print_inline(&format!("{} {self}", console::style("!").bright().green(),));

    match config::Manifest::new(&flags.config_path) {
      AppResult::Ok(manifest) => {
        if let Some(config) = manifest.update {
          for check in config.steps {
            if let AppResult::Quit = crate::runnable::run(check, progress, flags, context) {
              return Recoverable::Quit;
            }
          }
        }
        Recoverable::Ok(())
      }
      AppResult::Err(err) => Recoverable::Nonrecoverable(err.unwrap()),
      AppResult::Quit => Recoverable::Nonrecoverable("Unable to read manifest".into()),
    }
  }

  fn to_command(&self) -> Result<std::process::Command, MedicError> {
    Err(MedicError::Message(
      "UpdateConfig cannot be converted to a Command".to_string(),
    ))
  }
}

impl fmt::Display for UpdateConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      OptionalStyled::new("== Update ==", current_theme().text_style.clone())
    )
  }
}
