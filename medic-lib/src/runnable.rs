use crate::AppResult;

pub trait Runnable {
    fn allow_failure(&self) -> bool {
        false
    }

    fn run(self) -> AppResult<()>;
    fn to_command(&self) -> Option<std::process::Command>;
    fn verbose(&self) -> bool {
        false
    }
}
