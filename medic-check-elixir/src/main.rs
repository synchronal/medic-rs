use medic_check_elixir::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::UnusedDeps(args) => medic_check_elixir::check_unused_deps(args.cd)?,
    }
    CheckOk
}
