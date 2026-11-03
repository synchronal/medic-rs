// @related [subject](medic-src/src/check/output_format.rs)

use super::output_format::*;
use std::os::unix::process::ExitStatusExt;
use std::process::{ExitStatus, Output};

fn output(stdout: &str, stderr: &str) -> Output {
  Output {
    status: ExitStatus::from_raw(0),
    stdout: stdout.as_bytes().to_vec(),
    stderr: stderr.as_bytes().to_vec(),
  }
}

// // // Display, FromStr, Deserialize

#[test]
fn display() {
  assert_eq!(OutputFormat::Json.to_string(), "json");
  assert_eq!(OutputFormat::Stdio.to_string(), "stdio");
}

#[test]
fn default_json() {
  assert_eq!(OutputFormat::default(), OutputFormat::Json);
}

#[test]
fn from_str() {
  assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
  assert_eq!("stdio".parse::<OutputFormat>().unwrap(), OutputFormat::Stdio);
}

#[test]
fn from_str_unknown() {
  let err = "JSON".parse::<OutputFormat>().unwrap_err();

  assert_eq!(err.to_string(), "Unknown output format: JSON");
}

#[test]
fn round_trip() {
  for format in [OutputFormat::Json, OutputFormat::Stdio] {
    assert_eq!(format.to_string().parse::<OutputFormat>().unwrap(), format);
  }
}

#[test]
fn deserialize() {
  #[derive(serde::Deserialize)]
  struct Wrapper {
    output: OutputFormat,
  }

  let json: Wrapper = toml::from_str(r#"output = "json""#).unwrap();
  let stdio: Wrapper = toml::from_str(r#"output = "stdio""#).unwrap();

  assert_eq!(json.output, OutputFormat::Json);
  assert_eq!(stdio.output, OutputFormat::Stdio);
  assert!(toml::from_str::<Wrapper>(r#"output = "xml""#).is_err());
}

// // // parse: json

#[test]
fn json() {
  let result = OutputFormat::Json.parse(
    output(r#"{"output": "out", "error": "err", "remedy": "fix it"}"#, ""),
    None,
  );

  assert_eq!(result.stdout.as_deref(), Some("out"));
  assert_eq!(result.stderr.as_deref(), Some("err"));
  assert_eq!(result.remedy.as_deref(), Some("fix it"));
  assert!(!result.verbose);
}

#[test]
fn json_stderr_fallback() {
  let result = OutputFormat::Json.parse(output(r#"{"output": "out"}"#, "  from stderr \n"), None);

  assert_eq!(result.stderr.as_deref(), Some("from stderr"));
}

#[test]
fn json_error_over_stderr() {
  let result = OutputFormat::Json.parse(output(r#"{"error": "from json"}"#, "from stderr"), None);

  assert_eq!(result.stderr.as_deref(), Some("from json"));
}

#[test]
fn json_empty() {
  let result = OutputFormat::Json.parse(output("{}", ""), None);

  assert_eq!(result.stdout, None);
  assert_eq!(result.stderr, None);
  assert_eq!(result.remedy, None);
}

#[test]
fn json_cd() {
  let result = OutputFormat::Json.parse(output(r#"{"remedy": "fix it"}"#, ""), Some("./sub".into()));

  assert_eq!(result.remedy.as_deref(), Some("(cd ./sub && fix it)"));
}

#[test]
fn json_cd_without_remedy() {
  let result = OutputFormat::Json.parse(output("{}", ""), Some("./sub".into()));

  assert_eq!(result.remedy, None);
}

#[test]
fn invalid_json() {
  let result = OutputFormat::Json.parse(output("not json", "stderr"), Some("./sub".into()));

  assert_eq!(result.stdout.as_deref(), Some("Check did not return valid JSON"));
  assert_eq!(result.stderr, None);
  assert_eq!(result.remedy, None);
}

// // // parse: stdio

#[test]
fn stdio() {
  let result = OutputFormat::Stdio.parse(output("  fix it \n", " oops \n"), None);

  assert_eq!(result.stdout, None);
  assert_eq!(result.stderr.as_deref(), Some("oops"));
  assert_eq!(result.remedy.as_deref(), Some("fix it"));
}

#[test]
fn stdio_empty() {
  let result = OutputFormat::Stdio.parse(output("", ""), Some("./sub".into()));

  assert_eq!(result.stdout, None);
  assert_eq!(result.stderr, None);
  assert_eq!(result.remedy, None);
}

#[test]
fn stdio_cd() {
  let result = OutputFormat::Stdio.parse(output("fix it\n", ""), Some("./sub".into()));

  assert_eq!(result.remedy.as_deref(), Some("(cd ./sub && fix it)"));
}
