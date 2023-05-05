pub mod check_result;
pub use check_result::CheckResult;

use crate::runnable::Runnable;
use crate::std_to_string;
use crate::AppResult;

use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::process::{Command, Stdio};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum OutputFormat {
    // #[serde(rename(deserialize = "json"))]
    // Json,
    #[default]
    #[serde(rename(deserialize = "stdio"))]
    Stdio,
}

impl OutputFormat {
    fn parse(self, result: std::process::Output) -> CheckOutput {
        match self {
            OutputFormat::Stdio => {
                let stderr = if result.stderr.is_empty() {
                    None
                } else {
                    Some(std_to_string(result.stderr))
                };
                let remedy = if result.stdout.is_empty() {
                    None
                } else {
                    Some(std_to_string(result.stdout).trim().to_owned())
                };

                CheckOutput {
                    stderr,
                    remedy,
                    _stdout: None,
                }
            }
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // OutputFormat::Json => write!(f, "json"),
            OutputFormat::Stdio => write!(f, "stdio"),
        }
    }
}

pub struct CheckOutput {
    _stdout: Option<String>,
    stderr: Option<String>,
    remedy: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Check {
    pub args: Option<HashMap<String, String>>,
    pub check: String,
    pub command: Option<String>,
    #[serde(default)]
    pub output_format: OutputFormat,
    #[serde(default)]
    pub verbose: bool,
}

impl Runnable for Check {
    fn run(self) -> AppResult<()> {
        let verbose = self.verbose();

        print!("\x1b[32m• \x1b[0");
        print!("{self}  …");
        if let Some(mut command) = self.to_command() {
            if verbose {
                print!("\r\n");
                command.stderr(Stdio::inherit());
            }
            match command.output() {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                        AppResult::Ok(())
                    } else {
                        let output = self.output_format.parse(result);
                        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                        if !verbose && output.stderr.is_some() {
                            eprintln!("\x1b[0;31m== Check output ==\x1b[0m\r\n");
                            eprint!("{}", output.stderr.unwrap());
                        }

                        if output.remedy.is_none() {
                            println!("\x1b[0;33mNo remedy suggested.\x1b[0m");
                        } else {
                            let remedy = output.remedy.unwrap();
                            print!("\x1b[36mPossible remedy: \x1b[0;33m{remedy}\x1b[0m");
                            print!("  \x1b[32;1m(it's in the clipboard)\x1b[0m\r\n");

                            let mut clipboard = Clipboard::new().unwrap();
                            clipboard.set_text(remedy).unwrap();
                        }
                        AppResult::Err(None)
                    }
                }
                Err(err) => {
                    println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                    AppResult::Err(Some(err.into()))
                }
            }
        } else {
            AppResult::Err(Some("Unable to parse check".into()))
        }
    }
    fn to_command(&self) -> Option<Command> {
        let mut check_cmd: String = "medic-check-".to_owned();
        check_cmd.push_str(&self.check);
        let mut command = Command::new(check_cmd);
        command.env("MEDIC_OUTPUT_FORMAT", self.output_format.to_string());

        if let Some(subcmd) = &self.command {
            command.arg(subcmd);
        }
        if let Some(args) = &self.args {
            for (flag, value) in args {
                let mut flag_arg = "--".to_owned();
                flag_arg.push_str(flag);
                command.arg(flag_arg).arg(value);
            }
        }

        Some(command)
    }

    fn verbose(&self) -> bool {
        self.verbose
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
