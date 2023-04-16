use crate::AppResult;

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub doctor: Option<DoctorConfig>,
}

impl Manifest {
    pub fn new(path: PathBuf) -> AppResult<Manifest> {
        if path.exists() {
            let manifest_contents = std::fs::read_to_string(path)?;
            let manifest: Manifest = toml::from_str(&manifest_contents)?;
            Ok(manifest)
        } else {
            Err(format!(
                "Medic config file `{}` does not exist.",
                path.to_string_lossy()
            )
            .into())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DoctorConfig {
    pub checks: Vec<Check>,
}

#[derive(Debug, Deserialize)]
pub struct Check {
    pub check: String,
    pub command: String,
    pub args: HashMap<String, String>,
}
