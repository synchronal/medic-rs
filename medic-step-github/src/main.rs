use medic_lib::StepResult;
use medic_step_github::cli::{CliArgs, Command};

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Command::LinkToActions(args) => medic_step_github::link_to_actions(args),
    }
}
