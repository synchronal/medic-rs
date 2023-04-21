use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Step {
    #[serde(rename(deserialize = "shell"))]
    Shell(ShellConfig),
    #[serde(rename(deserialize = "step"))]
    Step(StepConfig),
}

#[derive(Debug, Deserialize)]
pub struct ShellConfig {}

#[derive(Debug, Deserialize)]
pub struct StepConfig {}
