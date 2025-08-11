#[derive(Clone)]
pub struct Flags {
  pub auto_apply_remedy: bool,
  pub interactive: bool,
  pub parallel: bool,
  pub recoverable: bool,
}

impl Default for Flags {
  fn default() -> Self {
    Self {
      auto_apply_remedy: false,
      interactive: false,
      parallel: false,
      recoverable: true,
    }
  }
}
