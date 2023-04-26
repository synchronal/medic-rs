pub trait Runnable {
    fn to_command(self) -> Option<std::process::Command>;
    fn verbose(&self) -> bool {
        false
    }
}
