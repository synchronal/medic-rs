use medic_check_homebrew::cli::CliArgs;
use medic_lib::CheckResult;

fn main() -> CheckResult {
    let _cli_args = CliArgs::new();
    medic_check_homebrew::bundled()
}

