use medic_lib::StepResult;
use medic_step_git::cli::{CliArgs, Command};

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Command::Pull => medic_step_git::run_git_pull(),
        Command::Push => medic_step_git::run_git_push(),
    }
}
