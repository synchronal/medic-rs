use crate::error::MedicError;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::Recoverable;
use crate::theme::current_theme;
use std::ops::{ControlFlow, FromResidual, Try};

pub enum AppResult<T> {
  Ok(T),
  Err(Option<MedicError>),
  Quit,
}

impl From<Result<std::process::Output, std::io::Error>> for AppResult<()> {
  fn from(result: Result<std::process::Output, std::io::Error>) -> Self {
    match result {
      Ok(_) => Self::Ok(()),
      Err(err) => Self::Err(Some(err.into())),
    }
  }
}

impl<T> From<Recoverable<T>> for AppResult<T> {
  fn from(recoverable: Recoverable<T>) -> Self {
    match recoverable {
      Recoverable::Ok(val) => Self::Ok(val),
      Recoverable::Err(e, _) => Self::Err(e),
      Recoverable::Manual(e, _) => Self::Err(e),
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
          eprintln!(
            "{} {}",
            OptionalStyled::new("ERROR:", current_theme().error_style.clone()),
            OptionalStyled::new(error.to_string(), current_theme().error_style.clone()),
          );
        }
        std::process::ExitCode::from(1)
      }
      AppResult::Quit => std::process::ExitCode::from(crate::QUIT_STATUS_CODE as u8),
    }
  }
}

// Retain the original error, as well as whether medic should completely
// quit. When false, recoverables may recover.
pub struct ResultCodeResidual(Option<MedicError>, bool);

impl<T> Try for AppResult<T> {
  type Output = T;
  type Residual = ResultCodeResidual;

  fn branch(self) -> ControlFlow<Self::Residual, T> {
    match self {
      AppResult::Err(err) => ControlFlow::Break(ResultCodeResidual(err, false)),
      AppResult::Quit => ControlFlow::Break(ResultCodeResidual(None, true)),
      AppResult::Ok(res) => ControlFlow::Continue(res),
    }
  }
  fn from_output(t: T) -> Self {
    AppResult::Ok(t)
  }
}

impl<T> FromResidual for AppResult<T> {
  fn from_residual(r: ResultCodeResidual) -> Self {
    if r.1 {
      Self::Quit
    } else {
      Self::Err(r.0)
    }
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, &str>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, &str>) -> Self {
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
    Self::Err(Some(r.unwrap_err().into()))
  }
}

impl<T> FromResidual<Result<std::convert::Infallible, MedicError>> for AppResult<T> {
  fn from_residual(r: Result<std::convert::Infallible, MedicError>) -> Self {
    Self::Err(Some(r.unwrap_err()))
  }
}
