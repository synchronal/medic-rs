use medic_check::CheckResult::{self, CheckError, CheckOk};
use medic_check_asdf::cli::CliArgs;

use std::process::Command;

fn main() -> CheckResult {
    let _cli_args = CliArgs::new();
    asdf_installed()?;
    CheckOk
}

fn asdf_installed() -> CheckResult {
    match Command::new("which").args(["asdf"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = CheckResult::from_std(which.stdout);
                let stderr = CheckResult::from_std(which.stderr);
                CheckError("Unable to find asdf.".into(),
                stdout,
                stderr,
            "open https://asdf-vm.com/guide/getting-started.html#community-supported-download-methods".into()
                            )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for asdf. Is `which` in your PATH?".into(),
            "".into(),
            "".into(),
            "open https://asdf-vm.com/guide/getting-started.html#community-supported-download-methods".into()
        ),
    }
}
