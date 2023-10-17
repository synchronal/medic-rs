use std::ops::{ControlFlow, FromResidual, Try};

pub enum AppResult<T> {
    Ok(T),
    Err(Option<Box<dyn std::error::Error>>),
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

impl<T> FromResidual<Result<std::convert::Infallible, std::io::Error>> for AppResult<T> {
    fn from_residual(r: Result<std::convert::Infallible, std::io::Error>) -> Self {
        Self::Err(Some(Box::new(r.unwrap_err())))
    }
}
