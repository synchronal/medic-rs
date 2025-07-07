use crate::optional_styled::OptionalStyled;
use crate::theme::current_theme;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CheckOutput {
  #[serde(rename(deserialize = "output"))]
  pub(crate) stdout: Option<String>,
  #[serde(rename(deserialize = "error"))]
  pub(crate) stderr: Option<String>,
  pub(crate) remedy: Option<String>,
  #[serde(default, skip_serializing)]
  pub(crate) verbose: bool,
}

impl CheckOutput {
  pub(crate) fn verbose(&mut self, verbose: bool) {
    self.verbose = verbose;
  }
}

impl fmt::Display for CheckOutput {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let stdout = self.stdout.clone();
    let stderr = self.stderr.clone();

    if let Some(stdout) = stdout {
      writeln!(
        f,
        "{}\r\n",
        OptionalStyled::new("== Check output ==", current_theme().error_style.clone())
      )?;
      write!(f, "{stdout}\r\n\r\n")?;
    }

    if let Some(stderr) = stderr {
      writeln!(
        f,
        "{}\r\n",
        OptionalStyled::new("== Check error ==", current_theme().error_style.clone())
      )?;
      write!(f, "{stderr}\r\n\r\n")?;
    }
    write!(f, "")
  }
}
