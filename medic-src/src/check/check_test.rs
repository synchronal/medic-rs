// @related [subject](medic-src/src/check/mod.rs)

use std::ffi::OsStr;

use super::*;

pub fn initialize() {
  crate::theme::THEME.get_or_init(crate::theme::dark_theme);
}

#[test]
fn arg_string() {
  let toml = r#"
        check = "check-name"
        command = "subcommand"
        args = { name = "first" }
        "#;

  let result: Check = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    Check {
      args: Some(BTreeMap::from([(
        "name".to_string(),
        StringOrList(vec!["first".to_string()])
      )])),
      cd: None,
      check: "check-name".to_string(),
      command: Some("subcommand".to_string()),
      env: BTreeMap::default(),
      manual: false,
      output: OutputFormat::Json,
      platform: None,
      verbose: false
    }
  )
}
#[test]
fn arg_list() {
  let toml = r#"
        check = "check-name"
        command = "subcommand"
        args = { name = ["first", "second"] }
        "#;

  let result: Check = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    Check {
      args: Some(BTreeMap::from([(
        "name".to_string(),
        StringOrList(vec!["first".to_string(), "second".to_string()])
      )])),
      cd: None,
      check: "check-name".to_string(),
      command: Some("subcommand".to_string()),
      env: BTreeMap::default(),
      manual: false,
      output: OutputFormat::Json,
      platform: None,
      verbose: false
    }
  )
}

#[test]
fn cd_string() {
  let toml = r#"
        cd = "./subdirectory"
        check = "check-name"
        "#;

  let result: Check = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    Check {
      args: None,
      cd: Some("./subdirectory".to_string()),
      check: "check-name".to_string(),
      command: None,
      env: BTreeMap::default(),
      manual: false,
      output: OutputFormat::Json,
      platform: None,
      verbose: false
    }
  )
}

#[test]
fn env() {
  let toml = r#"
        check = "check-name"
        env = { MY_VAR = "first", SECOND_VAR = "second" }
        "#;

  let result: Check = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    Check {
      args: None,
      cd: None,
      check: "check-name".to_string(),
      command: None,
      env: BTreeMap::from([
        ("MY_VAR".to_string(), "first".to_string()),
        ("SECOND_VAR".to_string(), "second".to_string())
      ]),
      manual: false,
      output: OutputFormat::Json,
      platform: None,
      verbose: false
    }
  )
}

#[test]
fn manual_string() {
  let toml = r#"
        check = "check-name"
        manual = true
        "#;

  let result: Check = toml::from_str(toml).unwrap();
  assert_eq!(
    result,
    Check {
      args: None,
      cd: None,
      check: "check-name".to_string(),
      command: None,
      env: BTreeMap::default(),
      manual: true,
      output: OutputFormat::Json,
      platform: None,
      verbose: false
    }
  )
}

#[test]
fn command() {
  let check = Check {
    args: None,
    cd: None,
    check: "json".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  let cmd = check.to_command().unwrap();

  assert_eq!(cmd.get_current_dir(), None);
  assert_eq!(cmd.get_program(), "medic-check-json");

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![];
  assert_eq!(args, expected_args);

  let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
  let expected = [(OsStr::new("MEDIC_OUTPUT_FORMAT"), Some(OsStr::new("json")))];

  assert!(expected.iter().all(|item| envs.contains(item)));
}

#[test]
fn cd_relative() {
  let check = Check {
    args: None,
    cd: Some("../fixtures/bin".to_string()),
    check: "json".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };
  let path_expansion = extra::env::subst("${PWD}/fixtures/bin").unwrap();

  let path: std::path::PathBuf = path_expansion.into();
  let cmd = check.to_command().unwrap();

  assert_eq!(cmd.get_current_dir(), Some(path.as_path()));
}

#[test]
fn subcommand() {
  let check = Check {
    args: None,
    cd: None,
    check: "json".to_string(),
    command: Some("sub-command".to_string()),
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  let cmd = check.to_command().unwrap();

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![OsStr::new("sub-command")];
  assert_eq!(args, expected_args);
}

#[test]
fn command_env() {
  let check = Check {
    args: None,
    cd: None,
    check: "json".to_string(),
    command: None,
    env: BTreeMap::from([
      ("VAR".to_string(), "value".to_string()),
      ("OTHER".to_string(), "other".to_string()),
    ]),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  let cmd = check.to_command().unwrap();

  let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
  let expected = [
    (OsStr::new("OTHER"), Some(OsStr::new("other"))),
    (OsStr::new("VAR"), Some(OsStr::new("value"))),
  ];

  assert!(expected.iter().all(|item| envs.contains(item)));
}

#[test]
fn stdio() {
  let check = Check {
    args: None,
    cd: None,
    check: "json".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Stdio,
    platform: None,
    verbose: false,
  };

  let cmd = check.to_command().unwrap();

  let envs: Vec<(&OsStr, Option<&OsStr>)> = cmd.get_envs().collect();
  let expected = [(OsStr::new("MEDIC_OUTPUT_FORMAT"), Some(OsStr::new("stdio")))];

  assert!(expected.iter().all(|item| envs.contains(item)));
}

#[test]
fn args() {
  let check = Check {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string()]),
    )])),
    cd: None,
    check: "json".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  let cmd = check.to_command().unwrap();

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![OsStr::new("--name"), OsStr::new("first")];
  assert_eq!(args, expected_args);
}

#[test]
fn args_list() {
  let check = Check {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string(), "second".to_string()]),
    )])),
    cd: None,
    check: "json".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  let cmd = check.to_command().unwrap();

  let args: Vec<&OsStr> = cmd.get_args().collect();
  let expected_args: Vec<&OsStr> = vec![
    OsStr::new("--name"),
    OsStr::new("first"),
    OsStr::new("--name"),
    OsStr::new("second"),
  ];
  assert_eq!(args, expected_args);
}

#[test]
fn missing_command() {
  let check = Check {
    args: None,
    cd: None,
    check: "missing".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  let e = check.to_command().err().unwrap();
  assert_eq!(
    format!("{e}"),
    "executable medic-check-missing not found in PATH".to_string()
  );
}

#[test]
fn single_arg() {
  initialize();
  let check = Check {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string()]),
    )])),
    cd: None,
    check: "check-name".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  assert_eq!(
    format!("{check}"),
    "\u{1b}[36mcheck-name\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
  )
}

#[test]
fn subcommand_single_arg() {
  initialize();
  let check = Check {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string()]),
    )])),
    cd: None,
    check: "check-name".to_string(),
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  assert_eq!(
    format!("{check}"),
    "\u{1b}[36mcheck-name: subcommand?\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
  )
}

#[test]
fn subcommand_args() {
  initialize();
  let check = Check {
    args: Some(BTreeMap::from([
      ("name".to_string(), StringOrList(vec!["first".to_string()])),
      ("version".to_string(), StringOrList(vec!["second".to_string()])),
    ])),
    cd: None,
    check: "check-name".to_string(),
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  assert_eq!(
    format!("{check}"),
    "\u{1b}[36mcheck-name: subcommand?\u{1b}[0m \u{1b}[33m(name: first, version: second)\u{1b}[0m"
  )
}

#[test]
fn subcommand_arg_values() {
  initialize();
  let check = Check {
    args: Some(BTreeMap::from([(
      "name".to_string(),
      StringOrList(vec!["first".to_string(), "second".to_string()]),
    )])),
    cd: None,
    check: "check-name".to_string(),
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  assert_eq!(
    format!("{check}"),
    "\u{1b}[36mcheck-name: subcommand?\u{1b}[0m \u{1b}[33m(name: first, name: second)\u{1b}[0m"
  )
}

#[test]
fn subcommand_arg_values_and_args() {
  initialize();
  let check = Check {
    args: Some(BTreeMap::from([
      (
        "name".to_string(),
        StringOrList(vec!["first".to_string(), "second".to_string()]),
      ),
      ("other".to_string(), StringOrList(vec!["third".to_string()])),
    ])),
    cd: None,
    check: "check-name".to_string(),
    command: Some("subcommand".to_string()),
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  assert_eq!(
    format!("{check}"),
    "\u{1b}[36mcheck-name: subcommand?\u{1b}[0m \u{1b}[33m(name: first, name: second, other: third)\u{1b}[0m"
  )
}

#[test]
fn display_cd() {
  initialize();
  let check = Check {
    args: None,
    cd: Some("../subdirectory".to_string()),
    check: "check-name".to_string(),
    command: None,
    env: BTreeMap::default(),
    manual: false,
    output: OutputFormat::Json,
    platform: None,
    verbose: false,
  };

  assert_eq!(
    format!("{check}"),
    "\u{1b}[36mcheck-name\u{1b}[0m \u{1b}[32m(../subdirectory)\u{1b}[0m"
  )
}

fn missing_dir(name: &str) -> String {
  std::env::temp_dir()
    .join(format!("medic-check-{}-{name}-missing", std::process::id()))
    .display()
    .to_string()
}

fn check_in(dir: &str) -> Check {
  toml::from_str(&format!("check = \"json\"\ncd = \"{dir}\"")).unwrap()
}

#[test]
fn missing_cd_command() {
  let missing = missing_dir("to-command");

  let err = check_in(&missing).to_command().unwrap_err();

  assert_eq!(err.to_string(), format!("directory {missing} does not exist"));
}

#[test]
fn missing_cd() {
  initialize();
  let missing = missing_dir("run");
  let mut flags = Flags::default();
  let mut progress = crate::progress::new(&flags);

  match check_in(&missing).run(&mut progress, &mut flags, &Context::new()) {
    Recoverable::Err(Some(err), None) => assert_eq!(
      err.to_string(),
      format!("Failed to parse command: directory {missing} does not exist")
    ),
    _ => panic!("expected Recoverable::Err(Some(_), None)"),
  }
}
