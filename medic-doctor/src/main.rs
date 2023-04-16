#![cfg_attr(feature = "strict", deny(warnings))]

use medic::config::{Check, Manifest};
use medic::AppResult;
use medic_doctor::cli::CliArgs;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io::stdout;

fn main() -> AppResult<()> {
    let cli_args = CliArgs::parse();

    if let Some(completion) = cli_args.completion {
        let mut cmd = CliArgs::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    let manifest = Manifest::new(cli_args.config)?;

    match manifest.doctor {
        Some(doctor) => {
            for check in doctor.checks {
                run_check(check)?;
            }
            Ok(())
        }
        None => Err("No doctor checks found in medic config.".into()),
    }
}

fn run_check(check: Check) -> AppResult<()> {
    let mut command = check.to_command();
    match command.output() {
        Ok(_) => Ok(()),
        Err(err) => {
            let mut error: String = "Check failed!\r\n".to_owned();
            error.push_str("Command:\r\n");
            error.push_str(&format!("{command:?}\r\n"));
            error.push_str(&format!("Error:\r\n{err:?}"));

            Err(error.into())
        }
    }
}
