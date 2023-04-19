#![cfg_attr(feature = "strict", deny(warnings))]

use medic::config::{Check, Manifest};
use medic::AppResult;
use medic_check::std_to_string;
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
    print!("\x1b[32m• \x1b[0");
    print!("{check}  …");
    let mut command = check.to_command();
    match command.output() {
        Ok(result) => {
            if result.status.success() {
                println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                Ok(())
            } else {
                println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                eprint!("\x1b[0;31m{}\x1b[0m", std_to_string(result.stderr));
                // let mut error: String = "Check failed!\r\n".to_owned();
                // error.push_str("Command:\r\n");
                // error.push_str(&format!("{command:?}\r\n"));
                println!(
                    "\x1b[36mPossible remedy: \x1b[0;33m{}\x1b[0m",
                    std_to_string(result.stdout)
                );
                Err("".into())
            }
        }
        Err(err) => {
            println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
            let mut error: String = "Check failed!\r\n".to_owned();
            error.push_str("Command:\r\n");
            error.push_str(&format!("{command:?}\r\n"));
            error.push_str(&format!("Error:\r\n{err:?}"));

            Err(error.into())
        }
    }
}
