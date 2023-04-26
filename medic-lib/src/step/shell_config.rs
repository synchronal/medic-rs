use serde::Deserialize;
use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub allow_failure: bool,
    pub name: String,
    pub shell: String,
    #[serde(default)]
    pub verbose: bool,
}

impl ShellConfig {
    pub fn to_command(self) -> Option<Command> {
        let cmd: Vec<&str> = self.shell.split(' ').collect();
        if let Some((first, args)) = cmd.split_first() {
            let mut command = Command::new(first);
            for arg in args {
                command.arg(arg);
            }
            Some(command)
        } else {
            None
        }
    }
}
impl fmt::Display for ShellConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[36m{:}", self.name)
    }
}
