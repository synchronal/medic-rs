use medic_check_asdf::cli::{CliArgs, Command as Cmd};
use medic_lib::std_to_string;
use medic_lib::CheckResult::{self, CheckError, CheckOk};

use std::process::Command;

fn main() -> CheckResult {
    let cli = CliArgs::new();
    asdf_installed()?;

    match cli.command {
        Cmd::PackageInstalled(args) => package_installed(args.plugin, args.version)?,
        Cmd::PluginInstalled(args) => plugin_installed(args.plugin)?,
    }
    CheckOk
}

fn asdf_installed() -> CheckResult {
    match Command::new("which").args(["asdf"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError("Unable to find asdf.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("open https://asdf-vm.com/guide/getting-started.html#community-supported-download-methods".into())
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for asdf. Is `which` in your PATH?".into(),
            None,
            None,
            Some("open https://asdf-vm.com/guide/getting-started.html#community-supported-download-methods".into())
        ),
    }
}

fn package_installed(plugin: String, version: Option<String>) -> CheckResult {
    let mut command = Command::new("asdf");
    let mut remedy = Command::new("asdf");

    command.arg("where").arg(&plugin);
    remedy.arg("install").arg(&plugin);

    if let Some(package_version) = version {
        command.arg(&package_version);
        remedy.arg(&package_version);
    }

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(output.stdout);
                let stderr = std_to_string(output.stderr);
                CheckError(
                    format!("Currently configured ASDF package for {plugin} has not been installed."),
                    Some(stdout),
                    Some(stderr),
                    Some(format!("({remedy:?})").replace("\"", "")),
                )
            }
        },
        Err(_err) => CheckError(
            "Unable to determine which asdf packages are installed.".into(),
            None,
            None,
            Some("open https://asdf-vm.com/guide/getting-started.html#community-supported-download-methods".into()),
        )
    }
}

fn plugin_installed(plugin: String) -> CheckResult {
    match Command::new("asdf").args(["plugin", "list"]).output() {
        Ok(list) => {
            let plugin_list = std_to_string(list.stdout);
            let plugins: Vec<String> = plugin_list.split('\n').map(str::to_string).collect();
            if plugins.contains(&plugin) {
                CheckOk
            } else {
                CheckError(
                    format!("ASDF plugin {plugin} has not been installed."),
                    Some(plugin_list),
                    None,
                    Some(format!("asdf plugin install {plugin}")),
                )
            }
        },
        Err(_err) => CheckError(
            "Unable to determine which asdf plugins are installed.".into(),
            None,
            None,
            Some("open https://asdf-vm.com/guide/getting-started.html#community-supported-download-methods".into()),
        )
    }
}
