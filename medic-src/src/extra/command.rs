pub fn to_string(command: &String, dir: &Option<String>) -> String {
  match dir {
    Some(dir) => format!("(cd {} && {})", dir, command),
    None => format!("({})", command),
  }
}

pub fn from_string(command: &str, dir: &Option<String>) -> std::process::Command {
  let filtered_env: std::collections::HashMap<String, String> = std::env::vars()
    .filter(|(_k, v)| !v.contains(['{', '}']))
    .collect();

  let mut remedy_chunks = command.split_whitespace();
  let cmd = remedy_chunks.next().unwrap();
  let args: Vec<&str> = remedy_chunks.collect();

  let mut command = std::process::Command::new(cmd);
  command.args(args).env_clear().envs(&filtered_env);

  if let Some(dir) = dir {
    let expanded = std::fs::canonicalize(dir).unwrap();
    command.current_dir(&expanded);
  };

  command
}
