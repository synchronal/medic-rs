use crate::runnable::Runnable;
use crate::std_to_string;
use crate::AppResult;

use arboard::Clipboard;
use serde::Deserialize;
use std::fmt;
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub allow_failure: bool,
    pub name: String,
    pub remedy: Option<String>,
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
                command
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());
            }
            match command.output() {
                Ok(result) => {
                    if result.status.success() {
                        println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                        AppResult::Ok(())
                    } else {
                        println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
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
                    println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
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
        self.verbose
    }
}
impl fmt::Display for ShellConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[36m{:}", self.name)?;
        write!(f, " \x1b[0;33m({})\x1b[0m", self.shell)
    }
}
