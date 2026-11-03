// @related [subject](medic-src/src/extra/command.rs)

use super::command::*;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::PathBuf;

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-extra-command-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

fn env_value<'a>(command: &'a std::process::Command, key: &str) -> Option<&'a OsStr> {
  command
    .get_envs()
    .find(|(k, _)| *k == key)
    .and_then(|(_, v)| v)
}

#[test]
fn to_string_plain() {
  assert_eq!(to_string(&"make".to_string(), &None), "make");
}

#[test]
fn to_string_cd() {
  assert_eq!(
    to_string(&"make".to_string(), &Some("sub dir".into())),
    "(cd sub dir && make)"
  );
}

#[test]
fn from_string_sh() {
  let command = from_string("echo $HOME | wc", &None, &BTreeMap::new()).unwrap();

  assert_eq!(command.get_program(), "sh");
  assert_eq!(command.get_args().collect::<Vec<_>>(), vec!["-c", "echo $HOME | wc"]);
  assert_eq!(command.get_current_dir(), None);
}

#[test]
fn new_program() {
  let command = new("medic-step-thing", &None, &BTreeMap::new()).unwrap();

  assert_eq!(command.get_program(), "medic-step-thing");
  assert_eq!(command.get_args().count(), 0);
}

#[test]
fn new_missing_cd() {
  let missing = tmp_dir("new-missing").join("nope");
  let missing = missing.display().to_string();

  let err = new("true", &Some(missing.clone()), &BTreeMap::new()).unwrap_err();

  assert_eq!(err.to_string(), format!("directory {missing} does not exist"));
}

#[test]
fn from_string_missing_cd() {
  let missing = tmp_dir("from-string-missing").join("nope");
  let missing = missing.display().to_string();

  let err = from_string("true", &Some(missing.clone()), &BTreeMap::new()).unwrap_err();

  assert_eq!(err.to_string(), format!("directory {missing} does not exist"));
}

#[test]
fn missing_cd() {
  let missing = tmp_dir("with-dir-missing").join("nope");
  let missing = missing.display().to_string();
  let mut command = std::process::Command::new("true");

  let err = with_dir(&mut command, &Some(missing.clone())).unwrap_err();

  assert_eq!(err.to_string(), format!("directory {missing} does not exist"));
  assert_eq!(command.get_current_dir(), None);
}

#[test]
fn unresolvable_cd() {
  let file = tmp_dir("with-dir-not-dir").join("file");
  std::fs::write(&file, "").unwrap();
  let through_file = format!("{}/child", file.display());
  let mut command = std::process::Command::new("true");

  let err = with_dir(&mut command, &Some(through_file.clone())).unwrap_err();

  assert!(
    err
      .to_string()
      .starts_with(&format!("unable to resolve directory {through_file}: ")),
    "{err}"
  );
}

#[test]
fn canonical_cd() {
  let dir = tmp_dir("with-dir");
  let relative = format!("{}/../{}", dir.display(), dir.file_name().unwrap().to_string_lossy());
  let mut command = std::process::Command::new("true");

  with_dir(&mut command, &Some(relative)).unwrap();

  assert_eq!(
    command.get_current_dir(),
    Some(std::fs::canonicalize(&dir).unwrap().as_path())
  );
}

#[test]
fn no_cd() {
  let mut command = std::process::Command::new("true");

  with_dir(&mut command, &None).unwrap();

  assert_eq!(command.get_current_dir(), None);
}

#[test]
fn env() {
  unsafe {
    std::env::set_var("MEDIC_COMMAND_TEST_INHERITED", "inherited");
  }
  let mut command = std::process::Command::new("true");

  with_env(
    &mut command,
    &BTreeMap::from([("MEDIC_COMMAND_TEST_ADDED".to_string(), "added".to_string())]),
  );

  assert_eq!(
    env_value(&command, "MEDIC_COMMAND_TEST_INHERITED"),
    Some(OsStr::new("inherited"))
  );
  assert_eq!(
    env_value(&command, "MEDIC_COMMAND_TEST_ADDED"),
    Some(OsStr::new("added"))
  );
}

#[test]
fn env_override() {
  unsafe {
    std::env::set_var("MEDIC_COMMAND_TEST_OVERRIDE", "original");
  }
  let mut command = std::process::Command::new("true");

  with_env(
    &mut command,
    &BTreeMap::from([("MEDIC_COMMAND_TEST_OVERRIDE".to_string(), "override".to_string())]),
  );

  assert_eq!(
    env_value(&command, "MEDIC_COMMAND_TEST_OVERRIDE"),
    Some(OsStr::new("override"))
  );
}

#[test]
fn env_drops_braces() {
  unsafe {
    std::env::set_var("MEDIC_COMMAND_TEST_BRACES", "() { echo function; }");
  }
  let mut command = std::process::Command::new("true");

  with_env(&mut command, &BTreeMap::new());

  assert_eq!(env_value(&command, "MEDIC_COMMAND_TEST_BRACES"), None);
}

#[test]
fn env_keeps_configured_braces() {
  let mut command = std::process::Command::new("true");

  with_env(
    &mut command,
    &BTreeMap::from([("MEDIC_COMMAND_TEST_JSON".to_string(), "{\"a\":1}".to_string())]),
  );

  assert_eq!(
    env_value(&command, "MEDIC_COMMAND_TEST_JSON"),
    Some(OsStr::new("{\"a\":1}"))
  );
}

#[test]
fn runs_in_cd_with_env() {
  let dir = tmp_dir("from-string-run");
  let command_dir = Some(dir.to_string_lossy().into_owned());
  let env = BTreeMap::from([("MEDIC_COMMAND_TEST_RUN".to_string(), "ran".to_string())]);

  let output = from_string(
    "printf '%s %s' \"$MEDIC_COMMAND_TEST_RUN\" \"$(pwd -P)\"",
    &command_dir,
    &env,
  )
  .unwrap()
  .output()
  .unwrap();

  assert!(output.status.success());
  assert_eq!(
    String::from_utf8(output.stdout).unwrap(),
    format!("ran {}", std::fs::canonicalize(&dir).unwrap().display())
  );
}
