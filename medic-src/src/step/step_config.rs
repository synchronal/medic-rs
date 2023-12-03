// @related [tests](medic-src/src/step/step_config_test.rs)

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
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use which::which;

#[derive(Debug, Deserialize, PartialEq)]
pub struct StepConfig {
    pub args: Option<BTreeMap<String, StringOrList>>,
    pub command: Option<String>,
    pub name: Option<String>,
    pub step: String,
    #[serde(default)]
    pub verbose: bool,
}

impl Runnable for StepConfig {
    fn run(self, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
        let allow_failure = self.allow_failure();
        let verbose = self.verbose();
        let pb = progress.append(&self.to_string());

        if let Ok(mut command) = self.to_command() {
            let output = if verbose {
                command
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());

                let mut child = command.spawn()?;
                let stderr = child
                    .stderr
                    .take()
                    .ok_or("Error capturing stderr of step.")?;
                let stdout = child
                    .stdout
                    .take()
                    .ok_or("Error capturing stdout of step.")?;

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

                let res = child.wait_with_output();
                out_thr.join().unwrap();
                err_thr.join().unwrap();
                res
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
                        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                        if !verbose {
                            eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                            eprint!("{}", std_to_string(result.stderr));
                        }
                        if allow_failure {
                            eprintln!("\r\n\x1b[32m(continuing)\x1b[0m");
                            AppResult::Ok(())
                        } else {
                            AppResult::Err(None)
                        }
                    }
                }
                Err(err) => {
                    progress.failed(pb);
                    AppResult::Err(Some(err.into()))
                }
            }
        } else {
            AppResult::Err(Some("Failed to parse command".into()))
        }
    }
    fn to_command(&self) -> Result<Command, Box<dyn std::error::Error>> {
        let mut step_cmd: String = "medic-step-".to_owned();
        step_cmd.push_str(&self.step);
        if let Err(_err) = which(&step_cmd) {
            let msg: Box<dyn std::error::Error> =
                format!("executable {step_cmd} not found in PATH").into();
            return Err(msg);
        };
        let mut command = Command::new(step_cmd);

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

        Ok(command)
    }

    fn verbose(&self) -> bool {
        self.verbose
    }
}

impl fmt::Display for StepConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", style(name).force_styling(true).cyan())
        } else {
            let mut cmd_str = self.step.clone();

            if let Some(command) = &self.command {
                cmd_str.push_str(": ");
                cmd_str.push_str(command);
                cmd_str.push('!');
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
            }

            write!(
                f,
                "{}{}",
                style(cmd_str).force_styling(true).cyan(),
                args_str
            )
        }
    }
}
