// @related [subject](medic-src/src/shipit/shipit_step.rs)

use super::*;
use crate::Check;
use crate::cli::Flags;
use crate::context::Context;
use crate::noop_config::NoopConfig;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::shell::ShellConfig;
use crate::step::StepConfig;
use std::path::{Path, PathBuf};

pub fn initialize() {
  crate::theme::THEME.get_or_init(crate::theme::dark_theme);
}

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-shipit-step-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

/// Writes a manifest whose `[section]` has `key = [ entries ]`.
fn write_manifest(dir: &Path, section: &str, key: &str, entries: &str) -> PathBuf {
  let path = dir.join("medic.toml");
  std::fs::write(&path, format!("[{section}]\n{key} = [\n{entries}\n]\n")).unwrap();
  path
}

fn parse(toml: &str) -> ShipitStep {
  toml::from_str(toml).unwrap()
}

fn run_with_manifest(step: &ShipitStep, manifest: PathBuf) -> Recoverable<()> {
  initialize();
  let mut flags = Flags {
    config_path: manifest,
    ..Flags::default()
  };
  let mut progress = crate::progress::new(&flags);
  step.run(&mut progress, &mut flags, &Context::new())
}

fn audit() -> ShipitStep {
  ShipitStep::Audit(AuditConfig { audit: NoopConfig {} })
}
fn test() -> ShipitStep {
  ShipitStep::Test(TestConfig { test: NoopConfig {} })
}
fn update() -> ShipitStep {
  ShipitStep::Update(UpdateConfig { update: NoopConfig {} })
}

/// Each nested-subcommand variant with its manifest section and list key.
fn nested() -> [(ShipitStep, &'static str, &'static str); 3] {
  [
    (audit(), "audit", "checks"),
    (test(), "test", "checks"),
    (update(), "update", "steps"),
  ]
}

// // // Deserialize

#[test]
fn deserialize() {
  assert_eq!(
    parse(r#"check = "json""#),
    ShipitStep::Check(toml::from_str::<Check>(r#"check = "json""#).unwrap())
  );
  assert_eq!(
    parse("name = \"Push\"\nshell = \"git push\""),
    ShipitStep::Shell(ShellConfig::new("Push".into(), "git push".into(), None, None, false))
  );
  assert_eq!(
    parse(r#"step = "thing""#),
    ShipitStep::Step(toml::from_str::<StepConfig>(r#"step = "thing""#).unwrap())
  );
}

#[test]
fn deserialize_nested() {
  assert_eq!(parse("audit = {}"), audit());
  assert_eq!(parse("test = {}"), test());
  assert_eq!(parse("update = {}"), update());
}

#[test]
fn no_variant() {
  assert!(toml::from_str::<ShipitStep>(r#"doctor = {}"#).is_err());
}

// // // Display

#[test]
fn display() {
  initialize();
  assert_eq!(
    parse(r#"check = "json""#).to_string(),
    toml::from_str::<Check>(r#"check = "json""#)
      .unwrap()
      .to_string()
  );
  assert_eq!(
    parse("name = \"Push\"\nshell = \"git push\"").to_string(),
    ShellConfig::new("Push".into(), "git push".into(), None, None, false).to_string()
  );
  assert_eq!(
    parse(r#"step = "thing""#).to_string(),
    toml::from_str::<StepConfig>(r#"step = "thing""#)
      .unwrap()
      .to_string()
  );
}

#[test]
fn display_nested() {
  initialize();
  assert_eq!(audit().to_string(), "\u{1b}[36m== Audit ==\u{1b}[0m");
  assert_eq!(test().to_string(), "\u{1b}[36m== Test ==\u{1b}[0m");
  assert_eq!(update().to_string(), "\u{1b}[36m== Update ==\u{1b}[0m");
}

#[test]
fn config_display() {
  initialize();
  assert_eq!(AuditConfig { audit: NoopConfig {} }.to_string(), audit().to_string());
  assert_eq!(TestConfig { test: NoopConfig {} }.to_string(), test().to_string());
  assert_eq!(UpdateConfig { update: NoopConfig {} }.to_string(), update().to_string());
}

// // // Runnable delegation

#[test]
fn allow_failure() {
  assert!(parse("name = \"x\"\nshell = \"true\"\nallow_failure = true").allow_failure());
  assert!(!parse(r#"check = "json""#).allow_failure());
  assert!(!parse(r#"step = "thing""#).allow_failure());
  for (step, _, _) in nested() {
    assert!(!step.allow_failure(), "{step:?}");
  }
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
  for (step, _, _) in nested() {
    assert_eq!(step.platform(), &None, "{step:?}");
  }
}

#[test]
fn verbose() {
  assert!(parse("check = \"json\"\nverbose = true").verbose());
  assert!(!parse(r#"check = "json""#).verbose());
  assert!(!parse("name = \"x\"\nshell = \"true\"\nverbose = true\ninline = true").verbose());
  assert!(parse("step = \"thing\"\nverbose = true").verbose());
  for (step, _, _) in nested() {
    assert!(step.verbose(), "{step:?}");
  }
}

#[test]
fn to_command() {
  assert_eq!(
    parse(r#"check = "json""#)
      .to_command()
      .unwrap()
      .get_program(),
    "medic-check-json"
  );
  assert_eq!(
    parse("name = \"x\"\nshell = \"true\"")
      .to_command()
      .unwrap()
      .get_program(),
    "sh"
  );
  assert_eq!(
    parse(r#"step = "thing""#)
      .to_command()
      .unwrap()
      .get_program(),
    "medic-step-thing"
  );
}

#[test]
fn to_command_nested() {
  for (step, expected) in [
    (audit(), "AuditConfig cannot be converted to a Command"),
    (test(), "TestConfig cannot be converted to a Command"),
    (update(), "UpdateConfig cannot be converted to a Command"),
  ] {
    assert_eq!(step.to_command().unwrap_err().to_string(), expected);
  }
}

// // // Nested subcommand run

#[test]
fn runs_all() {
  for (step, section, key) in nested() {
    let dir = tmp_dir(&format!("success-{section}"));
    let first = dir.join("first");
    let second = dir.join("second");
    let manifest = write_manifest(
      &dir,
      section,
      key,
      &format!(
        r#"{{ name = "first", shell = "touch {}" }},
{{ name = "second", shell = "touch {}" }},"#,
        first.display(),
        second.display()
      ),
    );

    assert!(
      matches!(run_with_manifest(&step, manifest), Recoverable::Ok(())),
      "{section}"
    );
    assert!(first.exists(), "{section}: first entry did not run");
    assert!(second.exists(), "{section}: second entry did not run");
  }
}

#[test]
fn missing_section() {
  for (step, section, _) in nested() {
    let dir = tmp_dir(&format!("missing-section-{section}"));
    let manifest = write_manifest(&dir, "doctor", "checks", "");

    assert!(
      matches!(run_with_manifest(&step, manifest), Recoverable::Ok(())),
      "{section}"
    );
  }
}

#[test]
fn missing_manifest() {
  for (step, section, _) in nested() {
    let dir = tmp_dir(&format!("missing-manifest-{section}"));

    match run_with_manifest(&step, dir.join("does-not-exist.toml")) {
      Recoverable::Nonrecoverable(err) => assert!(err.to_string().contains("does not exist"), "{section}: {err}"),
      _ => panic!("{section}: expected Nonrecoverable"),
    }
  }
}

#[test]
fn bad_manifest() {
  for (step, section, _) in nested() {
    let dir = tmp_dir(&format!("invalid-manifest-{section}"));
    let path = dir.join("medic.toml");
    std::fs::write(&path, "this is not = = toml").unwrap();

    match run_with_manifest(&step, path) {
      Recoverable::Nonrecoverable(err) => {
        assert!(err.to_string().contains("Unable to parse manifest"), "{section}: {err}")
      }
      _ => panic!("{section}: expected Nonrecoverable"),
    }
  }
}

#[test]
fn stops_on_manual() {
  for (step, section, key) in nested() {
    let dir = tmp_dir(&format!("manual-{section}"));
    let second = dir.join("second");
    let manifest = write_manifest(
      &dir,
      section,
      key,
      &format!(
        r#"{{ name = "manual", shell = "false", manual = true, remedy = "fix it" }},
{{ name = "never runs", shell = "touch {}" }},"#,
        second.display()
      ),
    );

    assert!(
      matches!(run_with_manifest(&step, manifest), Recoverable::Quit),
      "{section}"
    );
    assert!(!second.exists(), "{section}: continued after quit");
  }
}

#[test]
#[ignore = "bug: shipit's nested audit/test/update ignore AppResult::Err from failed entries and keep running (same issue fixed for DoctorConfig in 6118670)"]
fn stops_on_failure() {
  for (step, section, key) in nested() {
    let dir = tmp_dir(&format!("failure-{section}"));
    let second = dir.join("second");
    let manifest = write_manifest(
      &dir,
      section,
      key,
      &format!(
        r#"{{ name = "fails", shell = "false" }},
{{ name = "never runs", shell = "touch {}" }},"#,
        second.display()
      ),
    );

    assert!(
      matches!(run_with_manifest(&step, manifest), Recoverable::Nonrecoverable(_)),
      "{section}"
    );
    assert!(!second.exists(), "{section}: continued after a failed entry");
  }
}
