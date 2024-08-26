// @related [test](medic-src/src/shell/shell_config_test.rs)

use crate::optional_styled::OptionalStyled;
use crate::recoverable::{Recoverable, Remedy};
use crate::runnable::Runnable;
use crate::std_to_string;

use arboard::Clipboard;
use console::{style, Style};
use retrogress::Progress;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ShellConfig {
  #[serde(default)]
  pub allow_failure: bool,
  pub cd: Option<String>,
  #[serde(default)]
  pub env: BTreeMap<String, String>,
  #[serde(default)]
  pub inline: bool,
  pub name: String,
  pub remedy: Option<String>,
  pub shell: String,
  #[serde(default)]
  pub verbose: bool,
}

impl ShellConfig {
  pub fn new(name: String, shell: String, cd: Option<String>, remedy: Option<String>, verbose: bool) -> Self {
    Self {
      cd,
      env: BTreeMap::default(),
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

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    let allow_failure = self.allow_failure();
    let verbose = self.verbose();
    let pb = progress.append(&self.to_string());

    match self.to_command() {
      Ok(mut command) => {
        let output = if verbose {
          command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

          let mut child = command.spawn()?;
          let stderr = child
            .stderr
            .take()
            .ok_or("Error capturing stderr of shell command.")?;
          let stdout = child
            .stdout
            .take()
            .ok_or("Error capturing stdout of shell command.")?;

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
              Recoverable::Ok(())
            } else {
              progress.failed(pb);
              if !verbose {
                eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                eprint!("{}", std_to_string(result.stderr));
              }
              if allow_failure {
                eprintln!("\r\n\x1b[32m(continuing)\x1b[0m");
                Recoverable::Ok(())
              } else {
                let mut remedy: Option<Remedy> = None;

                if let Some(mut remedy_str) = self.remedy {
                  remedy = Some(Remedy::new(remedy_str.clone(), self.cd.clone()));

                  if self.cd.is_some() {
                    let dir = self.cd.unwrap();
                    remedy_str = format!("(cd {dir} && {remedy_str})");
                  }

                  eprint!("\x1b[36mPossible remedy: \x1b[0;33m{remedy_str}\x1b[0m");
                  eprintln!("  \x1b[32;1m(it's in the clipboard)\x1b[0m\r\n");

                  let mut clipboard = Clipboard::new()?;
                  clipboard.set_text(remedy_str)?;
                }
                Recoverable::Err(None, remedy)
              }
            }
          }
          Err(err) => {
            progress.failed(pb);
            Recoverable::Err(Some(err.into()), None)
          }
        }
      }
      Err(err) => Recoverable::Err(Some(format!("Failed to parse command: {err}").into()), None),
    }
  }
  fn to_command(&self) -> Result<Command, Box<dyn std::error::Error>> {
    if self.shell.is_empty() {
      Err("No shell command specified".into())
    } else {
      let mut command = Command::new("sh");
      command.arg("-c").arg(&self.shell);

      if let Some(directory) = &self.cd {
        if let Ok(expanded) = std::fs::canonicalize(directory) {
          command.current_dir(&expanded);
        } else {
          let msg: Box<dyn std::error::Error> = format!("directory {} does not exist", directory).into();
          return Err(msg);
        }
      }

      for (var, value) in &self.env {
        command.env(var, value);
      }

      Ok(command)
    }
  }

  fn verbose(&self) -> bool {
    self.verbose && !self.inline
  }
}
impl fmt::Display for ShellConfig {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut cd_str = OptionalStyled::with_style(Style::new().force_styling(true).green()).prefixed(" ");

    if let Some(dir) = &self.cd {
      cd_str.push('(');
      cd_str.push_str(dir);
      cd_str.push(')');
    }

    write!(f, "{}", style(&self.name).force_styling(true).cyan())?;
    write!(
      f,
      " {}{}",
      style(format!("({})", self.shell))
        .force_styling(true)
        .yellow(),
      cd_str
    )
  }
}
