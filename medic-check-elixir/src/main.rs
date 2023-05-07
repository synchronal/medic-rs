use medic_check_elixir::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::ArchiveInstalled(args) => medic_check_elixir::archive_installed(args.name)?,
        Command::LocalHex => medic_check_elixir::local_mix_installed()?,
        Command::LocalRebar => medic_check_elixir::local_rebar_installed()?,
        Command::PackagesCompiled(args) => medic_check_elixir::packages_compiled(args.cd)?,
        Command::PackagesInstalled(args) => medic_check_elixir::packages_installed(args.cd)?,
        Command::UnusedDeps(args) => medic_check_elixir::check_unused_deps(args.cd)?,
    }
    CheckOk
}
