// @related [subject](medic-src/src/error/mod.rs)

use super::*;
use std::error::Error;

#[derive(Debug)]
struct CustomError;

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "custom error")
  }
}

impl std::error::Error for CustomError {}

#[test]
fn io() {
  let err: MedicError = std::io::Error::other("disk on fire").into();

  assert!(matches!(err, MedicError::Io(_)));
  assert_eq!(err.to_string(), "disk on fire");
  assert_eq!(err.source().unwrap().to_string(), "disk on fire");
}

#[test]
fn envsubst() {
  let vars = std::collections::HashMap::from([("BAD$KEY".to_string(), "value".to_string())]);
  let inner = envsubst::substitute("", &vars).unwrap_err();
  let inner_message = inner.to_string();
  let err: MedicError = inner.into();

  assert!(matches!(err, MedicError::Envsubst(_)));
  assert_eq!(err.to_string(), inner_message);
  assert!(err.source().is_some());
}

#[test]
fn arboard() {
  let err: MedicError = arboard::Error::ContentNotAvailable.into();

  assert!(matches!(err, MedicError::Arboard(_)));
  assert_eq!(err.to_string(), arboard::Error::ContentNotAvailable.to_string());
  assert!(err.source().is_some());
}

#[test]
fn os_string() {
  let err: MedicError = std::ffi::OsString::from("weird").into();

  assert!(matches!(err, MedicError::OsString(_)));
  assert_eq!(err.to_string(), "Invalid OsString: \"weird\"");
  assert!(err.source().is_none());
}

#[test]
fn message() {
  let from_str: MedicError = "from str".into();
  let from_string: MedicError = String::from("from string").into();

  assert!(matches!(&from_str, MedicError::Message(m) if m == "from str"));
  assert!(matches!(&from_string, MedicError::Message(m) if m == "from string"));
  assert_eq!(from_str.to_string(), "from str");
  assert!(from_string.source().is_none());
}

#[test]
fn other() {
  let boxed: Box<dyn std::error::Error + Send + Sync> = Box::new(CustomError);
  let err: MedicError = boxed.into();

  assert!(matches!(err, MedicError::Other(_)));
  assert_eq!(err.to_string(), "custom error");
  assert_eq!(err.source().unwrap().to_string(), "custom error");
}
