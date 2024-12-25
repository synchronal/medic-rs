#[cfg(test)]
mod manifest_test;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::load()));

pub mod manifest;
pub use manifest::Manifest;

pub fn current() -> Project {
  let current_dir = std::env::current_dir().unwrap();
  let cwd = current_dir.to_str().unwrap();

  let config = CONFIG.lock().unwrap();

  match config.projects.get(cwd) {
    Some(p) => p.clone(),
    None => {
      Project::default()
    }
  }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
  pub projects: HashMap<String, Project>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Project {
  pub skipped: Vec<String>,
}

pub trait Skippable {
  fn to_skipped(self: &Self) -> String;
}

impl Config {
  pub(crate) fn load() -> Self {
    let config_path = Self::path();
    if config_path.exists() {
      let config_contents = std::fs::read_to_string(config_path).unwrap();
      toml::from_str(&config_contents).unwrap()
    } else {
      Self {
        projects: HashMap::new(),
      }
    }
  }

  fn path() -> std::path::PathBuf {
    let mut context = std::collections::HashMap::new();
    for (key, value) in std::env::vars() {
      context.insert(key, value);
    }
    assert!(envsubst::validate_vars(&context).is_ok());

    let path_expansion = envsubst::substitute("${HOME}/.config/medic/config.toml", &context).unwrap();
    path_expansion.into()
  }
}
