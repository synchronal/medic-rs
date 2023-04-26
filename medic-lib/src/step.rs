use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Shell(ShellConfig),
    Step(StepConfig),
    Doctor(DoctorConfig),
}

#[derive(Debug, Deserialize)]
pub struct DoctorConfig {}

#[derive(Debug, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub allow_failure: bool,
    pub name: String,
    pub shell: String,
    #[serde(default)]
    pub verbose: bool,
}

#[derive(Debug, Deserialize)]
pub struct StepConfig {
    pub args: Option<HashMap<String, String>>,
    pub command: Option<String>,
    pub name: Option<String>,
    pub step: String,
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

impl StepConfig {
    pub fn to_command(self) -> Option<Command> {
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

impl Step {
    pub fn to_command(self) -> Option<Command> {
        match self {
            Step::Shell(config) => config.to_command(),
            Step::Step(config) => config.to_command(),
            Step::Doctor(_) => doctor_command(),
        }
    }

    pub fn verbose(&self) -> bool {
        match self {
            Step::Shell(config) => config.verbose,
            Step::Step(config) => config.verbose,
            Step::Doctor(_) => true,
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Step::Shell(config) => config.fmt(f),
            Step::Step(config) => config.fmt(f),
            Step::Doctor(_) => write!(f, "\x1b[36m== Doctor ===\x1b[0m"),
        }
    }
}

fn doctor_command() -> Option<Command> {
    let mut command = Command::new("medic");
    command.arg("doctor");
    Some(command)
}
