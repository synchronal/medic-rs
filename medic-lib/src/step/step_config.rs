use crate::runnable::Runnable;
use crate::std_to_string;
use crate::AppResult;

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
use std::process::{Command, Stdio};

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
    fn run(self) -> AppResult<()> {
        let allow_failure = self.allow_failure();
        let verbose = self.verbose();

        print!("\x1b[32m• \x1b[0{self}  …");
        io::stdout().flush().unwrap();
        if let Some(mut command) = self.to_command() {
            if verbose {
                print!("\r\n");
                command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            }
            match command.output() {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                        Ok(())
                    } else {
                        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                        eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                        eprint!("{}", std_to_string(result.stderr));
                        if allow_failure {
                            eprintln!("\x1b[32m(continuing)\x1b[0m");
                            Ok(())
                        } else {
                            Err("".into())
                        }
                    }
                }
                Err(err) => {
                    println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                    let mut error: String = "Check failed!\r\n".to_owned();
                    error.push_str("Command:\r\n");
                    error.push_str(&format!("{command:?}\r\n"));
                    error.push_str(&format!("Error:\r\n{err:?}"));

                    Err(error.into())
                }
            }
        } else {
            Err("Failed to parse command".into())
        }
    }
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
