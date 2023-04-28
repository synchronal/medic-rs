use medic_check_hex::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::ArchiveInstalled(args) => medic_check_hex::archive_installed(args.name)?,
        Command::LocalHex => medic_check_hex::local_mix_installed()?,
        Command::LocalRebar => medic_check_hex::local_rebar_installed()?,
        Command::PackagesCompiled(args) => medic_check_hex::packages_compiled(args.cd)?,
        Command::PackagesInstalled(args) => medic_check_hex::packages_installed(args.cd)?,
    }
    CheckOk
}
