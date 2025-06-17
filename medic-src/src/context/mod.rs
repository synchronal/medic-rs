use std::process::Command;

pub struct Context {
  pub platform: String,
}

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}

impl Context {
  pub fn new() -> Self {
    Self {
      platform: current_platform(),
    }
  }

  pub fn matches_platform(&self, platforms: &Option<Vec<String>>) -> bool {
    match platforms {
      None => true,
      Some(platform_list) if platform_list.is_empty() => true,
      Some(platform_list) => platform_list.iter().any(|p| p == &self.platform),
    }
  }
}

fn current_platform() -> String {
  let output = match Command::new("uname").output() {
    Ok(output) => output,
    Err(_e) => return "Unknown".into(),
  };
  if !output.status.success() {
    return "Unknown".into();
  }

  match String::from_utf8(output.stdout) {
    Ok(s) => s.trim().into(),
    Err(_e) => "Unknown".into(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_current_platform() {
    let result = current_platform();
    assert!(!result.is_empty());
    // Should be something like "Darwin", "Linux", etc.
    assert!(result.chars().all(|c| c.is_ascii_alphanumeric()));
  }

  #[test]
  fn test_platform_matches_none() {
    let context = Context {
      platform: "Something".into(),
    };
    assert!(context.matches_platform(&None));
  }

  #[test]
  fn test_platform_matches_empty() {
    let context = Context {
      platform: "Something".into(),
    };
    assert!(context.matches_platform(&Some(vec![])));
  }

  #[test]
  fn test_platform_matches_current() {
    let context = Context {
      platform: "Something".into(),
    };
    assert!(context.matches_platform(&Some(vec!["Something".into(), "Other".into()])));
  }

  #[test]
  fn test_platform_matches_not_current() {
    let context = Context {
      platform: "Something".into(),
    };
    assert!(!context.matches_platform(&Some(vec!["SomethingElse".into(), "Other".into()])));
  }
}
