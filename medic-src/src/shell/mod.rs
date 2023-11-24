// @related [test](medic-src/src/shell/shell_test.rs)

#[cfg(test)]
mod shell_test;

use crate::runnable::Runnable;
use crate::std_to_string;
use crate::AppResult;

use arboard::Clipboard;
use console::style;
use retrogress::Progress;
use serde::Deserialize;
use std::fmt;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ShellConfig {
    #[serde(default)]
    pub allow_failure: bool,
    #[serde(default)]
    pub inline: bool,
    pub name: String,
    pub remedy: Option<String>,
    pub shell: String,
    #[serde(default)]
    pub verbose: bool,
}

impl ShellConfig {
    pub fn new(name: String, shell: String, remedy: Option<String>, verbose: bool) -> Self {
        Self {
            name,
            shell,
            remedy,
            verbose,
            allow_failure: false,
            inline: false,
        }
    }
}

impl Runnable for ShellConfig {
    fn allow_failure(&self) -> bool {
        self.allow_failure
    }

    fn run(self, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
        let allow_failure = self.allow_failure();
        let verbose = self.verbose();
        let pb = progress.append(&self.to_string());

        io::stdout().flush().unwrap();
        if let Some(mut command) = self.to_command() {
            let output = if verbose {
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

                let res = child.wait_with_output();
                out_thr.join().unwrap();
                err_thr.join().unwrap();
                res
            } else {
                if self.inline {
                    command
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit());
                    progress.hide(pb);
                }
                command.output()
            };

            if self.inline {
                progress.show(pb);
            }

            match output {
                Ok(result) => {
                    if result.status.success() {
                        progress.succeeded(pb);
                        AppResult::Ok(())
                    } else {
                        progress.failed(pb);
                        if !verbose {
                            eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                            eprint!("{}", std_to_string(result.stderr));
                        }
                        if allow_failure {
                            eprintln!("\r\n\x1b[32m(continuing)\x1b[0m");
                            AppResult::Ok(())
                        } else {
                            if let Some(remedy) = self.remedy {
                                eprint!("\x1b[36mPossible remedy: \x1b[0;33m{remedy}\x1b[0m");
                                eprintln!("  \x1b[32;1m(it's in the clipboard)\x1b[0m\r\n");
                                let mut clipboard = Clipboard::new().unwrap();
                                clipboard.set_text(remedy).unwrap();
                            }
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
    fn to_command(&self) -> Option<Command> {
        if self.shell.is_empty() {
            None
        } else {
            let mut command = Command::new("sh");
            command.arg("-c").arg(&self.shell);
            Some(command)
        }
    }

    fn verbose(&self) -> bool {
        self.verbose && !self.inline
    }
}
impl fmt::Display for ShellConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", style(&self.name).force_styling(true).cyan())?;
        write!(
            f,
            " {}",
            style(format!("({})", self.shell))
                .force_styling(true)
                .yellow()
        )
    }
}
