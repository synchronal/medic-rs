// @related [subject](medic-src/src/shell/shell_config.rs)

use super::*;
use crate::runnable::Runnable;
use std::collections::BTreeMap;

#[test]
fn test_deserialize() {
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
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn test_deserialize_cd() {
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
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn test_deserialize_env() {
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
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn test_deserialize_verbose() {
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
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: true,
    }
  );
}

#[test]
fn test_deserialize_allow_failure() {
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
      platform: None,
      name: "Run some command".to_string(),
      remedy: None,
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn test_to_command() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: None,
    env: BTreeMap::default(),
    inline: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };

  let cmd = shell.to_command().unwrap();
  let cmd_str = format!("{cmd:?}");

  assert_eq!(cmd_str, "\"sh\" \"-c\" \"some command\"")
}

#[test]
fn test_to_command_cd() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: Some("../fixtures/bin".to_string()),
    env: BTreeMap::default(),
    inline: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };

  let mut context = std::collections::HashMap::new();
  for (key, value) in std::env::vars() {
    if value.contains(['{', '}']) {
      continue;
    };
    context.insert(key, value);
  }
  let path_expansion = envsubst::substitute("${PWD}/fixtures/bin", &context).unwrap();
  let expected_cmd_str = format!("cd \"{path_expansion}\" && \"sh\" \"-c\" \"some command\"");

  let cmd = shell.to_command().unwrap();
  let cmd_str = format!("{cmd:?}");

  assert_eq!(cmd_str, expected_cmd_str)
}

#[test]
fn test_to_command_env() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: None,
    env: BTreeMap::from([
      ("VAR".to_string(), "value".to_string()),
      ("OTHER".to_string(), "other".to_string()),
    ]),
    inline: false,
    platform: None,
    name: "Run some command".to_string(),
    remedy: Some("do something".to_string()),
    shell: "some command".to_string(),
    verbose: false,
  };
  let expected_cmd_str = "OTHER=\"other\" VAR=\"value\" \"sh\" \"-c\" \"some command\"".to_string();

  let cmd = shell.to_command().unwrap();
  let cmd_str = format!("{cmd:?}");

  assert_eq!(cmd_str, expected_cmd_str)
}

#[test]
fn test_deserialize_remedy() {
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
      platform: None,
      name: "Run some command".to_string(),
      remedy: Some("do something".to_string()),
      shell: "some command".to_string(),
      verbose: false,
    }
  );
}

#[test]
fn test_to_string() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: None,
    env: BTreeMap::default(),
    inline: false,
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
fn test_to_string_cd() {
  let shell = ShellConfig {
    allow_failure: false,
    cd: Some("../fixtures/bin".to_string()),
    env: BTreeMap::default(),
    inline: false,
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
