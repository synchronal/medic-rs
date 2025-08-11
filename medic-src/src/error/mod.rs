use std::fmt;

#[derive(Debug)]
pub enum MedicError {
  Io(std::io::Error),
  Arboard(arboard::Error),
  Envsubst(envsubst::Error),
  OsString(std::ffi::OsString),
  Message(String),
  Other(Box<dyn std::error::Error>),
}

impl fmt::Display for MedicError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MedicError::Io(e) => e.fmt(f),
      MedicError::Arboard(e) => e.fmt(f),
      MedicError::Envsubst(e) => e.fmt(f),
      MedicError::OsString(os) => write!(f, "Invalid OsString: {os:?}"),
      MedicError::Message(msg) => write!(f, "{msg}"),
      MedicError::Other(e) => e.fmt(f),
    }
  }
}

impl std::error::Error for MedicError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      MedicError::Io(e) => Some(e),
      MedicError::Arboard(e) => Some(e),
      MedicError::Envsubst(e) => Some(e),
      MedicError::OsString(_) => None,
      MedicError::Message(_) => None,
      MedicError::Other(e) => Some(e.as_ref()),
    }
  }
}

impl From<std::io::Error> for MedicError {
  fn from(err: std::io::Error) -> Self {
    MedicError::Io(err)
  }
}

impl From<arboard::Error> for MedicError {
  fn from(err: arboard::Error) -> Self {
    MedicError::Arboard(err)
  }
}

impl From<envsubst::Error> for MedicError {
  fn from(err: envsubst::Error) -> Self {
    MedicError::Envsubst(err)
  }
}

impl From<std::ffi::OsString> for MedicError {
  fn from(err: std::ffi::OsString) -> Self {
    MedicError::OsString(err)
  }
}

impl From<&str> for MedicError {
  fn from(err: &str) -> Self {
    MedicError::Message(err.to_string())
  }
}

impl From<String> for MedicError {
  fn from(err: String) -> Self {
    MedicError::Message(err)
  }
}

impl From<Box<dyn std::error::Error>> for MedicError {
  fn from(err: Box<dyn std::error::Error>) -> Self {
    MedicError::Other(err)
  }
}

impl From<terminal_colorsaurus::Error> for MedicError {
  fn from(err: terminal_colorsaurus::Error) -> Self {
    MedicError::Other(Box::new(err))
  }
}
