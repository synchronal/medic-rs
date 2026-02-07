use std::collections::BTreeMap;

pub fn to_string(command: &String, dir: &Option<String>) -> String {
  match dir {
    Some(dir) => format!("(cd {dir} && {command})"),
    None => command.to_string(),
  }
}

pub fn from_string(cmd: &str, dir: &Option<String>, env: &BTreeMap<String, String>) -> std::process::Command {
  let mut command = new("sh", dir, env);
  command.arg("-c").arg(cmd);
  command
}

pub fn new(cmd: &str, dir: &Option<String>, env: &BTreeMap<String, String>) -> std::process::Command {
  let mut filtered_env: BTreeMap<String, String> = std::env::vars()
    .filter(|(_k, v)| !v.contains(['{', '}']))
    .collect();

  for (key, value) in env {
    filtered_env.insert(key.clone(), value.clone());
  }

  let mut command = std::process::Command::new(cmd);
  command.env_clear().envs(&filtered_env);
  if let Some(dir) = dir {
    let expanded = std::fs::canonicalize(dir).unwrap();
    command.current_dir(&expanded);
  };
  command
}
