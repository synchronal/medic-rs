use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Shell(ShellConfig),
    Step(StepConfig),
}

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

impl Step {
    pub fn to_command(self) -> Option<Command> {
        match self {
            Step::Shell(config) => {
                let cmd: Vec<&str> = config.shell.split(' ').collect();
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
            Step::Step(config) => {
                let mut check_cmd: String = "medic-step-".to_owned();
                check_cmd.push_str(&config.step);
                let mut command = Command::new(check_cmd);

                if let Some(subcmd) = config.command {
                    command.arg(subcmd);
                }
                if let Some(args) = config.args {
                    for (flag, value) in args {
                        let mut flag_arg = "--".to_owned();
                        flag_arg.push_str(&flag);
                        command.arg(flag_arg).arg(value);
                    }
                }

                Some(command)
            }
        }
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Step::Shell(config) => {
                write!(f, "\x1b[36m{:}", config.name)
            }
            Step::Step(config) => {
                if let Some(name) = &config.name {
                    write!(f, "\x1b[36m{:}\x1b[0m", name)
                } else {
                    write!(f, "\x1b[36m{:}", config.step)?;
                    if let Some(command) = &config.command {
                        write!(f, ": \x1b[0;36m{}!", command)?;
                    }
                    if let Some(args) = &config.args {
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
    }
}
