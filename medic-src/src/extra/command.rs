pub fn to_string(command: &String, dir: &Option<String>) -> String {
  match dir {
    Some(dir) => format!("(cd {} && {})", dir, command),
    None => format!("({})", command),
  }
}

pub fn from_string(cmd: &str, dir: &Option<String>) -> std::process::Command {
  let mut command = new("sh", dir);
  command.arg("-c").arg(cmd);

  command
}

pub fn new(cmd: &str, dir: &Option<String>) -> std::process::Command {
  let filtered_env: std::collections::HashMap<String, String> = std::env::vars()
    .filter(|(_k, v)| !v.contains(['{', '}']))
    .collect();

  let mut command = std::process::Command::new(cmd);
  command.env_clear().envs(&filtered_env);
  if let Some(dir) = dir {
    let expanded = std::fs::canonicalize(dir).unwrap();
    command.current_dir(&expanded);
  };
  command
}
