#![feature(try_trait_v2)]

use std::ops::{ControlFlow, FromResidual, Try};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum CheckResult {
    #[default]
    CheckOk,
    CheckError(String, String, String, String),
}

impl std::process::Termination for CheckResult {
    fn report(self) -> std::process::ExitCode {
        match self {
            CheckResult::CheckOk => std::process::ExitCode::from(0),
            CheckResult::CheckError(msg, stdout, stderr, remedy) => {
                eprintln!("{msg}\r\n");
                eprintln!("stdout:\r\n{stdout}");
                eprintln!("stderr:\r\n{stderr}");
                eprintln!("Possible remedy:\r\n{remedy}\r\n");

                std::process::ExitCode::from(1)
            }
        }
    }
}

pub struct ResultCodeResidual(String, String, String, String);

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

pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}

#[cfg(test)]
mod tests {
    // use super::*;
}
