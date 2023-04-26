use crate::runnable::Runnable;
use crate::std_to_string;
use crate::AppResult;

use serde::Deserialize;
use std::fmt;
use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;

#[derive(Debug, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub allow_failure: bool,
    pub name: String,
    pub shell: String,
    #[serde(default)]
    pub verbose: bool,
}

impl Runnable for ShellConfig {
    fn allow_failure(&self) -> bool {
        self.allow_failure
    }

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

    fn verbose(&self) -> bool {
        self.verbose
    }
}
impl fmt::Display for ShellConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[36m{:}", self.name)
    }
}
