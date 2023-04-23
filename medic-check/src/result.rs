use std::io::{self, Write};
use std::ops::{ControlFlow, FromResidual, Try};

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
