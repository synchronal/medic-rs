pub mod check_result;
pub use check_result::CheckResult;

use crate::runnable::Runnable;
use crate::std_to_string;
use crate::AppResult;

use arboard::Clipboard;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Check {
    pub check: String,
    pub command: Option<String>,
    pub args: Option<HashMap<String, String>>,
    #[serde(default)]
    pub verbose: bool,
}

impl Runnable for Check {
    fn run(self) -> AppResult<()> {
        let verbose = self.verbose();

        print!("\x1b[32m• \x1b[0");
        print!("{self}  …");
        if let Some(mut command) = self.to_command() {
            match command.output() {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                        Ok(())
                    } else {
                        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                        if !verbose {
                            eprintln!("\x1b[0;31m== Check output ==\x1b[0m\r\n");
                            eprint!("{}", std_to_string(result.stderr));
                        }

                        if result.stdout.is_empty() {
                            println!("\x1b[0;33mNo remedy suggested.\x1b[0m");
                        } else {
                            let remedy = std_to_string(result.stdout).trim().to_owned();
                            print!("\x1b[36mPossible remedy: \x1b[0;33m{remedy}\x1b[0m");
                            print!("  \x1b[32;1m(it's in the clipboard)\x1b[0m\r\n");

                            let mut clipboard = Clipboard::new().unwrap();
                            clipboard.set_text(remedy).unwrap();
                        }
                        Err("".into())
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
            Err("Unable to parse check".into())
        }
    }
    fn to_command(self) -> Option<Command> {
        let mut check_cmd: String = "medic-check-".to_owned();
        check_cmd.push_str(&self.check);
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

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[36m{:}", self.check)?;
        if let Some(command) = &self.command {
            write!(f, ": \x1b[0;36m{}?", command)?;
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
