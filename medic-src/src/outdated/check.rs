// @related [tests](medic-src/src/outdated/check_test.rs)

use super::summary::OutdatedSummary;
use crate::cli::Flags;
use crate::context::Context;
use crate::error::MedicError;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::{Recoverable, Remedy};
use crate::runnable::Runnable;
use crate::std_to_string;
use crate::theme::current_theme;
use crate::util::StringOrList;
use console::style;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use which::which;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct OutdatedCheck {
  pub args: Option<BTreeMap<String, StringOrList>>,
  pub cd: Option<String>,
  pub check: String,
  pub name: Option<String>,
  pub platform: Option<Vec<String>>,
  pub remedy: Option<String>,
}

impl Runnable for OutdatedCheck {
  fn platform(&self) -> &Option<Vec<String>> {
    &self.platform
  }

  fn run(&self, progress: &mut retrogress::ProgressBar, _flags: &mut Flags, _ctx: &Context) -> Recoverable<()> {
    let command_name = self.to_string();
    let pb = progress.append(&command_name);

    match self.to_command() {
      Ok(mut command) => {
        command.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = command.spawn()?;
        let stderr = child
          .stderr
          .take()
          .ok_or("Error capturing stderr of outdated check.")?;
        let mut err_progress = progress.clone();
        let command_name_err = command_name.clone();

        let err_thr = thread::spawn(move || {
          let reader = BufReader::new(stderr);
          reader.lines().map_while(Result::ok).for_each(|line| {
            let msg = line.split("::").last().unwrap_or("");
            err_progress.set_message(pb, format!("{command_name_err}\t{}", style(msg).dim()));
          })
        });

        let output = child.wait_with_output();
        err_thr.join().unwrap();

        progress.set_message(pb, command_name);

        match output {
          Ok(result) => {
            if !result.status.success() {
              progress.failed(pb);
              let stderr = &std_to_string(result.stderr);
              return Recoverable::Err(
                Some(format!("Unable to parse outdated output:\r\n{stderr}").into()),
                None,
              );
            }
            let stdout = &std_to_string(result.stdout);
            let summary_result = stdout.parse::<OutdatedSummary>();
            if summary_result.is_err() {
              progress.failed(pb);
              return Recoverable::Err(
                Some(format!("Unable to parse outdated output:\r\n{}", summary_result.err().unwrap()).into()),
                None,
              );
            }

            let summary = summary_result.unwrap();

            if summary.deps.is_empty() {
              progress.succeeded(pb);
              return Recoverable::Ok(());
            }

            progress.println(pb, "");
            progress.println(pb, &format!("{summary}"));
            progress.println(pb, "");

            let mut remedy: Option<Remedy> = None;
            let mut remedy_opt: Option<String> = None;

            if let Some(remedy_cmd) = summary.remedy {
              remedy = Some(Remedy::new(remedy_cmd.clone(), self.cd.clone(), BTreeMap::new()));
              remedy_opt = Some(crate::extra::command::to_string(&remedy_cmd, &self.cd));
            }

            if let Some(remedy_str) = &remedy_opt {
              progress.println(
                pb,
                &format!(
                  "    {} {}",
                  style("Remedy:").bold().underlined(),
                  style(remedy_str).yellow(),
                ),
              );

              progress.println(pb, "");
            }

            progress.failed(pb);
            Recoverable::Optional((), remedy)
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
    let check_cmd = format!("medic-outdated-{}", self.check);
    if let Err(_err) = which(&check_cmd) {
      return Err(MedicError::Message(format!("executable {check_cmd} not found in PATH")));
    };
    let mut command = Command::new(check_cmd);

    if let Some(directory) = &self.cd {
      if let Ok(expanded) = std::fs::canonicalize(directory) {
        command.current_dir(&expanded);
      } else {
        return Err(MedicError::Message(format!("directory {directory} does not exist")));
      }
    }

    if let Some(args) = &self.args {
      for (flag, values) in args {
        for value in values {
          let flag_arg = format!("--{}", flag);
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
      write!(f, "{}", OptionalStyled::new(name, current_theme().text_style.clone()))
    } else {
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
        OptionalStyled::new("outdated:", current_theme().text_style.clone()),
        OptionalStyled::new(&self.check, current_theme().highlight_style.clone()),
        cd_str,
        args_str,
      )
    }
  }
}
