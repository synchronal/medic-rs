use crate::AppResult;

pub trait Runnable: std::fmt::Display {
    fn allow_failure(&self) -> bool {
        false
    }

    fn run(self, progress: &mut retrogress::ProgressBar) -> AppResult<()>;
    fn to_command(&self) -> Option<std::process::Command>;
    fn verbose(&self) -> bool {
        false
    }
}
