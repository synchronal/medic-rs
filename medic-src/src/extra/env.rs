use std::collections::HashMap;

pub fn subst(string: &str) -> Result<String, Box<dyn std::error::Error>> {
  let cwd = std::env::current_dir()?
    .into_os_string()
    .into_string()
    .map_err(|_| "Failed to convert current directory to string")?;

  let mut context = HashMap::new();
  context.insert("CWD".to_string(), cwd);
  for (key, value) in std::env::vars() {
    if value.contains(['{', '}', '$']) {
      continue;
    };
    context.insert(key, value);
  }

  Ok(envsubst::substitute(string, &context)?)
}
