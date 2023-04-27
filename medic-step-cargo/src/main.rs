use medic_lib::StepResult::{self, StepOk};
use medic_step_cargo::cli::{CliArgs, Command};

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Command::Clippy => medic_step_cargo::run_clippy()?,
        Command::Test => medic_step_cargo::run_tests()?,
    }
    StepOk
}
