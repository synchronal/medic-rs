// @related [subject](medic-lib/src/check_result/mod.rs)

use super::*;
use std::process::{ExitCode, Termination};

fn error(msg: &str) -> CheckResult {
  CheckResult::CheckError(
    msg.to_owned(),
    Some("out".to_owned()),
    Some("err".to_owned()),
    Some("fix it".to_owned()),
  )
}

#[test]
fn json_stderr() {
  let json = CheckJson::new("boom".into(), None, Some("err".into()), None);

  assert_eq!(json.error.as_deref(), Some("\x1b[31;1mstderr:\x1b[0m\r\nerr"));
}

#[test]
fn json_stdout() {
  let json = CheckJson::new("boom".into(), Some("out".into()), None, Some("fix it".into()));

  assert_eq!(
    json.output,
    "\x1b[31;1mError: \x1b[0m boom\r\n\r\n\x1b[31;1mstdout:\x1b[0m\r\nout"
  );
  assert!(json.error.is_none());
  assert_eq!(json.remedy.as_deref(), Some("fix it"));
}

#[test]
fn json_empty() {
  let json = CheckJson::new("boom".into(), Some(String::new()), Some(String::new()), None);

  assert_eq!(json.output, "\x1b[31;1mError: \x1b[0m boom");
  assert!(json.error.is_none());
}

fn propagate(first: CheckResult, reached: &mut bool) -> CheckResult {
  first?;
  *reached = true;
  CheckResult::CheckOk
}

/// Runs `f` with MEDIC_OUTPUT_FORMAT set to `value`. Goes through `temp_env` so
/// it can't race other env-touching tests under `cargo test`.
fn with_format<R>(value: Option<&str>, f: impl FnOnce() -> R) -> R {
  temp_env::with_var("MEDIC_OUTPUT_FORMAT", value, f)
}

#[test]
fn default() {
  assert_eq!(CheckResult::default(), CheckResult::CheckOk);
}

#[test]
fn try_ok() {
  let mut reached = false;
  let result = propagate(CheckResult::CheckOk, &mut reached);
  assert!(reached);
  assert_eq!(result, CheckResult::CheckOk);
}

#[test]
fn try_err() {
  let mut reached = false;
  let result = propagate(error("boom"), &mut reached);
  assert!(!reached);
  assert_eq!(result, error("boom"));
}

#[test]
fn try_err_none() {
  let mut reached = false;
  let original = CheckResult::CheckError("boom".to_owned(), None, None, None);
  let result = propagate(original.clone(), &mut reached);
  assert!(!reached);
  assert_eq!(result, original);
}

#[test]
fn from_output() {
  assert_eq!(CheckResult::from_output(()), CheckResult::CheckOk);
}

#[test]
fn branch_err() {
  assert!(matches!(error("boom").branch(), ControlFlow::Break(_)));
  assert!(matches!(CheckResult::CheckOk.branch(), ControlFlow::Continue(())));
}

#[test]
fn report_ok() {
  assert_eq!(CheckResult::CheckOk.report(), ExitCode::from(0));
}

#[test]
fn report_err_json() {
  assert_eq!(with_format(None, || error("boom").report()), ExitCode::from(1));
}

#[test]
fn report_err_stdio() {
  assert_eq!(with_format(Some("stdio"), || error("boom").report()), ExitCode::from(1));
}

#[test]
fn format_unset() {
  assert!(matches!(
    with_format(None, CheckResultFormat::from_env),
    CheckResultFormat::Json
  ));
}

#[test]
fn format_stdio() {
  assert!(matches!(
    with_format(Some("stdio"), CheckResultFormat::from_env),
    CheckResultFormat::Stdio
  ));
}

#[test]
fn format_json() {
  assert!(matches!(
    with_format(Some("json"), CheckResultFormat::from_env),
    CheckResultFormat::Json
  ));
}

#[test]
fn format_unknown() {
  assert!(matches!(
    with_format(Some("STDIO"), CheckResultFormat::from_env),
    CheckResultFormat::Json
  ));
}
