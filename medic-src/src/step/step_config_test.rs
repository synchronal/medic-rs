// @related [subject](medic-src/src/step/step_config.rs)

use super::*;
use crate::util::StringOrList;
use std::collections::BTreeMap;
use std::ffi::OsStr;

#[test]
fn deserialize_cd() {
  let toml = r#"
        cd = "./subdirectory"
        step = "step-name"
        "#;

  let result: StepConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    StepConfig {
      args: None,
      cd: Some("./subdirectory".to_string()),
      command: None,
      env: BTreeMap::default(),
      name: None,
      platform: None,
      step: "step-name".to_string(),
      verbose: false
    }
  )
}
#[test]
fn deserialize_arg_string() {
  let toml = r#"
        step = "step-name"
        command = "subcommand"
        args = { name = "first" }
        "#;

  let result: StepConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    StepConfig {
      args: Some(BTreeMap::from([(
        "name".to_string(),
        StringOrList(vec!["first".to_string()])
      )])),
      cd: None,
      command: Some("subcommand".to_string()),
      env: BTreeMap::default(),
      name: None,
      platform: None,
      step: "step-name".to_string(),
      verbose: false
    }
  )
}
#[test]
fn deserialize_arg_list() {
  assert!(false);
  let toml = r#"
        step = "step-name"
        command = "subcommand"
        args = { name = ["first", "second"] }
        "#;

  let result: StepConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    StepConfig {
      args: Some(BTreeMap::from([(
        "name".to_string(),
        StringOrList(vec!["first".to_string(), "second".to_string()])
      )])),
      cd: None,
      command: Some("subcommand".to_string()),
      env: BTreeMap::default(),
      name: None,
      platform: None,
      step: "step-name".to_string(),
      verbose: false
    }
  )
}
#[test]
fn deserialize_env() {
  let toml = r#"
        env = { MY_VAR = "first", SECOND_VAR = "second" }
        step = "step-name"
        "#;

  let result: StepConfig = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    StepConfig {
      args: None,
      cd: None,
      command: None,
      env: BTreeMap::from([
        ("MY_VAR".to_string(), "first".to_string()),
        ("SECOND_VAR".to_string(), "second".to_string())
      ]),
      name: None,
      platform: None,
      step: "step-name".to_string(),
      verbose: false
    }
  )
}

#[test]
fn to_command() {
  let step = StepConfig {
    args: None,
    cd: None,
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "thing".to_string(),
    verbose: false,
  };

  let cmd = step.to_command().unwrap();

  assert_eq!(cmd.get_current_dir(), None);
  assert_eq!(cmd.get_program(), "medic-step-thing");

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![];
  assert_eq!(args, expected_args);

  // let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
  // let expected_envs: Vec<(&OsStr, Option<&OsStr>)> = vars_as_osstr();
  // assert_eq!(envs, expected_envs);
}

#[test]
fn to_command_cd() {
  let step = StepConfig {
    args: None,
    cd: Some("../fixtures/bin".to_string()),
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "thing".to_string(),
    verbose: false,
  };

  let path_expansion = extra::env::subst("${PWD}/fixtures/bin").unwrap();
  let path: std::path::PathBuf = path_expansion.into();
  let cmd = step.to_command().unwrap();

  assert_eq!(cmd.get_current_dir(), Some(path.as_path()));
  assert_eq!(cmd.get_program(), "medic-step-thing");
}

#[test]
fn to_command_subcommand() {
  let step = StepConfig {
    args: None,
    cd: None,
    command: Some("sub-command".to_string()),
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "thing".to_string(),
    verbose: false,
  };

  let cmd = step.to_command().unwrap();

  assert_eq!(cmd.get_program(), "medic-step-thing");

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![OsStr::new("sub-command")];
  assert_eq!(args, expected_args);
}

#[test]
fn to_command_args() {
  let step = StepConfig {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string()]),
    )])),
    cd: None,
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "thing".to_string(),
    verbose: false,
  };

  let cmd = step.to_command().unwrap();

  assert_eq!(cmd.get_program(), "medic-step-thing");

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![OsStr::new("--name"), OsStr::new("first")];
  assert_eq!(args, expected_args);
}

#[test]
fn to_command_args_list() {
  let step = StepConfig {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string(), "second".to_string()]),
    )])),
    cd: None,
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "thing".to_string(),
    verbose: false,
  };

  let cmd = step.to_command().unwrap();

  assert_eq!(cmd.get_program(), "medic-step-thing");

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args = vec![
    OsStr::new("--name"),
    OsStr::new("first"),
    OsStr::new("--name"),
    OsStr::new("second"),
  ];
  assert_eq!(args, expected_args);
}

#[test]
fn to_command_env() {
  let step = StepConfig {
    args: None,
    cd: None,
    command: None,
    env: BTreeMap::from([
      ("VAR".to_string(), "value".to_string()),
      ("OTHER".to_string(), "other".to_string()),
    ]),
    name: None,
    platform: None,
    step: "thing".to_string(),
    verbose: false,
  };

  let cmd = step.to_command().unwrap();

  let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
  let expected = [
    (OsStr::new("OTHER"), Some(OsStr::new("other"))),
    (OsStr::new("VAR"), Some(OsStr::new("value"))),
  ];

  assert!(expected.iter().all(|item| envs.contains(item)));
}

#[test]
fn to_command_missing_command() {
  let step = StepConfig {
    args: None,
    cd: None,
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "missing".to_string(),
    verbose: false,
  };

  let e = step.to_command().err().unwrap();
  assert_eq!(
    format!("{e}"),
    "executable medic-step-missing not found in PATH".to_string()
  );
}

#[test]
fn to_string_cd() {
  let step = StepConfig {
    args: None,
    cd: Some("./subdirectory".to_string()),
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "step-name".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{step}"),
    "\u{1b}[36mstep-name\u{1b}[0m \u{1b}[32m(./subdirectory)\u{1b}[0m"
  )
}

#[test]
fn to_string_single_arg() {
  let step = StepConfig {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string()]),
    )])),
    cd: None,
    command: None,
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "step-name".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{step}"),
    "\u{1b}[36mstep-name\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
  )
}

#[test]
fn to_string_subcommand_single_arg() {
  let step = StepConfig {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string()]),
    )])),
    cd: None,
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "step-name".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{step}"),
    "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
  )
}

#[test]
fn to_string_subcommand_multiple_args() {
  let step = StepConfig {
    args: Some(BTreeMap::from([
      ("name".to_string(), StringOrList(vec!["first".to_string()])),
      ("version".to_string(), StringOrList(vec!["second".to_string()])),
    ])),
    cd: None,
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "step-name".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{step}"),
    "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, version: second)\u{1b}[0m"
  )
}

#[test]
fn to_string_subcommand_multiple_arg_values() {
  let step = StepConfig {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string(), "second".to_string()]),
    )])),
    cd: None,
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "step-name".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{step}"),
    "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second)\u{1b}[0m"
  )
}

#[test]
fn to_string_subcommand_multiple_arg_values_and_args() {
  let step = StepConfig {
    args: Some(BTreeMap::from([
      (
        "name".to_string(),
        StringOrList(vec!["first".to_string(), "second".to_string()]),
      ),
      ("other".to_string(), StringOrList(vec!["third".to_string()])),
    ])),
    cd: None,
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    name: None,
    platform: None,
    step: "step-name".to_string(),
    verbose: false,
  };

  assert_eq!(
    format!("{step}"),
    "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second, other: third)\u{1b}[0m"
  )
}
