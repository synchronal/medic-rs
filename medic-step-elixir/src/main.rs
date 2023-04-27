use medic_lib::StepResult::{self, StepOk};
use medic_step_elixir::cli::{CliArgs, Command};

fn main() -> StepResult {
    let cli = CliArgs::new();
    match cli.command {
        Command::AuditDeps(args) => medic_step_elixir::run_mix_audit(args)?,
        Command::Credo(args) => medic_step_elixir::run_credo(args)?,
        Command::Dialyzer(args) => medic_step_elixir::run_dialyzer(args)?,
    }
    StepOk
}
