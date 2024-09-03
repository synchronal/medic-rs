use crate::cli::Flags;
use crate::recoverable::Recoverable;
use crate::AppResult;

pub trait Runnable: std::fmt::Display {
  fn allow_failure(&self) -> bool {
    false
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()>;
  fn to_command(&self) -> Result<std::process::Command, Box<dyn std::error::Error>>;
  fn verbose(&self) -> bool {
    false
  }
}

pub fn run(runnable: impl Runnable, progress: &mut retrogress::ProgressBar, _flags: &[Flags]) -> AppResult<()> {
  runnable.run(progress).into()
}
