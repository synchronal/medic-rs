use crate::runnable::Runnable;

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct StepConfig {
    pub args: Option<HashMap<String, String>>,
    pub command: Option<String>,
    pub name: Option<String>,
    pub step: String,
    #[serde(default)]
    pub verbose: bool,
}

impl Runnable for StepConfig {
    fn to_command(self) -> Option<Command> {
        let mut check_cmd: String = "medic-step-".to_owned();
        check_cmd.push_str(&self.step);
        let mut command = Command::new(check_cmd);

        if let Some(subcmd) = self.command {
            command.arg(subcmd);
        }
        if let Some(args) = self.args {
            for (flag, value) in args {
                let mut flag_arg = "--".to_owned();
                flag_arg.push_str(&flag);
                command.arg(flag_arg).arg(value);
            }
        }

        Some(command)
    }

    fn verbose(&self) -> bool {
        self.verbose
    }
}

impl fmt::Display for StepConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "\x1b[36m{:}\x1b[0m", name)
        } else {
            write!(f, "\x1b[36m{:}", self.step)?;
            if let Some(command) = &self.command {
                write!(f, ": \x1b[0;36m{}!", command)?;
            }
            if let Some(args) = &self.args {
                write!(f, " \x1b[0;33m(")?;
                for value in args.values() {
                    write!(f, "{value}")?;
                }
                write!(f, ")")?;
            }
            write!(f, "\x1b[0m")
        }
    }
}
