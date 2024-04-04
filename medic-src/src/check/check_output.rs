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
    let remedy = self.remedy.clone();

    if let Some(stdout) = stdout {
      writeln!(f, "\x1b[0;31m== Check output ==\x1b[0m\r\n")?;
      write!(f, "{stdout}\r\n\r\n")?;
    }

    if let Some(stderr) = stderr {
      writeln!(f, "\x1b[0;31m== Check error ==\x1b[0m\r\n")?;
      write!(f, "{stderr}\r\n\r\n")?;
    }

    if let Some(remedy) = remedy {
      write!(f, "\x1b[36mPossible remedy: \x1b[0;33m{remedy}\x1b[0m")?;
      write!(f, "  \x1b[32;1m(it's in the clipboard)\x1b[0m\r\n")?;
    } else {
      writeln!(f, "\x1b[0;33mNo remedy suggested.\x1b[0m")?;
    }

    write!(f, "")
  }
}
