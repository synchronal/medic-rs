use medic_check_rust::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::CrateInstalled(args) => medic_check_rust::crate_installed(args.name)?,
        Command::FormatCheck(_) => medic_check_rust::check_formatting()?,
        Command::TargetInstalled(args) => medic_check_rust::target_installed(args.target)?,
    }
    CheckOk
}
