use std::path::PathBuf;

#[derive(Clone)]
pub struct Flags {
  pub auto_apply_remedy: bool,
  pub config_path: PathBuf,
  pub interactive: bool,
  pub parallel: bool,
  pub recoverable: bool,
}

impl Default for Flags {
  fn default() -> Self {
    Self {
      auto_apply_remedy: false,
      config_path: "${PWD}/.config/medic.toml".into(),
      interactive: false,
      parallel: false,
      recoverable: true,
    }
  }
}
