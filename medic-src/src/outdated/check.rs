// @related [tests](medic-src/src/outdated/check_test.rs)

use super::summary::OutdatedSummary;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::runnable::Runnable;
use crate::std_to_string;
use crate::util::StringOrList;
use console::{style, Style};
use retrogress::Progress;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use which::which;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct OutdatedCheck {
  pub args: Option<BTreeMap<String, StringOrList>>,
  pub cd: Option<String>,
  pub check: String,
  pub name: Option<String>,
  pub remedy: Option<String>,
}

impl Runnable for OutdatedCheck {
  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()> {
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
            let stdout = &std_to_string(result.stdout);
            let summary_result = OutdatedSummary::from_str(stdout);
            if !result.status.success() || summary_result.is_err() {
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
            progress.println(pb, &format!("{}", summary));
            progress.println(pb, "");

            let mut remedy_str: Option<String> = None;

            match (summary.remedy, self.cd) {
              (Some(remedy), Some(dir)) => {
                remedy_str = Some(format!("(cd {} && {})", dir, remedy));
              }
              (Some(remedy), None) => {
                remedy_str = Some(format!("({})", remedy));
              }
              (_, _) => {}
            }

            if remedy_str.is_some() {
              progress.println(
                pb,
                &format!(
                  "    {} {}",
                  style("Remedy:").bold().underlined(),
                  style(remedy_str.as_ref().unwrap()).yellow(),
                ),
              );

              progress.println(pb, "");
            }

            progress.failed(pb);
            Recoverable::Optional((), remedy_str)
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
    let mut check_cmd: String = "medic-outdated-".to_owned();
    check_cmd.push_str(&self.check);
    if let Err(_err) = which(&check_cmd) {
      let msg: Box<dyn std::error::Error> = format!("executable {check_cmd} not found in PATH").into();
      return Err(msg);
    };
    let mut command = Command::new(check_cmd);

    if let Some(directory) = &self.cd {
      if let Ok(expanded) = std::fs::canonicalize(directory) {
        command.current_dir(&expanded);
      } else {
        let msg: Box<dyn std::error::Error> = format!("directory {} does not exist", directory).into();
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
