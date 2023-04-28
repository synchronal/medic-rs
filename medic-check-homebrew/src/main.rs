use medic_check_homebrew::cli::CliArgs;
use medic_lib::CheckResult;

fn main() -> CheckResult {
    let args = CliArgs::new();
    medic_check_homebrew::bundled(args.cd)
}
