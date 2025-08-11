// @related [test](medic-src/src/check/check_test.rs)

#[cfg(test)]
mod check_test;

mod check_output;
mod output_format;

pub use self::output_format::OutputFormat;
use crate::error::MedicError;
use crate::extra;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::{Recoverable, Remedy};
use crate::runnable::Runnable;
use crate::theme::current_theme;
use crate::util::StringOrList;

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use which::which;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Check {
  pub args: Option<BTreeMap<String, StringOrList>>,
  pub cd: Option<String>,
  pub check: String,
  pub command: Option<String>,
  #[serde(default)]
  pub env: BTreeMap<String, String>,
  #[serde(default)]
  pub output: OutputFormat,
  pub platform: Option<Vec<String>>,
  #[serde(default)]
  pub verbose: bool,
}

impl Runnable for Check {
  fn platform(&self) -> &Option<Vec<String>> {
    &self.platform
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
    let verbose = self.verbose();
    let pb = progress.append(&self.to_string());

    match self.to_command() {
      Ok(mut command) => {
        let output = if verbose {
          command.stderr(Stdio::piped());
          let mut child = command.spawn()?;
          let stderr = child
            .stderr
            .take()
            .ok_or("Error capturing stderr of check.")?;
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
              Recoverable::Ok(())
            } else {
              progress.failed(pb);
              let mut output = self.output.parse(result, self.cd.clone());
              output.verbose(verbose);
              eprint!("{output}");

              let mut remedy: Option<Remedy> = None;

              if let Some(remedy_str) = output.remedy {
                remedy = Some(Remedy::new(remedy_str.clone(), self.cd.clone()));
              }
              Recoverable::Err(None, remedy)
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
    let mut check_cmd: String = "medic-check-".to_owned();
    check_cmd.push_str(&self.check);

    if let Err(_err) = which(&check_cmd) {
      return Err(MedicError::Message(format!("executable {check_cmd} not found in PATH")));
    };

    let mut command = extra::command::new(&check_cmd, &self.cd);
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

    for (var, value) in &self.env {
      command.env(var, value);
    }

    Ok(command)
  }

  fn verbose(&self) -> bool {
    self.verbose
  }
}

impl fmt::Display for Check {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut cmd_str = OptionalStyled::with_style(current_theme().text_style.clone());
    cmd_str.push_str(&self.check);

    if let Some(command) = &self.command {
      cmd_str.push_str(": ");
      cmd_str.push_str(command);
      cmd_str.push('?');
    }

    let mut cd_str = OptionalStyled::with_style(current_theme().cd_style.clone()).prefixed(" ");

    if let Some(dir) = &self.cd {
      cd_str.push('(');
      cd_str.push_str(dir);
      cd_str.push(')');
    }

    let mut args_str = OptionalStyled::with_style(current_theme().args_style.clone()).prefixed(" ");
    if let Some(args) = &self.args {
      args_str.push('(');

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
    }

    write!(f, "{cmd_str}{args_str}{cd_str}")
  }
}
