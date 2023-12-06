use crate::noop_config::NoopConfig;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use crate::{AppResult, Check};
use console::style;
use retrogress::Progress;
use serde::Deserialize;

use std::fmt;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum ShipitStep {
    Check(Check),
    Shell(ShellConfig),
    Step(StepConfig),
    Audit(AuditConfig),
    Test(TestConfig),
    Update(UpdateConfig),
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct AuditConfig {
    pub audit: NoopConfig,
}
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct TestConfig {
    pub test: NoopConfig,
}
#[derive(Debug, Deserialize, Eq, PartialEq)]
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

    fn run(self, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
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
            ShipitStep::Audit(_) => write!(f, "\x1b[36m== Audit ===\x1b[0m"),
            ShipitStep::Test(_) => write!(f, "\x1b[36m== Test ===\x1b[0m"),
            ShipitStep::Update(_) => write!(f, "\x1b[36m== Update ===\x1b[0m"),
        }
    }
}

fn run_audit(progress: &mut retrogress::ProgressBar) -> AppResult<()> {
    let pb = progress.append("doctor");
    progress.println(
        pb,
        &format!(
            "{} {}",
            style("!").bright().green(),
            style("==== Audit ====").cyan()
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
            AppResult::Ok(())
        } else {
            AppResult::Err(Some("Audit failure".into()))
        }
    } else {
        AppResult::Err(Some("Unable to run medic audit".into()))
    }
}

fn run_test(progress: &mut retrogress::ProgressBar) -> AppResult<()> {
    let pb = progress.append("doctor");
    progress.println(
        pb,
        &format!(
            "{} {}",
            style("!").bright().green(),
            style("==== Test ====").cyan()
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
            AppResult::Ok(())
        } else {
            AppResult::Err(Some("Test failure".into()))
        }
    } else {
        AppResult::Err(Some("Unable to run medic test".into()))
    }
}

fn run_update(progress: &mut retrogress::ProgressBar) -> AppResult<()> {
    let pb = progress.append("doctor");
    progress.println(
        pb,
        &format!(
            "{} {}",
            style("!").bright().green(),
            style("==== Update ====").cyan()
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
            AppResult::Ok(())
        } else {
            AppResult::Err(Some("Unable to update project".into()))
        }
    } else {
        AppResult::Err(Some("Unable to run medic update".into()))
    }
}

fn audit_cmd() -> Result<Command, Box<dyn std::error::Error>> {
    let mut command = Command::new("medic");
    command.arg("audit");
    Ok(command)
}

fn test_cmd() -> Result<Command, Box<dyn std::error::Error>> {
    let mut command = Command::new("medic");
    command.arg("test");
    Ok(command)
}

fn update_cmd() -> Result<Command, Box<dyn std::error::Error>> {
    let mut command = Command::new("medic");
    command.arg("update");
    Ok(command)
}
