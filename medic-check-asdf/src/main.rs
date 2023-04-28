use medic_check_asdf::cli::{CliArgs, Command as Cmd};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Cmd::PackageInstalled(args) => {
            medic_check_asdf::package_installed(args.plugin, args.version)?
        }
        Cmd::PluginInstalled(args) => medic_check_asdf::plugin_installed(args.plugin)?,
    }
    CheckOk
}
