// @related [subject](medic-src/src/check/check_output.rs)

use super::check_output::*;
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

#[test]
fn deserialize() {
  let output: CheckOutput =
    serde_json::from_str(r#"{"output": "out", "error": "err", "remedy": "fix", "verbose": true}"#).unwrap();

  assert_eq!(output.stdout.as_deref(), Some("out"));
  assert_eq!(output.stderr.as_deref(), Some("err"));
  assert_eq!(output.remedy.as_deref(), Some("fix"));
  assert!(output.verbose);
}

#[test]
fn deserialize_empty() {
  let output: CheckOutput = serde_json::from_str("{}").unwrap();

  assert_eq!(output.stdout, None);
  assert_eq!(output.stderr, None);
  assert_eq!(output.remedy, None);
  assert!(!output.verbose);
}

#[test]
fn serialize() {
  let output = CheckOutput {
    stdout: Some("out".into()),
    verbose: true,
    ..Default::default()
  };

  assert_eq!(
    serde_json::to_string(&output).unwrap(),
    r#"{"stdout":"out","stderr":null,"remedy":null}"#
  );
}

#[test]
fn verbose() {
  let mut output = CheckOutput::default();

  output.verbose(true);
  assert!(output.verbose);

  output.verbose(false);
  assert!(!output.verbose);
}

#[test]
fn display_empty() {
  initialize();
  let output = CheckOutput {
    remedy: Some("fix".into()),
    ..Default::default()
  };

  assert_eq!(output.to_string(), "");
}

#[test]
fn display_stdout() {
  initialize();
  let output = CheckOutput {
    stdout: Some("out".into()),
    ..Default::default()
  };

  assert_eq!(
    output.to_string(),
    "\u{1b}[31m== Check output ==\u{1b}[0m\r\n\nout\r\n\r\n"
  );
}

#[test]
fn display_stderr() {
  initialize();
  let output = CheckOutput {
    stderr: Some("err".into()),
    ..Default::default()
  };

  assert_eq!(
    output.to_string(),
    "\u{1b}[31m== Check error ==\u{1b}[0m\r\n\nerr\r\n\r\n"
  );
}

#[test]
fn display_both() {
  initialize();
  let output = CheckOutput {
    stdout: Some("out".into()),
    stderr: Some("err".into()),
    ..Default::default()
  };

  assert_eq!(
    output.to_string(),
    "\u{1b}[31m== Check output ==\u{1b}[0m\r\n\nout\r\n\r\n\u{1b}[31m== Check error ==\u{1b}[0m\r\n\nerr\r\n\r\n"
  );
}
