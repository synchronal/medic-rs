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
                        inline: false,
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
                        output: OutputFormat::Json,
                    }),
                    AuditStep::Step(StepConfig {
                        args: None,
                        command: Some("clippy".to_string()),
                        name: None,
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
                        inline: false,
                        name: "Shell check".to_string(),
                        remedy: None,
                        shell: "do something".to_string(),
                        verbose: false,
                    }),
                    DoctorStep::Check(Check {
                        verbose: false,
                        args: None,
                        cd: None,
                        check: "rust".to_string(),
                        command: Some("format-check".to_string()),
                        output: OutputFormat::Json,
                    }),
                    DoctorStep::Step(StepConfig {
                        args: None,
                        command: Some("clippy".to_string()),
                        name: None,
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
                        remedy: None,
                    },
                    OutdatedCheck {
                        args: None,
                        cd: Some("assets".to_string()),
                        check: "node".to_string(),
                        name: None,
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
                    ShipitStep::Audit(shipit::AuditConfig {
                        audit: NoopConfig {}
                    }),
                    ShipitStep::Update(shipit::UpdateConfig {
                        update: NoopConfig {}
                    }),
                    ShipitStep::Test(shipit::TestConfig {
                        test: NoopConfig {}
                    }),
                    ShipitStep::Check(Check {
                        args: None,
                        cd: None,
                        check: "rust".to_string(),
                        command: None,
                        output: OutputFormat::Json,
                        verbose: false
                    }),
                    ShipitStep::Shell(ShellConfig {
                        allow_failure: false,
                        inline: false,
                        name: "Do stuff".to_string(),
                        remedy: None,
                        shell: "do something".to_string(),
                        verbose: false
                    }),
                    ShipitStep::Step(StepConfig {
                        args: None,
                        command: None,
                        name: None,
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
                    Step::Doctor(step::DoctorConfig {
                        doctor: NoopConfig {}
                    }),
                    Step::Check(Check {
                        args: None,
                        cd: None,
                        check: "rust".to_string(),
                        command: None,
                        output: OutputFormat::Json,
                        verbose: false
                    }),
                    Step::Shell(ShellConfig {
                        allow_failure: false,
                        inline: false,
                        name: "Do stuff".to_string(),
                        remedy: None,
                        shell: "do something".to_string(),
                        verbose: false
                    }),
                    Step::Step(StepConfig {
                        args: None,
                        command: None,
                        name: None,
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
                    Step::Doctor(step::DoctorConfig {
                        doctor: NoopConfig {}
                    }),
                    Step::Check(Check {
                        args: None,
                        cd: None,
                        check: "rust".to_string(),
                        command: None,
                        output: OutputFormat::Json,
                        verbose: false
                    }),
                    Step::Shell(ShellConfig {
                        allow_failure: false,
                        inline: false,
                        name: "Do stuff".to_string(),
                        remedy: None,
                        shell: "do something".to_string(),
                        verbose: false
                    }),
                    Step::Step(StepConfig {
                        args: None,
                        command: None,
                        name: None,
                        step: "rust".to_string(),
                        verbose: false
                    })
                ]
            }),
        }
    );
}
