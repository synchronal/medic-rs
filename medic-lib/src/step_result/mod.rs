use std::io::{self, Write};
use std::ops::{ControlFlow, FromResidual, Try};

pub enum StepResult {
  StepOk,
  StepError(String, Option<String>, Option<String>),
}

impl std::process::Termination for StepResult {
  fn report(self) -> std::process::ExitCode {
    match self {
      StepResult::StepOk => std::process::ExitCode::from(0),
      StepResult::StepError(msg, stdout, stderr) => {
        eprintln!("\x1b[31;1mError:\x1b[0m {msg}\r\n");
        if let Some(stdout) = stdout
          && !stdout.is_empty()
        {
          eprintln!("\x1b[31;1mstdout:\x1b[0m\r\n{stdout}");
        }
        if let Some(stderr) = stderr
          && !stderr.is_empty()
        {
          eprintln!("\x1b[31;1mstderr:\x1b[0m\r\n{stderr}");
        }
        io::stderr().flush().unwrap();

        std::process::ExitCode::from(1)
      }
    }
  }
}

pub struct ResultCodeResidual(String, Option<String>, Option<String>);

impl Try for StepResult {
  type Output = ();
  type Residual = ResultCodeResidual;

  fn branch(self) -> ControlFlow<Self::Residual> {
    match self {
      StepResult::StepError(msg, stdout, stderr) => ControlFlow::Break(ResultCodeResidual(msg, stdout, stderr)),
      StepResult::StepOk => ControlFlow::Continue(()),
    }
  }
  fn from_output((): ()) -> Self {
    StepResult::StepOk
  }
}

impl FromResidual for StepResult {
  fn from_residual(r: ResultCodeResidual) -> Self {
    Self::StepError(r.0, r.1, r.2)
  }
}
