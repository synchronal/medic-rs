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
    #[default]
    #[serde(rename(deserialize = "json"))]
    Json,
    #[serde(rename(deserialize = "stdio"))]
    Stdio,
}

impl OutputFormat {
    fn parse(self, result: std::process::Output) -> CheckOutput {
        match self {
            OutputFormat::Json => {
                let stdout = std_to_string(result.stdout);
                let stderr = std_to_string(result.stderr);
                let o: Result<CheckOutput, serde_json::Error> = serde_json::from_str(&stdout);
                match o {
                    Ok(mut check_output) => {
                        if check_output.stderr.is_none() && !stderr.is_empty() {
                            check_output.stderr = Some(stderr.trim().to_owned());
                        }
                        check_output
                    }
                    Err(_err) => CheckOutput {
                        stdout: Some("Check did not return valid JSON".into()),
                        ..Default::default()
                    },
                }
            }
            OutputFormat::Stdio => {
                let stderr = if result.stderr.is_empty() {
                    None
                } else {
                    Some(std_to_string(result.stderr).trim().to_owned())
                };
                let remedy = if result.stdout.is_empty() {
                    None
                } else {
                    Some(std_to_string(result.stdout).trim().to_owned())
                };

                CheckOutput {
                    stderr,
                    remedy,
                    ..Default::default()
                }
            }
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Stdio => write!(f, "stdio"),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CheckOutput {
    #[serde(rename(deserialize = "output"))]
    stdout: Option<String>,
    #[serde(rename(deserialize = "error"))]
    stderr: Option<String>,
    remedy: Option<String>,
    #[serde(default, skip_serializing)]
    verbose: bool,
}

impl CheckOutput {
    fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }
}

impl fmt::Display for CheckOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stdout = self.stdout.clone();
        let stderr = self.stderr.clone();
        let remedy = self.remedy.clone();

        if let Some(stdout) = stdout {
            writeln!(f, "\x1b[0;31m== Check output ==\x1b[0m\r\n")?;
            write!(f, "{stdout}\r\n\r\n")?;
        }

        if let Some(stderr) = stderr {
            writeln!(f, "\x1b[0;31m== Check error ==\x1b[0m\r\n")?;
            write!(f, "{stderr}\r\n\r\n")?;
        }

        if let Some(remedy) = remedy {
            write!(f, "\x1b[36mPossible remedy: \x1b[0;33m{remedy}\x1b[0m")?;
            write!(f, "  \x1b[32;1m(it's in the clipboard)\x1b[0m\r\n")?;
        } else {
            writeln!(f, "\x1b[0;33mNo remedy suggested.\x1b[0m")?;
        }

        write!(f, "")
    }
}

#[derive(Debug, Deserialize)]
pub struct Check {
    pub args: Option<HashMap<String, String>>,
    pub check: String,
    pub command: Option<String>,
    #[serde(default)]
    pub output: OutputFormat,
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
                        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                        let mut output = self.output.parse(result);
                        output.verbose(verbose);
                        eprint!("{output}");

                        if output.remedy.is_some() {
                            let mut clipboard = Clipboard::new().unwrap();
                            clipboard.set_text(output.remedy.unwrap()).unwrap();
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
        command.env("MEDIC_OUTPUT_FORMAT", self.output.to_string());

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
            for (i, (key, value)) in args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{key}: {value}")?;
            }
            write!(f, ")")?;
        }
        write!(f, "\x1b[0m")
    }
}
