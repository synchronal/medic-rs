use super::check_output::CheckOutput;
use crate::std_to_string;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum OutputFormat {
    #[default]
    #[serde(rename(deserialize = "json"))]
    Json,
    #[serde(rename(deserialize = "stdio"))]
    Stdio,
}

impl OutputFormat {
    pub(crate) fn parse(self, result: std::process::Output, cd: Option<String>) -> CheckOutput {
        match self {
            OutputFormat::Json => {
                let stdout = std_to_string(result.stdout);
                let stderr = std_to_string(result.stderr);
                let o: Result<CheckOutput, serde_json::Error> = serde_json::from_str(&stdout);
                match o {
                    Ok(mut check_output) => {
                        if check_output.stderr.is_none() && !stderr.is_empty() {
                            check_output.stderr = Some(stderr.trim().to_owned());
                        }
                        if cd.is_some() && check_output.remedy.is_some() {
                            let directory = cd.unwrap();
                            let remedy = check_output.remedy.unwrap();
                            check_output.remedy = Some(format!("(cd {directory} && {remedy})"))
                        }
                        check_output
                    }
                    Err(_err) => CheckOutput {
                        stdout: Some("Check did not return valid JSON".into()),
                        ..Default::default()
                    },
                }
            }
            OutputFormat::Stdio => {
                let stderr = if result.stderr.is_empty() {
                    None
                } else {
                    Some(std_to_string(result.stderr).trim().to_owned())
                };
                let remedy = if result.stdout.is_empty() {
                    None
                } else {
                    let remedy = std_to_string(result.stdout).trim().to_owned();
                    if cd.is_some() {
                        let dir = cd.unwrap();
                        Some(format!("(cd {dir} && {remedy})"))
                    } else {
                        Some(format!("{remedy}"))
                    }
                };

                CheckOutput {
                    stderr,
                    remedy,
                    ..Default::default()
                }
            }
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Stdio => write!(f, "stdio"),
        }
    }
}
