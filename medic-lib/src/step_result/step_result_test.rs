// @related [subject](medic-lib/src/step_result/mod.rs)

use super::*;
use std::process::{ExitCode, Termination};

fn propagate(first: StepResult, reached: &mut bool) -> StepResult {
  first?;
  *reached = true;
  StepResult::StepOk
}

#[test]
fn try_ok() {
  let mut reached = false;
  let result = propagate(StepResult::StepOk, &mut reached);
  assert!(reached);
  assert!(matches!(result, StepResult::StepOk));
}

#[test]
fn try_err() {
  let mut reached = false;
  let original = StepResult::StepError("boom".to_owned(), Some("out".to_owned()), Some("err".to_owned()));
  let result = propagate(original, &mut reached);
  assert!(!reached);
  match result {
    StepResult::StepError(msg, stdout, stderr) => {
      assert_eq!(msg, "boom");
      assert_eq!(stdout.as_deref(), Some("out"));
      assert_eq!(stderr.as_deref(), Some("err"));
    }
    StepResult::StepOk => panic!("expected StepError"),
  }
}

#[test]
fn try_err_none() {
  let mut reached = false;
  let result = propagate(StepResult::StepError("boom".to_owned(), None, None), &mut reached);
  assert!(!reached);
  assert!(matches!(result, StepResult::StepError(ref m, None, None) if m == "boom"));
}

#[test]
fn from_output() {
  assert!(matches!(StepResult::from_output(()), StepResult::StepOk));
}

#[test]
fn branch_err() {
  let error = StepResult::StepError("boom".to_owned(), None, None);
  assert!(matches!(error.branch(), ControlFlow::Break(_)));
  assert!(matches!(StepResult::StepOk.branch(), ControlFlow::Continue(())));
}

#[test]
fn report_ok() {
  assert_eq!(StepResult::StepOk.report(), ExitCode::from(0));
}

#[test]
fn report_err() {
  let error = StepResult::StepError("boom".to_owned(), Some("out".to_owned()), Some(String::new()));
  assert_eq!(error.report(), ExitCode::from(1));
}
