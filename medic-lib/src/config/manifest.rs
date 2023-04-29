use crate::AppResult;
use crate::AuditStep;
use crate::Check;
use crate::ShipitStep;
use crate::Step;

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub audit: Option<AuditConfig>,
    pub doctor: Option<DoctorConfig>,
    pub shipit: Option<ShipitConfig>,
    pub test: Option<TestConfig>,
    pub update: Option<UpdateConfig>,
}

impl Manifest {
    pub fn new(path: PathBuf) -> AppResult<Manifest> {
        let cwd = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let mut context = std::collections::HashMap::new();
        context.insert("PWD".to_string(), cwd.clone());
        context.insert("CWD".to_string(), cwd);

        let path_expansion = envsubst::substitute(path.to_string_lossy(), &context).unwrap();
        let expanded_path = std::path::Path::new(&path_expansion);

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
                format!(
                    "Medic config file `{}` does not exist.",
                    path.to_string_lossy()
                )
                .into(),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuditConfig {
    pub checks: Vec<AuditStep>,
}

#[derive(Debug, Deserialize)]
pub struct DoctorConfig {
    pub checks: Vec<Check>,
}

#[derive(Debug, Deserialize)]
pub struct ShipitConfig {
    pub steps: Vec<ShipitStep>,
}

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub checks: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfig {
    pub steps: Vec<Step>,
}
