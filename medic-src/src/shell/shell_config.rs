// @related [test](medic-src/src/shell/shell_config_test.rs)

use crate::cli::Flags;
use crate::error::MedicError;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::{Recoverable, Remedy};
use crate::runnable::Runnable;
use crate::std_to_string;
use crate::theme::current_theme;

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
  #[serde(default)]
  pub manual: bool,
  pub name: String,
  pub platform: Option<Vec<String>>,
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
      manual: false,
      name,
      platform: None,
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

  fn platform(&self) -> &Option<Vec<String>> {
    &self.platform
  }

  fn run(self, progress: &mut retrogress::ProgressBar, _flags: &Flags) -> Recoverable<()> {
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
              let err = std_to_string(result.stderr);
              if !verbose && err.trim() != "" {
                eprintln!(
                  "{}",
                  OptionalStyled::new("== Step output ==", current_theme().error_style.clone()),
                );
                eprintln!();
                eprint!("{err}");
              }
              let mut remedy: Option<Remedy> = None;

              if let Some(remedy_str) = self.remedy {
                remedy = Some(Remedy::new(remedy_str.clone(), self.cd.clone()));
              }

              match (self.manual, allow_failure) {
                (true, _) => Recoverable::Manual(Some(err.into()), remedy),
                (false, true) => Recoverable::Optional((), remedy),
                (false, false) => Recoverable::Err(None, remedy),
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
  fn to_command(&self) -> Result<Command, MedicError> {
    if self.shell.is_empty() {
      Err(MedicError::Message("No shell command specified".to_string()))
    } else {
      let mut command = Command::new("sh");
      command.arg("-c").arg(&self.shell);

      if let Some(directory) = &self.cd {
        if let Ok(expanded) = std::fs::canonicalize(directory) {
          command.current_dir(&expanded);
        } else {
          return Err(MedicError::Message(format!("directory {directory} does not exist")));
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
    let mut cd_str = OptionalStyled::with_style(current_theme().cd_style.clone()).prefixed(" ");

    if let Some(dir) = &self.cd {
      cd_str.push('(');
      cd_str.push_str(dir);
      cd_str.push(')');
    }

    write!(
      f,
      "{}",
      OptionalStyled::new(&self.name, current_theme().text_style.clone())
    )?;
    write!(
      f,
      " {}{}",
      OptionalStyled::new(format!("({})", self.shell), current_theme().args_style.clone()),
      cd_str
    )
  }
}
