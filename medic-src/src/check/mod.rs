// @related [test](medic-src/src/check/check_test.rs)

#[cfg(test)]
mod check_test;

mod check_output;
mod output_format;

use self::output_format::OutputFormat;
use crate::runnable::Runnable;
use crate::util::StringOrList;
use crate::AppResult;

use arboard::Clipboard;
use console::style;
use retrogress::Progress;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Check {
    pub args: Option<BTreeMap<String, StringOrList>>,
    pub check: String,
    pub command: Option<String>,
    #[serde(default)]
    pub output: OutputFormat,
    #[serde(default)]
    pub verbose: bool,
}

impl Runnable for Check {
    fn run(self, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
        let verbose = self.verbose();
        let pb = progress.append(&self.to_string());

        if let Some(mut command) = self.to_command() {
            let output = if verbose {
                command.stderr(Stdio::piped());
                let mut child = command.spawn()?;
                let stderr = child.stderr.take().unwrap();
                let reader = BufReader::new(stderr);

                reader
                    .lines()
                    .map_while(Result::ok)
                    .for_each(|line| progress.println(pb, &line));
                child.wait_with_output()
            } else {
                command.output()
            };

            match output {
                Ok(result) => {
                    if result.status.success() {
                        progress.succeeded(pb);
                        AppResult::Ok(())
                    } else {
                        progress.failed(pb);
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
                    progress.failed(pb);
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
            for (flag, values) in args {
                for value in values {
                    let mut flag_arg = "--".to_owned();
                    flag_arg.push_str(flag);
                    command.arg(flag_arg).arg(value);
                }
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
        let mut cmd_str = "".to_owned();
        cmd_str.push_str(&self.check);

        if let Some(command) = &self.command {
            cmd_str.push_str(": ");
            cmd_str.push_str(command);
            cmd_str.push('?');
        }

        if let Some(args) = &self.args {
            let mut args_str = "(".to_owned();

            for (i, (key, values)) in args.iter().enumerate() {
                if i > 0 {
                    args_str.push_str(", ");
                }
                for (j, value) in values.into_iter().enumerate() {
                    if j > 0 {
                        args_str.push_str(", ");
                    }
                    args_str.push_str(&format!("{key}: {value}"));
                }
            }
            args_str.push(')');
            write!(
                f,
                "{} {}",
                style(cmd_str).force_styling(true).cyan(),
                style(args_str).force_styling(true).yellow()
            )
        } else {
            write!(f, "{}", style(cmd_str).force_styling(true).cyan())
        }
    }
}
