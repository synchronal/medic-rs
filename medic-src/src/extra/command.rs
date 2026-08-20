use std::collections::BTreeMap;
use std::process::Command;

pub fn to_string(command: &String, dir: &Option<String>) -> String {
  match dir {
    Some(dir) => format!("(cd {dir} && {command})"),
    None => command.to_string(),
  }
}

pub fn from_string(cmd: &str, dir: &Option<String>, env: &BTreeMap<String, String>) -> Command {
  let mut command = new("sh", dir, env);
  command.arg("-c").arg(cmd);
  command
}

pub fn new(cmd: &str, dir: &Option<String>, env: &BTreeMap<String, String>) -> Command {
  let mut command = std::process::Command::new(cmd);
  with_env(&mut command, env);
  with_dir(&mut command, dir);

  command
}

pub fn with_dir(cmd: &mut Command, dir: &Option<String>) {
  if let Some(dir) = dir {
    let expanded = std::fs::canonicalize(dir).unwrap();
    cmd.current_dir(&expanded);
  };
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
