// @related [tests](medic-src/src/config/manifest_test.rs)

use crate::AppResult;
use crate::AuditStep;
use crate::DoctorStep;
use crate::OutdatedCheck;
use crate::ShipitStep;
use crate::Step;

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Manifest {
  pub audit: Option<AuditConfig>,
  pub doctor: Option<DoctorConfig>,
  pub outdated: Option<OutdatedConfig>,
  pub shipit: Option<ShipitConfig>,
  pub test: Option<TestConfig>,
  pub update: Option<UpdateConfig>,
}

impl Manifest {
  pub fn new(path: &Path) -> AppResult<Manifest> {
    let cwd = std::env::current_dir()?.into_os_string().into_string()?;
    let mut context = std::collections::HashMap::new();
    context.insert("CWD".to_string(), cwd);
    for (key, value) in std::env::vars() {
      if value.contains(['{', '}']) {
        continue;
      };
      context.insert(key, value);
    }

    let path_expansion = envsubst::substitute(path.to_string_lossy(), &context)?;
    let expanded_path = std::path::Path::new(&path_expansion);

    std::env::set_var("MEDIC_CONFIG", path);

    if expanded_path.exists() {
      match std::fs::read_to_string(expanded_path) {
        Ok(manifest_contents) => match toml::from_str(&manifest_contents) {
          Ok(manifest) => AppResult::Ok(manifest),
          Err(err) => AppResult::Err(Some(
            format!("Unable to parse manifest {expanded_path:?}\r\n{err}")
              .replace('"', "")
              .into(),
          )),
        },
        Err(err) => AppResult::Err(Some(err.into())),
      }
    } else {
      AppResult::Err(Some(
        format!("Medic config file `{}` does not exist.", path.to_string_lossy()).into(),
      ))
    }
  }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct AuditConfig {
  pub checks: Vec<AuditStep>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct DoctorConfig {
  pub checks: Vec<DoctorStep>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct OutdatedConfig {
  pub checks: Vec<OutdatedCheck>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ShipitConfig {
  pub steps: Vec<ShipitStep>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct TestConfig {
  pub checks: Vec<Step>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateConfig {
  pub steps: Vec<Step>,
}
