// @related [subject](medic-src/src/config/manifest.rs)

use super::manifest::*;
use crate::audit::*;
use crate::check::OutputFormat;
use crate::doctor::*;
use crate::noop_config::NoopConfig;
use crate::shell::ShellConfig;
use crate::shipit;
use crate::step;
use crate::step::StepConfig;
use crate::Check;
use crate::OutdatedCheck;
use crate::ShipitStep;
use crate::Step;
use indoc::indoc;
use std::collections::BTreeMap;

#[test]
fn deserialize_audit() {
  let toml = indoc! {r#"
    [audit]
    checks = [
      { name = "Shell check", shell = "do something" },
      { check = "rust", command = "format-check" },
      { step = "rust", command = "clippy" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: Some(AuditConfig {
        checks: vec![
          AuditStep::Shell(ShellConfig {
            allow_failure: false,
            cd: None,
            platform: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "Shell check".to_string(),
            remedy: None,
            shell: "do something".to_string(),
            verbose: false,
          }),
          AuditStep::Check(Check {
            verbose: false,
            args: None,
            cd: None,
            check: "rust".to_string(),
            command: Some("format-check".to_string()),
            env: BTreeMap::default(),
            output: OutputFormat::Json,
            platform: None,
          }),
          AuditStep::Step(StepConfig {
            args: None,
            cd: None,
            command: Some("clippy".to_string()),
            env: BTreeMap::default(),
            name: None,
            platform: None,
            step: "rust".to_string(),
            verbose: false,
          }),
        ]
      }),
      doctor: None,
      outdated: None,
      shipit: None,
      test: None,
      update: None,
    }
  );
}

#[test]
fn deserialize_doctor() {
  let toml = indoc! {r#"
    [doctor]
    checks = [
      { name = "Shell check", shell = "do something" },
      { check = "rust", command = "format-check" },
      { step = "rust", command = "clippy" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: None,
      doctor: Some(DoctorConfig {
        checks: vec![
          DoctorStep::Shell(ShellConfig {
            allow_failure: false,
            cd: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "Shell check".to_string(),
            platform: None,
            remedy: None,
            shell: "do something".to_string(),
            verbose: false,
          }),
          DoctorStep::Check(Check {
            args: None,
            cd: None,
            check: "rust".to_string(),
            command: Some("format-check".to_string()),
            env: BTreeMap::default(),
            output: OutputFormat::Json,
            platform: None,
            verbose: false,
          }),
          DoctorStep::Step(StepConfig {
            args: None,
            cd: None,
            command: Some("clippy".to_string()),
            env: BTreeMap::default(),
            name: None,
            platform: None,
            step: "rust".to_string(),
            verbose: false,
          }),
        ]
      }),
      outdated: None,
      shipit: None,
      test: None,
      update: None,
    }
  );
}

#[test]
fn deserialize_outdated() {
  let toml = indoc! {r#"
    [outdated]
    checks = [
      { check = "rust" },
      { check = "node", cd = "assets", remedy = "npm update" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: None,
      doctor: None,
      outdated: Some(OutdatedConfig {
        checks: vec![
          OutdatedCheck {
            args: None,
            cd: None,
            check: "rust".to_string(),
            name: None,
            platform: None,
            remedy: None,
          },
          OutdatedCheck {
            args: None,
            cd: Some("assets".to_string()),
            check: "node".to_string(),
            name: None,
            platform: None,
            remedy: Some("npm update".to_string()),
          },
        ]
      }),
      shipit: None,
      test: None,
      update: None,
    }
  );
}

#[test]
fn deserialize_shipit() {
  let toml = indoc! {r#"
    [shipit]
    steps = [
      { audit = {} },
      { update = {} },
      { test = {} },
      { check = "rust" },
      { name = "Do stuff", shell = "do something" },
      { step = "rust" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: None,
      doctor: None,
      outdated: None,
      shipit: Some(ShipitConfig {
        steps: vec![
          ShipitStep::Audit(shipit::AuditConfig { audit: NoopConfig {} }),
          ShipitStep::Update(shipit::UpdateConfig { update: NoopConfig {} }),
          ShipitStep::Test(shipit::TestConfig { test: NoopConfig {} }),
          ShipitStep::Check(Check {
            args: None,
            cd: None,
            check: "rust".to_string(),
            command: None,
            env: BTreeMap::default(),
            output: OutputFormat::Json,
            platform: None,
            verbose: false
          }),
          ShipitStep::Shell(ShellConfig {
            allow_failure: false,
            cd: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "Do stuff".to_string(),
            platform: None,
            remedy: None,
            shell: "do something".to_string(),
            verbose: false,
          }),
          ShipitStep::Step(StepConfig {
            args: None,
            cd: None,
            command: None,
            env: BTreeMap::default(),
            name: None,
            platform: None,
            step: "rust".to_string(),
            verbose: false
          })
        ]
      }),
      test: None,
      update: None,
    }
  );
}

#[test]
fn deserialize_test() {
  let toml = indoc! {r#"
    [test]
    checks = [
      { doctor = {} },
      { check = "rust" },
      { name = "Do stuff", shell = "do something" },
      { step = "rust" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: None,
      doctor: None,
      outdated: None,
      shipit: None,
      test: Some(TestConfig {
        checks: vec![
          Step::Doctor(step::DoctorConfig { doctor: NoopConfig {} }),
          Step::Check(Check {
            args: None,
            cd: None,
            check: "rust".to_string(),
            command: None,
            env: BTreeMap::default(),
            output: OutputFormat::Json,
            platform: None,
            verbose: false
          }),
          Step::Shell(ShellConfig {
            allow_failure: false,
            cd: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "Do stuff".to_string(),
            platform: None,
            remedy: None,
            shell: "do something".to_string(),
            verbose: false
          }),
          Step::Step(StepConfig {
            args: None,
            cd: None,
            command: None,
            env: BTreeMap::default(),
            name: None,
            platform: None,
            step: "rust".to_string(),
            verbose: false
          })
        ]
      }),
      update: None,
    }
  );
}

#[test]
fn deserialize_update() {
  let toml = indoc! {r#"
    [update]
    steps = [
      { doctor = {} },
      { check = "rust" },
      { name = "Do stuff", shell = "do something" },
      { step = "rust" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: None,
      doctor: None,
      outdated: None,
      shipit: None,
      test: None,
      update: Some(UpdateConfig {
        steps: vec![
          Step::Doctor(step::DoctorConfig { doctor: NoopConfig {} }),
          Step::Check(Check {
            args: None,
            cd: None,
            check: "rust".to_string(),
            command: None,
            env: BTreeMap::default(),
            output: OutputFormat::Json,
            platform: None,
            verbose: false
          }),
          Step::Shell(ShellConfig {
            allow_failure: false,
            cd: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "Do stuff".to_string(),
            platform: None,
            remedy: None,
            shell: "do something".to_string(),
            verbose: false
          }),
          Step::Step(StepConfig {
            args: None,
            cd: None,
            command: None,
            env: BTreeMap::default(),
            name: None,
            platform: None,
            step: "rust".to_string(),
            verbose: false
          })
        ]
      }),
    }
  );
}

#[test]
fn deserialize_nested_steps() {
  let toml = indoc! {r#"
    [test]
    checks = [
      { name = "First step", shell = "echo 'Step 1'" },
      [
        { name = "Nested step 1", shell = "echo 'Nested 1'" },
        { name = "Nested step 2", shell = "echo 'Nested 2'" },
      ],
      { name = "Last step", shell = "echo 'Step 3'" },
    ]
    "#};

  let manifest: Manifest = toml::from_str(toml).expect("Unable to parse Manifest from toml");
  assert_eq!(
    manifest,
    Manifest {
      audit: None,
      doctor: None,
      outdated: None,
      shipit: None,
      test: Some(TestConfig {
        checks: vec![
          Step::Shell(ShellConfig {
            allow_failure: false,
            platform: None,
            cd: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "First step".to_string(),
            remedy: None,
            shell: "echo 'Step 1'".to_string(),
            verbose: false
          }),
          Step::Steps(vec![
            Step::Shell(ShellConfig {
              allow_failure: false,
              platform: None,
              cd: None,
              env: BTreeMap::default(),
              inline: false,
              manual: false,
              name: "Nested step 1".to_string(),
              remedy: None,
              shell: "echo 'Nested 1'".to_string(),
              verbose: false
            }),
            Step::Shell(ShellConfig {
              allow_failure: false,
              platform: None,
              cd: None,
              env: BTreeMap::default(),
              inline: false,
              manual: false,
              name: "Nested step 2".to_string(),
              remedy: None,
              shell: "echo 'Nested 2'".to_string(),
              verbose: false
            }),
          ]),
          Step::Shell(ShellConfig {
            allow_failure: false,
            platform: None,
            cd: None,
            env: BTreeMap::default(),
            inline: false,
            manual: false,
            name: "Last step".to_string(),
            remedy: None,
            shell: "echo 'Step 3'".to_string(),
            verbose: false
          }),
        ]
      }),
      update: None,
    }
  );
}
