// @related [subject](medic-src/src/recoverable/mod.rs)

use super::*;
use std::process::{ExitCode, Termination};
use std::sync::Once;

static INIT: Once = Once::new();
pub fn initialize() {
  INIT.call_once(|| {
    if crate::theme::THEME.get().is_none() {
      let theme = crate::theme::dark_theme();
      crate::theme::set_theme(theme);
    }
  });
}

fn propagate<T, U>(inner: Recoverable<T>, after: U) -> Recoverable<U> {
  inner?;
  Recoverable::Ok(after)
}

fn error_message<T>(recoverable: Recoverable<T>) -> Option<String> {
  match recoverable {
    Recoverable::Err(Some(err), None) => Some(err.to_string()),
    _ => None,
  }
}

// // // Remedy

#[test]
fn remedy_display() {
  let remedy = Remedy::new("bundle install".into(), None, BTreeMap::new());

  assert_eq!(remedy.to_string(), "bundle install");
}

#[test]
fn remedy_display_cd() {
  let remedy = Remedy::new("bundle install".into(), Some("./sub".into()), BTreeMap::new());

  assert_eq!(remedy.to_string(), "(cd ./sub && bundle install)");
}

#[test]
fn remedy_command() {
  let dir = std::env::temp_dir();
  let remedy = Remedy::new(
    "echo hi".into(),
    Some(dir.to_string_lossy().into_owned()),
    BTreeMap::from([("REMEDY_VAR".to_string(), "value".to_string())]),
  );

  let command = remedy.to_command().unwrap();

  assert_eq!(command.get_program(), "sh");
  assert_eq!(command.get_args().collect::<Vec<_>>(), vec!["-c", "echo hi"]);
  assert_eq!(
    command.get_current_dir(),
    Some(std::fs::canonicalize(&dir).unwrap().as_path())
  );
  assert!(
    command
      .get_envs()
      .any(|(k, v)| k == "REMEDY_VAR" && v == Some(std::ffi::OsStr::new("value")))
  );
}

#[test]
fn missing_cd() {
  let missing = std::env::temp_dir()
    .join(format!("medic-recoverable-{}-missing", std::process::id()))
    .display()
    .to_string();
  let remedy = Remedy::new("echo hi".into(), Some(missing.clone()), BTreeMap::new());

  let err = remedy.to_command().unwrap_err();

  assert_eq!(err.to_string(), format!("directory {missing} does not exist"));
}

// // // Try

#[test]
fn try_ok() {
  assert!(matches!(propagate(Recoverable::Ok(()), 7), Recoverable::Ok(7)));
}

#[test]
fn try_optional() {
  let optional = Recoverable::Optional((), Some(Remedy::new("fix".into(), None, BTreeMap::new())));

  assert!(matches!(propagate(optional, 7), Recoverable::Ok(7)));
}

#[test]
fn try_err() {
  let err: Recoverable<()> = Recoverable::Err(
    Some("bad".into()),
    Some(Remedy::new("fix".into(), None, BTreeMap::new())),
  );

  assert_eq!(error_message(propagate(err, ())), Some("bad".into()));
}

#[test]
fn try_manual() {
  let manual: Recoverable<()> = Recoverable::Manual(
    Some("manual".into()),
    Some(Remedy::new("fix".into(), None, BTreeMap::new())),
  );

  assert_eq!(error_message(propagate(manual, ())), Some("manual".into()));
}

#[test]
fn try_nonrecoverable() {
  let nonrecoverable: Recoverable<()> = Recoverable::Nonrecoverable("fatal".into());

  assert_eq!(error_message(propagate(nonrecoverable, ())), Some("fatal".into()));
}

#[test]
fn try_quit() {
  let quit: Recoverable<()> = Recoverable::Quit;

  assert!(matches!(propagate(quit, ()), Recoverable::Err(None, None)));
}

#[test]
fn try_str() {
  fn inner() -> Recoverable<()> {
    Err("string error")?;
    Recoverable::Ok(())
  }

  assert_eq!(error_message(inner()), Some("string error".into()));
}

#[test]
fn try_io() {
  fn inner() -> Recoverable<()> {
    Err(std::io::Error::other("io error"))?;
    Recoverable::Ok(())
  }

  assert_eq!(error_message(inner()), Some("io error".into()));
}

#[test]
fn try_os_string() {
  fn inner() -> Recoverable<()> {
    Err(std::ffi::OsString::from("bad"))?;
    Recoverable::Ok(())
  }

  assert_eq!(error_message(inner()), Some("Invalid OsString: \"bad\"".into()));
}

#[test]
fn try_envsubst() {
  fn inner() -> Recoverable<()> {
    let vars = std::collections::HashMap::from([("BAD$KEY".to_string(), "value".to_string())]);
    envsubst::substitute("${BAD$KEY}", &vars)?;
    Recoverable::Ok(())
  }

  assert!(matches!(inner(), Recoverable::Err(Some(MedicError::Envsubst(_)), None)));
}

// // // Termination

#[test]
fn report_ok() {
  initialize();
  assert_eq!(Recoverable::Ok(()).report(), ExitCode::from(0));
  assert_eq!(Recoverable::Optional((), None).report(), ExitCode::from(0));
}

#[test]
fn report_err() {
  initialize();
  assert_eq!(
    Recoverable::<()>::Err(Some("bad".into()), None).report(),
    ExitCode::from(1)
  );
  assert_eq!(Recoverable::<()>::Err(None, None).report(), ExitCode::from(1));
  assert_eq!(Recoverable::<()>::Manual(None, None).report(), ExitCode::from(1));
}

#[test]
fn report_quit() {
  initialize();
  let quit = ExitCode::from(crate::QUIT_STATUS_CODE as u8);

  assert_eq!(Recoverable::<()>::Nonrecoverable("fatal".into()).report(), quit);
  assert_eq!(Recoverable::<()>::Quit.report(), quit);
}
