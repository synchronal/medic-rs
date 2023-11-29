// @related [tests](medic-src/src/outdated_check/outdated_check_test.rs)

#[cfg(test)]
mod outdated_check_test;

use crate::optional_styled::OptionalStyled;
use crate::runnable::Runnable;
use crate::std_to_string;
use crate::util::StringOrList;
use crate::AppResult;

use console::{style, Style};
use retrogress::Progress;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use which::which;

#[derive(Debug, Deserialize, PartialEq)]
pub struct OutdatedCheck {
    pub args: Option<BTreeMap<String, StringOrList>>,
    pub cd: Option<String>,
    pub check: String,
    pub name: Option<String>,
    pub remedy: Option<String>,
}

impl Runnable for OutdatedCheck {
    fn run(self, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
        let verbose = self.verbose();
        let pb = progress.append(&self.to_string());

        io::stdout().flush().unwrap();
        match self.to_command() {
            Ok(mut command) => {
                command
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());

                let mut child = command.spawn()?;
                let stderr = child.stderr.take().unwrap();
                let stdout = child.stdout.take().unwrap();

                let mut out_progress = progress.clone();
                let mut err_progress = progress.clone();

                let out_thr = thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    reader
                        .lines()
                        .map_while(Result::ok)
                        .for_each(|line| out_progress.println(pb, &line));
                });
                let err_thr = thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    reader
                        .lines()
                        .map_while(Result::ok)
                        .for_each(|line| err_progress.println(pb, &line));
                });

                let output = child.wait_with_output();
                out_thr.join().unwrap();
                err_thr.join().unwrap();

                match output {
                    Ok(result) => {
                        if result.status.success() {
                            progress.succeeded(pb);
                            AppResult::Ok(())
                        } else {
                            progress.failed(pb);
                            println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                            if !verbose {
                                eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                                eprint!("{}", std_to_string(result.stderr));
                            }
                            AppResult::Ok(())
                        }
                    }
                    Err(err) => {
                        progress.failed(pb);
                        AppResult::Err(Some(err.into()))
                    }
                }
            }
            Err(err) => AppResult::Err(Some(format!("Failed to parse command: {err}").into())),
        }
    }
    fn to_command(&self) -> Result<Command, Box<dyn std::error::Error>> {
        let mut check_cmd: String = "medic-outdated-".to_owned();
        check_cmd.push_str(&self.check);
        if let Err(_err) = which(&check_cmd) {
            let msg: Box<dyn std::error::Error> =
                format!("executable {check_cmd} not found in PATH").into();
            return Err(msg);
        };
        let mut command = Command::new(check_cmd);

        if let Some(directory) = &self.cd {
            if let Ok(expanded) = std::fs::canonicalize(directory) {
                command.current_dir(&expanded);
            } else {
                let msg: Box<dyn std::error::Error> =
                    format!("directory {} does not exist", directory).into();
                return Err(msg);
            }
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

        Ok(command)
    }

    fn verbose(&self) -> bool {
        true
    }
}

impl fmt::Display for OutdatedCheck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", style(name).force_styling(true).cyan())
        } else {
            let cmd_str = self.check.clone();

            let mut cd_str =
                OptionalStyled::with_style(Style::new().force_styling(true).green()).prefixed(" ");

            if let Some(dir) = &self.cd {
                cd_str.push('(');
                cd_str.push_str(dir);
                cd_str.push(')');
            }

            let mut args_str =
                OptionalStyled::with_style(Style::new().force_styling(true).yellow()).prefixed(" ");

            if let Some(args) = &self.args {
                args_str.push('(');

                for (i, (key, values)) in args.iter().enumerate() {
                    if i > 0 {
                        args_str.push_str(", ")
                    }
                    for (j, value) in values.into_iter().enumerate() {
                        if j > 0 {
                            args_str.push_str(", ")
                        }

                        args_str.push_str(&format!("{key}: {value}"));
                    }
                }
                args_str.push(')');
            };

            write!(
                f,
                "{} {}{}{}",
                style("outdated:").force_styling(true).cyan(),
                style(cmd_str).force_styling(true).cyan().bright().bold(),
                cd_str,
                args_str,
            )
        }
    }
}
