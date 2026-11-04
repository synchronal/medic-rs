// @related [subject](medic-src/src/app_result/mod.rs)

use super::*;
use crate::recoverable::Remedy;
use std::collections::BTreeMap;
use std::process::{Command, ExitCode, Termination};

pub fn initialize() {
  crate::theme::THEME.get_or_init(crate::theme::dark_theme);
}

fn propagate<T, U>(inner: AppResult<T>, after: U) -> AppResult<U> {
  inner?;
  AppResult::Ok(after)
}

fn error_message<T>(result: AppResult<T>) -> Option<String> {
  match result {
    AppResult::Err(Some(err)) => Some(err.to_string()),
    _ => None,
  }
}

fn remedy() -> Option<Remedy> {
  Some(Remedy::new("fix".into(), None, BTreeMap::new()))
}

// // // From<Result<Output, io::Error>>

#[test]
fn from_output() {
  let output = Command::new("false").output();

  assert!(matches!(AppResult::from(output), AppResult::Ok(())));
}

#[test]
fn from_io_error() {
  let output = Command::new("medic-this-binary-does-not-exist").output();

  assert!(matches!(
    AppResult::from(output),
    AppResult::Err(Some(MedicError::Io(_)))
  ));
}

// // // From<Recoverable>

#[test]
fn from_ok() {
  assert!(matches!(AppResult::from(Recoverable::Ok(1)), AppResult::Ok(1)));
  assert!(matches!(
    AppResult::from(Recoverable::Optional(2, remedy())),
    AppResult::Ok(2)
  ));
}

#[test]
fn from_err() {
  assert_eq!(
    error_message(AppResult::from(Recoverable::<()>::Err(Some("bad".into()), remedy()))),
    Some("bad".into())
  );
  assert_eq!(
    error_message(AppResult::from(Recoverable::<()>::Manual(
      Some("manual".into()),
      remedy()
    ))),
    Some("manual".into())
  );
  assert!(matches!(
    AppResult::from(Recoverable::<()>::Err(None, None)),
    AppResult::Err(None)
  ));
}

#[test]
fn from_nonrecoverable() {
  assert_eq!(
    error_message(AppResult::from(Recoverable::<()>::Nonrecoverable("fatal".into()))),
    Some("fatal".into())
  );
}

#[test]
fn from_quit() {
  assert!(matches!(AppResult::from(Recoverable::<()>::Quit), AppResult::Quit));
}

// // // Try

#[test]
fn try_ok() {
  assert!(matches!(propagate(AppResult::Ok(()), 3), AppResult::Ok(3)));
}

#[test]
fn try_err() {
  assert_eq!(
    error_message(propagate(AppResult::<()>::Err(Some("bad".into())), ())),
    Some("bad".into())
  );
  assert!(matches!(
    propagate(AppResult::<()>::Err(None), ()),
    AppResult::Err(None)
  ));
}

#[test]
fn try_quit() {
  assert!(matches!(propagate(AppResult::<()>::Quit, ()), AppResult::Quit));
}

#[test]
fn try_str() {
  fn inner() -> AppResult<()> {
    Err("string error")?;
    AppResult::Ok(())
  }

  assert_eq!(error_message(inner()), Some("string error".into()));
}

#[test]
fn try_io() {
  fn inner() -> AppResult<()> {
    Err(std::io::Error::other("io error"))?;
    AppResult::Ok(())
  }

  assert_eq!(error_message(inner()), Some("io error".into()));
}

#[test]
fn try_medic_error() {
  fn inner() -> AppResult<()> {
    Err(MedicError::Message("medic error".into()))?;
    AppResult::Ok(())
  }

  assert_eq!(error_message(inner()), Some("medic error".into()));
}

#[test]
fn try_envsubst() {
  fn inner() -> AppResult<()> {
    let vars = std::collections::HashMap::from([("BAD$KEY".to_string(), "value".to_string())]);
    envsubst::substitute("${BAD$KEY}", &vars)?;
    AppResult::Ok(())
  }

  assert!(matches!(inner(), AppResult::Err(Some(MedicError::Envsubst(_)))));
}

// // // Termination

#[test]
fn report_ok() {
  assert_eq!(AppResult::Ok(()).report(), ExitCode::from(0));
}

#[test]
fn report_err() {
  initialize();
  assert_eq!(AppResult::<()>::Err(Some("bad".into())).report(), ExitCode::from(1));
  assert_eq!(AppResult::<()>::Err(None).report(), ExitCode::from(1));
}

#[test]
fn report_quit() {
  assert_eq!(
    AppResult::<()>::Quit.report(),
    ExitCode::from(crate::QUIT_STATUS_CODE as u8)
  );
}
