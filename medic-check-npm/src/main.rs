use medic_check_npm::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::Exists => medic_check_npm::npm_exists()?,
        Command::PackagesInstalled(args) => {
            medic_check_npm::packages_installed(args.cd, args.prefix)?
        }
    }
    CheckOk
}
