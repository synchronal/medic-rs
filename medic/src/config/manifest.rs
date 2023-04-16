use crate::AppResult;

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

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
    pub command: Option<String>,
    pub args: Option<HashMap<String, String>>,
}

impl Check {
    pub fn to_command(self) -> Command {
        let mut check_cmd: String = "medic-check-".to_owned();
        check_cmd.push_str(&self.check);
        let mut command = Command::new(check_cmd);

        if let Some(subcmd) = self.command {
            command.arg(subcmd);
        }
        if let Some(args) = self.args {
            for (flag, value) in args {
                let mut flag_arg = "--".to_owned();
                flag_arg.push_str(&flag);
                command.arg(flag_arg).arg(value);
            }
        }

        command
    }
}
