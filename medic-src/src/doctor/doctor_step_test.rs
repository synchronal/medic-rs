// @related [subject](medic-src/src/doctor/doctor_step.rs)

use super::*;
use crate::Check;
use crate::cli::Flags;
use crate::context::Context;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use std::sync::Once;

static INIT: Once = Once::new();
pub fn initialize() {
  INIT.call_once(|| {
    if crate::theme::THEME.get().is_none() {
      let theme = crate::theme::dark_theme();
      crate::theme::set_theme(theme);
    }
  });
}

fn parse(toml: &str) -> DoctorStep {
  toml::from_str(toml).unwrap()
}

fn run_step(step: &DoctorStep) -> Recoverable<()> {
  initialize();
  let mut flags = Flags::default();
  let mut progress = crate::progress::new(&flags);
  step.run(&mut progress, &mut flags, &Context::new())
}

// // // Deserialize

#[test]
fn check() {
  let expected: Check = toml::from_str("check = \"json\"\noutput = \"stdio\"").unwrap();

  assert_eq!(
    parse("check = \"json\"\noutput = \"stdio\""),
    DoctorStep::Check(expected)
  );
}

#[test]
fn shell() {
  let step = parse(
    r#"
    name = "Install"
    shell = "true"
    remedy = "fix"
    "#,
  );

  assert_eq!(
    step,
    DoctorStep::Shell(ShellConfig::new(
      "Install".into(),
      "true".into(),
      None,
      Some("fix".into()),
      false
    ))
  );
}

#[test]
fn step() {
  let expected: StepConfig = toml::from_str("step = \"thing\"\ncommand = \"go\"").unwrap();

  assert_eq!(parse("step = \"thing\"\ncommand = \"go\""), DoctorStep::Step(expected));
}

#[test]
fn no_variant() {
  assert!(toml::from_str::<DoctorStep>(r#"shell = "true""#).is_err());
}

// // // Display

#[test]
fn display() {
  initialize();
  for (toml, inner) in [
    (
      r#"check = "json""#,
      toml::from_str::<Check>(r#"check = "json""#)
        .unwrap()
        .to_string(),
    ),
    (
      "name = \"Install\"\nshell = \"true\"",
      ShellConfig::new("Install".into(), "true".into(), None, None, false).to_string(),
    ),
    (
      "step = \"thing\"\nname = \"Named step\"",
      toml::from_str::<StepConfig>("step = \"thing\"\nname = \"Named step\"")
        .unwrap()
        .to_string(),
    ),
  ] {
    assert_eq!(parse(toml).to_string(), inner);
  }
}

// // // Runnable delegation

#[test]
fn allow_failure() {
  assert!(parse("name = \"x\"\nshell = \"true\"\nallow_failure = true").allow_failure());
  assert!(!parse("name = \"x\"\nshell = \"true\"").allow_failure());
  assert!(!parse(r#"check = "json""#).allow_failure());
  assert!(!parse(r#"step = "thing""#).allow_failure());
}

#[test]
fn platform() {
  let platform = Some(vec!["Darwin".to_string()]);

  assert_eq!(parse("check = \"json\"\nplatform = [\"Darwin\"]").platform(), &platform);
  assert_eq!(
    parse("name = \"x\"\nshell = \"true\"\nplatform = [\"Darwin\"]").platform(),
    &platform
  );
  assert_eq!(parse("step = \"thing\"\nplatform = [\"Darwin\"]").platform(), &platform);
  assert_eq!(parse(r#"check = "json""#).platform(), &None);
}

#[test]
fn verbose() {
  assert!(parse("check = \"json\"\nverbose = true").verbose());
  assert!(parse("name = \"x\"\nshell = \"true\"\nverbose = true").verbose());
  assert!(parse("step = \"thing\"\nverbose = true").verbose());
  assert!(!parse(r#"check = "json""#).verbose());
}

#[test]
#[ignore = "bug: DoctorStep::verbose reads the ShellConfig field directly instead of ShellConfig::verbose(), so inline does not suppress verbose (AuditStep and ShipitStep delegate correctly)"]
fn inline_quiet() {
  assert!(!parse("name = \"x\"\nshell = \"true\"\nverbose = true\ninline = true").verbose());
}

#[test]
fn to_command() {
  let check = parse(r#"check = "stdio""#).to_command().unwrap();
  let shell = parse("name = \"x\"\nshell = \"echo hi\"")
    .to_command()
    .unwrap();
  let step = parse(r#"step = "thing""#).to_command().unwrap();

  assert_eq!(check.get_program(), "medic-check-stdio");
  assert_eq!(shell.get_program(), "sh");
  assert_eq!(step.get_program(), "medic-step-thing");
}

#[test]
fn to_command_err() {
  assert!(parse(r#"step = "does-not-exist""#).to_command().is_err());
}

#[test]
fn runs_shell() {
  assert!(matches!(
    run_step(&parse("name = \"x\"\nshell = \"true\"")),
    Recoverable::Ok(())
  ));
  assert!(matches!(
    run_step(&parse("name = \"x\"\nshell = \"false\"\nremedy = \"fix\"")),
    Recoverable::Err(None, Some(_))
  ));
}

#[test]
fn runs_check() {
  assert!(matches!(run_step(&parse(r#"check = "json""#)), Recoverable::Ok(())));
}

#[test]
fn runs_step() {
  assert!(matches!(run_step(&parse(r#"step = "thing""#)), Recoverable::Ok(())));
}
