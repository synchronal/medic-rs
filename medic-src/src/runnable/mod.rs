use crate::cli::Flags;
use crate::recoverable::Recoverable;
use crate::AppResult;

pub trait Runnable: std::fmt::Display + Clone {
  fn allow_failure(&self) -> bool {
    false
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()>;
  fn to_command(&self) -> Result<std::process::Command, Box<dyn std::error::Error>>;
  fn verbose(&self) -> bool {
    false
  }
}

pub fn run(runnable: impl Runnable + Clone, progress: &mut retrogress::ProgressBar, flags: &Flags) -> AppResult<()> {
  let _ = flags;
  match runnable.clone().run(progress) {
    Recoverable::Ok(ok) => AppResult::Ok(ok),
    Recoverable::Err(err, None) => AppResult::Err(err),
    Recoverable::Err(err, Some(_remedy)) => AppResult::Err(err),
    Recoverable::Optional(ok, None) => AppResult::Ok(ok),
    Recoverable::Optional(ok, Some(_remedy)) => AppResult::Ok(ok),
  }
}
