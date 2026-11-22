use crate::cli::Flags;
use retrogress::ProgressBar;

pub fn new(flags: &Flags) -> ProgressBar {
  if flags.parallel {
    let parallel = retrogress::Parallel::new().failure_output(retrogress::FailureOutput::Full);
    retrogress::ProgressBar::new(Box::new(parallel))
  } else {
    retrogress::ProgressBar::new(retrogress::Sync::boxed())
  }
}
