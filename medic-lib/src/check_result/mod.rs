use serde::Serialize;
use std::io::{self, Write};
use std::ops::{ControlFlow, FromResidual, Try};

#[derive(Serialize)]
struct CheckJson {
  output: String,
  error: Option<String>,
  remedy: Option<String>,
}

enum CheckResultFormat {
  Json,
  Stdio,
}

impl CheckResultFormat {
  fn from_env() -> Self {
    let format = std::env::var("MEDIC_OUTPUT_FORMAT").unwrap_or("json".to_owned());
    match format.as_str() {
      "stdio" => Self::Stdio,
      _ => Self::Json,
    }
  }

  fn print(self, msg: String, stdout: Option<String>, stderr: Option<String>, remedy: Option<String>) {
    match self {
      CheckResultFormat::Stdio => {
        eprintln!("\x1b[31;1mError:\x1b[0m {msg}\r\n");
        if let Some(stdout) = stdout {
          if !stdout.is_empty() {
            eprintln!("\x1b[31;1mstdout:\x1b[0m\r\n{stdout}");
          }
        }
        if let Some(stderr) = stderr {
          if !stderr.is_empty() {
            eprintln!("\x1b[31;1mstderr:\x1b[0m\r\n{stderr}");
          }
        }
        io::stderr().flush().unwrap();
        if let Some(remedy) = remedy {
          println!("{remedy}");
        }
      }
      CheckResultFormat::Json => {
        let mut output = format!("\x1b[31;1mError: \x1b[0m {msg}");
        let mut error = None;

        if let Some(stdout) = stdout {
          if !stdout.is_empty() {
            output.push_str(&format!("\r\n\r\n\x1b[31;1mstdout:\x1b[0m\r\n{stdout}"));
          }
        }
        if let Some(stderr) = stderr {
          if !stderr.is_empty() {
            error = Some(format!("\x1b[31;1mstdout:\x1b[0m\r\n{stderr}"));
          }
        }

        let json = CheckJson { output, error, remedy };

        println!("{}", serde_json::to_string(&json).unwrap());
      }
    }
  }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum CheckResult {
  #[default]
  CheckOk,
  CheckError(String, Option<String>, Option<String>, Option<String>),
}

impl std::process::Termination for CheckResult {
  fn report(self) -> std::process::ExitCode {
    match self {
      CheckResult::CheckOk => std::process::ExitCode::from(0),
      CheckResult::CheckError(msg, stdout, stderr, remedy) => {
        CheckResultFormat::from_env().print(msg, stdout, stderr, remedy);
        std::process::ExitCode::from(1)
      }
    }
  }
}

pub struct ResultCodeResidual(String, Option<String>, Option<String>, Option<String>);

impl Try for CheckResult {
  type Output = ();
  type Residual = ResultCodeResidual;

  fn branch(self) -> ControlFlow<Self::Residual> {
    match self {
      CheckResult::CheckError(msg, stdout, stderr, remedy) => {
        ControlFlow::Break(ResultCodeResidual(msg, stdout, stderr, remedy))
      }
      CheckResult::CheckOk => ControlFlow::Continue(()),
    }
  }
  fn from_output((): ()) -> Self {
    CheckResult::CheckOk
  }
}

impl FromResidual for CheckResult {
  fn from_residual(r: ResultCodeResidual) -> Self {
    Self::CheckError(r.0, r.1, r.2, r.3)
  }
}
