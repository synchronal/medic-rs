use crate::AppResult;

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub doctor: Option<DoctorConfig>,
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
            let manifest_contents = std::fs::read_to_string(expanded_path)?;
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

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[36m{:}", self.check)?;
        if let Some(command) = &self.command {
            write!(f, ": \x1b[0;36m{}?", command)?;
        }
        if let Some(args) = &self.args {
            write!(f, " \x1b[0;33m(")?;
            for value in args.values() {
                write!(f, "{value}")?;
            }
            write!(f, ")")?;
        }
        write!(f, "\x1b[0m")
    }
}
