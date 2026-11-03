// @related [subject](medic-src/src/shell/shell_config.rs)

use super::*;
use crate::extra;
use crate::runnable::Runnable;
use std::collections::BTreeMap;
use std::ffi::OsStr;
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

#[test]
fn deserialize() {
  let toml = r#"
        shell = "some command"
        name = "Run some command"
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: false,
      cd: None,
      env: BTreeMap::default(),
      inline: false,
      manual: false,
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn cd() {
  let toml = r#"
        shell = "some command"
        name = "Run some command"
        cd = "./subdirectory"
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: false,
      cd: Some("./subdirectory".to_string()),
      env: BTreeMap::default(),
      inline: false,
      manual: false,
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn env() {
  let toml = r#"
        env = { MY_VAR = "first", SECOND_VAR = "second" }
        shell = "some command"
        name = "Run some command"
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: false,
      cd: None,
      env: BTreeMap::from([
        ("MY_VAR".to_string(), "first".to_string()),
        ("SECOND_VAR".to_string(), "second".to_string())
      ]),
      inline: false,
      manual: false,
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn verbose() {
  let toml = r#"
        shell = "some command"
        name = "Run some command"
        verbose = true
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: false,
      cd: None,
      env: BTreeMap::default(),
      inline: false,
      manual: false,
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: true,
    }
  );
}

#[test]
fn allow_failure() {
  let toml = r#"
        shell = "some command"
        name = "Run some command"
        allow_failure = true
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: true,
      cd: None,
      env: BTreeMap::default(),
      inline: false,
      manual: false,
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn command() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: None,
    env: BTreeMap::default(),
    inline: false,
    manual: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };

  let cmd = shell.to_command().unwrap();
  let args: Vec<&OsStr> = cmd.get_args().collect();

  assert_eq!(cmd.get_program(), "sh");
  assert_eq!(args, ["-c", "some command"]);
}

#[test]
fn command_cd() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: Some("../fixtures/bin".to_string()),
    env: BTreeMap::default(),
    inline: false,
    manual: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };

  let path_expansion = extra::env::subst("${PWD}/fixtures/bin").unwrap();

  let path: std::path::PathBuf = path_expansion.into();
  let cmd = shell.to_command().unwrap();

  assert_eq!(cmd.get_current_dir(), Some(path.as_path()));
}

#[test]
fn command_env() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: None,
    env: BTreeMap::from([
      ("VAR".to_string(), "value".to_string()),
      ("OTHER".to_string(), "other".to_string()),
    ]),
    inline: false,
    manual: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };
  let cmd = shell.to_command().unwrap();

  let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
  let expected = [
    (OsStr::new("OTHER"), Some(OsStr::new("other"))),
    (OsStr::new("VAR"), Some(OsStr::new("value"))),
  ];

  assert!(expected.iter().all(|item| envs.contains(item)));
}

#[test]
fn manual() {
  let toml = r#"
        shell = "some command"
        name = "Run some command"
        manual = true
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: false,
      cd: None,
      env: BTreeMap::default(),
      inline: false,
      manual: true,
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}
#[test]
fn remedy() {
  let toml = r#"
        shell = "some command"
        name = "Run some command"
        remedy = "do something"
        "#;

  let result: ShellConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    ShellConfig {
      allow_failure: false,
      cd: None,
      env: BTreeMap::default(),
      inline: false,
      manual: false,
      platform: None,
      name: "Run some command".to_string(),
      remedy: Some("do something".to_string()),
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn display() {
  initialize();
  let shell = ShellConfig {
    allow_failure: false,
    cd: None,
    env: BTreeMap::default(),
    inline: false,
    manual: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{shell}"),
    "\u{1b}[36mRun some command\u{1b}[0m \u{1b}[33m(some command)\u{1b}[0m"
  );
}

#[test]
fn display_cd() {
  initialize();
  let shell = ShellConfig {
    allow_failure: false,
    cd: Some("../fixtures/bin".to_string()),
    env: BTreeMap::default(),
    inline: false,
    manual: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{shell}"),
    "\u{1b}[36mRun some command\u{1b}[0m \u{1b}[33m(some command)\u{1b}[0m \u{1b}[32m(../fixtures/bin)\u{1b}[0m"
  );
}

fn missing_dir(name: &str) -> String {
  std::env::temp_dir()
    .join(format!("medic-shell-config-{}-{name}-missing", std::process::id()))
    .display()
    .to_string()
}

#[test]
fn missing_cd_command() {
  let missing = missing_dir("to-command");
  let shell = ShellConfig::new("x".into(), "true".into(), Some(missing.clone()), None, false);

  let err = shell.to_command().unwrap_err();

  assert_eq!(err.to_string(), format!("directory {missing} does not exist"));
}

#[test]
fn missing_cd() {
  initialize();
  let missing = missing_dir("run");
  let shell = ShellConfig::new("x".into(), "true".into(), Some(missing.clone()), None, false);
  let mut flags = crate::cli::Flags::default();
  let mut progress = crate::progress::new(&flags);

  match shell.run(&mut progress, &mut flags, &crate::context::Context::new()) {
    crate::recoverable::Recoverable::Err(Some(err), None) => assert_eq!(
      err.to_string(),
      format!("Failed to parse command: directory {missing} does not exist")
    ),
    _ => panic!("expected Recoverable::Err(Some(_), None)"),
  }
}
