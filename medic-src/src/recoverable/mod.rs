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
  Ok(T),
  Optional(T, Option<Remedy>),
  Err(Option<Box<dyn std::error::Error>>, Option<Remedy>),
}

impl<T> std::process::Termination for Recoverable<T> {
  fn report(self) -> std::process::ExitCode {
    match self {
      Recoverable::Ok(_) => std::process::ExitCode::from(0),
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
      Recoverable::Optional(_, _) => std::process::ExitCode::from(0),
    }
  }
}

pub struct ResultCodeResidual(Option<Box<dyn std::error::Error>>);

impl<T> Try for Recoverable<T> {
  type Output = T;
  type Residual = ResultCodeResidual;

  fn branch(self) -> ControlFlow<Self::Residual, T> {
    match self {
      Recoverable::Err(err, _remedy) => ControlFlow::Break(ResultCodeResidual(err)),
      Recoverable::Ok(res) => ControlFlow::Continue(res),
      Recoverable::Optional(res, _remedy) => ControlFlow::Continue(res),
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
    Self::Err(Some(Box::new(r.unwrap_err())), None)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, std::ffi::OsString>> for Recoverable<T> {
  fn from_residual(r: Result<std::convert::Infallible, std::ffi::OsString>) -> Self {
    let err = r.unwrap_err();
    Self::Err(Some(err.into_string()?.into()), None)
  }
}
