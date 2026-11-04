// @related [subject](medic-src/src/audit/audit_step.rs)

use super::*;
use crate::Check;
use crate::cli::Flags;
use crate::context::Context;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;

pub fn initialize() {
  crate::theme::THEME.get_or_init(crate::theme::dark_theme);
}

fn parse(toml: &str) -> AuditStep {
  toml::from_str(toml).unwrap()
}

fn run_step(step: &AuditStep) -> Recoverable<()> {
  initialize();
  let mut flags = Flags::default();
  let mut progress = crate::progress::new(&flags);
  step.run(&mut progress, &mut flags, &Context::new())
}

// // // Deserialize

#[test]
fn check() {
  let step = parse(
    r#"
    check = "json"
    command = "sub"
    "#,
  );

  let expected: Check = toml::from_str(
    r#"
    check = "json"
    command = "sub"
    "#,
  )
  .unwrap();
  assert_eq!(step, AuditStep::Check(expected));
}

#[test]
fn shell() {
  let step = parse(
    r#"
    name = "Lint"
    shell = "true"
    allow_failure = true
    "#,
  );

  let mut expected = ShellConfig::new("Lint".into(), "true".into(), None, None, false);
  expected.allow_failure = true;
  assert_eq!(step, AuditStep::Shell(expected));
}

#[test]
fn step() {
  let step = parse(r#"step = "thing""#);

  let expected: StepConfig = toml::from_str(r#"step = "thing""#).unwrap();
  assert_eq!(step, AuditStep::Step(expected));
}

#[test]
fn no_variant() {
  assert!(toml::from_str::<AuditStep>(r#"name = "only a name""#).is_err());
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
      "name = \"Lint\"\nshell = \"true\"\ncd = \"./sub\"",
      ShellConfig::new("Lint".into(), "true".into(), Some("./sub".into()), None, false).to_string(),
    ),
    (
      r#"step = "thing""#,
      toml::from_str::<StepConfig>(r#"step = "thing""#)
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
}

#[test]
fn allow_failure_false() {
  assert!(!parse(r#"check = "json""#).allow_failure());
  assert!(!parse(r#"step = "thing""#).allow_failure());
}

#[test]
fn platform() {
  let platform = Some(vec!["Linux".to_string()]);

  assert_eq!(parse("check = \"json\"\nplatform = [\"Linux\"]").platform(), &platform);
  assert_eq!(
    parse("name = \"x\"\nshell = \"true\"\nplatform = [\"Linux\"]").platform(),
    &platform
  );
  assert_eq!(parse("step = \"thing\"\nplatform = [\"Linux\"]").platform(), &platform);
  assert_eq!(parse(r#"step = "thing""#).platform(), &None);
}

#[test]
fn verbose() {
  assert!(parse("check = \"json\"\nverbose = true").verbose());
  assert!(parse("name = \"x\"\nshell = \"true\"\nverbose = true").verbose());
  assert!(parse("step = \"thing\"\nverbose = true").verbose());
  assert!(!parse(r#"step = "thing""#).verbose());
}

#[test]
fn inline_quiet() {
  assert!(!parse("name = \"x\"\nshell = \"true\"\nverbose = true\ninline = true").verbose());
}

#[test]
fn to_command() {
  let check = parse(r#"check = "json""#).to_command().unwrap();
  let shell = parse("name = \"x\"\nshell = \"echo hi\"")
    .to_command()
    .unwrap();
  let step = parse(r#"step = "thing""#).to_command().unwrap();

  assert_eq!(check.get_program(), "medic-check-json");
  assert_eq!(shell.get_program(), "sh");
  assert_eq!(shell.get_args().collect::<Vec<_>>(), vec!["-c", "echo hi"]);
  assert_eq!(step.get_program(), "medic-step-thing");
}

#[test]
fn to_command_err() {
  assert!(parse(r#"check = "does-not-exist""#).to_command().is_err());
  assert!(parse("name = \"x\"\nshell = \"\"").to_command().is_err());
}

#[test]
fn runs_shell() {
  assert!(matches!(
    run_step(&parse("name = \"x\"\nshell = \"true\"")),
    Recoverable::Ok(())
  ));
  assert!(matches!(
    run_step(&parse("name = \"x\"\nshell = \"false\"\nallow_failure = true")),
    Recoverable::Optional((), None)
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
