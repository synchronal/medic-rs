use crate::cli::Flags;
use retrogress::ProgressBar;

pub fn new(flags: &Flags) -> ProgressBar {
  if flags.parallel {
    retrogress::ProgressBar::new(retrogress::Parallel::boxed())
  } else {
    retrogress::ProgressBar::new(retrogress::Sync::boxed())
  }
}
