use std::ops::{ControlFlow, FromResidual, Try};

use crate::recoverable::Recoverable;

pub enum AppResult<T> {
  Ok(T),
  Err(Option<Box<dyn std::error::Error>>),
}

impl<T> From<Recoverable<T>> for AppResult<T> {
  fn from(recoverable: Recoverable<T>) -> Self {
    match recoverable {
      Recoverable::Ok(val) => Self::Ok(val),
      Recoverable::Err(e, _) => Self::Err(e),
      Recoverable::Optional(val, _) => Self::Ok(val),
    }
  }
}

impl<T> std::process::Termination for AppResult<T> {
  fn report(self) -> std::process::ExitCode {
    match self {
      AppResult::Ok(_) => std::process::ExitCode::from(0),
      AppResult::Err(err) => {
        if let Some(error) = err {
          eprintln!("\x1b[31;1mERROR: {}\x1b[0m", error);
        }
        std::process::ExitCode::from(1)
      }
    }
  }
}

pub struct ResultCodeResidual(Option<Box<dyn std::error::Error>>);

impl<T> Try for AppResult<T> {
  type Output = T;
  type Residual = ResultCodeResidual;

  fn branch(self) -> ControlFlow<Self::Residual, T> {
    match self {
      AppResult::Err(err) => ControlFlow::Break(ResultCodeResidual(err)),
      AppResult::Ok(res) => ControlFlow::Continue(res),
    }
  }
  fn from_output(t: T) -> Self {
    AppResult::Ok(t)
  }
}

impl<T> FromResidual for AppResult<T> {
  fn from_residual(r: ResultCodeResidual) -> Self {
    Self::Err(r.0)
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, &str>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, &str>) -> Self {
    Self::Err(Some(r.unwrap_err().into()))
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, arboard::Error>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, arboard::Error>) -> Self {
    Self::Err(Some(r.unwrap_err().into()))
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, envsubst::Error>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, envsubst::Error>) -> Self {
    Self::Err(Some(r.unwrap_err().into()))
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, std::io::Error>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, std::io::Error>) -> Self {
    Self::Err(Some(Box::new(r.unwrap_err())))
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, std::ffi::OsString>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, std::ffi::OsString>) -> Self {
    let err = r.unwrap_err();
    Self::Err(Some(err.into_string()?.into()))
  }
}
