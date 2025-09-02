// @related [tests](medic-src/src/step/step_config_test.rs)

use crate::cli::Flags;
use crate::context::Context;
use crate::error::MedicError;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::theme::current_theme;
use crate::util::StringOrList;
use crate::{extra, std_to_string};

use console::{style, Style};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use which::which;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct StepConfig {
  pub args: Option<BTreeMap<String, StringOrList>>,
  pub cd: Option<String>,
  pub command: Option<String>,
  #[serde(default)]
  pub env: BTreeMap<String, String>,
  pub name: Option<String>,
  pub platform: Option<Vec<String>>,
  pub step: String,
  #[serde(default)]
  pub verbose: bool,
}

impl Runnable for StepConfig {
  fn platform(&self) -> &Option<Vec<String>> {
    &self.platform
  }

  fn run(&self, progress: &mut retrogress::ProgressBar, _flags: &mut Flags, _ctx: &Context) -> Recoverable<()> {
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
              Recoverable::Ok(())
            } else {
              progress.failed(pb);
              eprintln!(
                "{}{}",
                (8u8 as char),
                OptionalStyled::new("FAILED", current_theme().error_style.clone())
              );
              let err = std_to_string(result.stderr);
              if !verbose && err.trim() != "" {
                eprintln!(
                  "{}",
                  OptionalStyled::new("== Step output ==", current_theme().error_style.clone())
                );
                eprintln!();
                eprint!("{err}");
              }
              if allow_failure {
                Recoverable::Optional((), None)
              } else {
                Recoverable::Err(None, None)
              }
            }
          }
          Err(err) => {
            progress.failed(pb);
            Recoverable::Err(Some(err.into()), None)
          }
        }
      }
      Err(err) => {
        progress.failed(pb);
        Recoverable::Err(Some(format!("Failed to parse command: {err}").into()), None)
      }
    }
  }

  fn to_command(&self) -> Result<Command, MedicError> {
    let step_cmd = format!("medic-step-{}", self.step);
    if let Err(_err) = which(&step_cmd) {
      return Err(MedicError::Message(format!("executable {step_cmd} not found in PATH")));
    };
    let mut command = extra::command::new(&step_cmd, &self.cd);

    if let Some(subcmd) = &self.command {
      command.arg(subcmd);
    }
    if let Some(args) = &self.args {
      for (flag, values) in args {
        for value in values {
          let flag_arg = format!("--{}", flag);
          command.arg(flag_arg).arg(value);
        }
      }
    }

    for (var, value) in &self.env {
      command.env(var, value);
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

      let mut cd_str = OptionalStyled::with_style(Style::new().force_styling(true).green()).prefixed(" ");

      if let Some(dir) = &self.cd {
        cd_str.push('(');
        cd_str.push_str(dir);
        cd_str.push(')');
      }

      let mut args_str = OptionalStyled::with_style(Style::new().force_styling(true).yellow()).prefixed(" ");
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

      write!(f, "{}{}{}", style(cmd_str).force_styling(true).cyan(), args_str, cd_str,)
    }
  }
}
