// @related [test](medic-src/src/extra/command_test.rs)

use crate::error::MedicError;
use std::collections::BTreeMap;
use std::process::Command;

pub fn to_string(command: &String, dir: &Option<String>) -> String {
  match dir {
    Some(dir) => format!("(cd {dir} && {command})"),
    None => command.to_string(),
  }
}

pub fn from_string(cmd: &str, dir: &Option<String>, env: &BTreeMap<String, String>) -> Result<Command, MedicError> {
  let mut command = new("sh", dir, env)?;
  command.arg("-c").arg(cmd);
  Ok(command)
}

pub fn new(cmd: &str, dir: &Option<String>, env: &BTreeMap<String, String>) -> Result<Command, MedicError> {
  let mut command = std::process::Command::new(cmd);
  with_env(&mut command, env);
  with_dir(&mut command, dir)?;

  Ok(command)
}

pub fn with_dir(cmd: &mut Command, dir: &Option<String>) -> Result<(), MedicError> {
  if let Some(dir) = dir {
    let expanded = std::fs::canonicalize(dir).map_err(|err| match err.kind() {
      std::io::ErrorKind::NotFound => MedicError::Message(format!("directory {dir} does not exist")),
      _ => MedicError::Message(format!("unable to resolve directory {dir}: {err}")),
    })?;
    cmd.current_dir(&expanded);
  };
  Ok(())
}

pub fn with_env(cmd: &mut Command, env: &BTreeMap<String, String>) {
  let mut filtered_env: BTreeMap<String, String> = std::env::vars()
    .filter(|(_k, v)| !v.contains(['{', '}']))
    .collect();

  for (key, value) in env {
    filtered_env.insert(key.clone(), value.clone());
  }

  cmd.env_clear().envs(&filtered_env);
}
