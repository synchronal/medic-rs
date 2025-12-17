use crate::error::MedicError;
use crate::extra;
use crate::optional_styled::OptionalStyled;
use crate::theme::current_theme;
use std::ops::{ControlFlow, FromResidual, Try};
use std::process::Command;

pub struct Remedy {
  pub command: String,
  pub cd: Option<String>,
}

impl Remedy {
  pub fn new(command: String, cd: Option<String>) -> Self {
    Self { command, cd }
  }
  pub fn to_command(&self) -> Command {
    extra::command::from_string(&self.command, &self.cd)
  }
}

impl std::fmt::Display for Remedy {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", extra::command::to_string(&self.command, &self.cd))
  }
}

// // //

pub enum Recoverable<T> {
  Err(Option<MedicError>, Option<Remedy>),
  Manual(Option<MedicError>, Option<Remedy>),
  Nonrecoverable(MedicError),
  Ok(T),
  Optional(T, Option<Remedy>),
  Quit,
}

impl<T> std::process::Termination for Recoverable<T> {
  fn report(self) -> std::process::ExitCode {
    match self {
      Recoverable::Err(err, _remedy) => {
        if let Some(error) = err {
          eprintln!(
            "{} {}",
            OptionalStyled::new("ERROR:", current_theme().error_style.clone()),
            OptionalStyled::new(error.to_string(), current_theme().error_style.clone())
          );
        }
        std::process::ExitCode::from(1)
      }
      Recoverable::Manual(_, _) => std::process::ExitCode::from(1),
      Recoverable::Nonrecoverable(err) => {
        eprintln!(
          "{} {}",
          OptionalStyled::new("ERROR:", current_theme().error_style.clone()),
          OptionalStyled::new(err.to_string(), current_theme().error_style.clone())
        );
        std::process::ExitCode::from(crate::QUIT_STATUS_CODE as u8)
      }
      Recoverable::Ok(_) => std::process::ExitCode::from(0),
      Recoverable::Optional(_, _) => std::process::ExitCode::from(0),
      Recoverable::Quit => std::process::ExitCode::from(crate::QUIT_STATUS_CODE as u8),
    }
  }
}

pub struct ResultCodeResidual(Option<MedicError>);

impl<T> Try for Recoverable<T> {
  type Output = T;
  type Residual = ResultCodeResidual;

  fn branch(self) -> ControlFlow<Self::Residual, T> {
    match self {
      Recoverable::Err(err, _remedy) => ControlFlow::Break(ResultCodeResidual(err)),
      Recoverable::Manual(res, _remedy) => ControlFlow::Break(ResultCodeResidual(res)),
      Recoverable::Nonrecoverable(err) => ControlFlow::Break(ResultCodeResidual(Some(err))),
      Recoverable::Ok(res) => ControlFlow::Continue(res),
      Recoverable::Optional(res, _remedy) => ControlFlow::Continue(res),
      Recoverable::Quit => ControlFlow::Break(ResultCodeResidual(None)),
    }
  }
  fn from_output(t: T) -> Self {
    Recoverable::Ok(t)
  }
}

impl<T> FromResidual for Recoverable<T> {
  fn from_residual(r: ResultCodeResidual) -> Self {
    Self::Err(r.0, None)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, &str>> for Recoverable<T> {
  fn from_residual(r: Result<std::convert::Infallible, &str>) -> Self {
    Self::Err(Some(r.unwrap_err().into()), None)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, arboard::Error>> for Recoverable<T> {
  fn from_residual(r: Result<std::convert::Infallible, arboard::Error>) -> Self {
    Self::Err(Some(r.unwrap_err().into()), None)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, envsubst::Error>> for Recoverable<T> {
  fn from_residual(r: Result<std::convert::Infallible, envsubst::Error>) -> Self {
    Self::Err(Some(r.unwrap_err().into()), None)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, std::io::Error>> for Recoverable<T> {
  fn from_residual(r: Result<std::convert::Infallible, std::io::Error>) -> Self {
    Self::Err(Some(r.unwrap_err().into()), None)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, std::ffi::OsString>> for Recoverable<T> {
  fn from_residual(r: Result<std::convert::Infallible, std::ffi::OsString>) -> Self {
    Self::Err(Some(r.unwrap_err().into()), None)
  }
}
