#![cfg_attr(feature = "strict", deny(warnings))]

use medic_lib::config::Manifest;
use medic_lib::runnable::Runnable;
use medic_lib::std_to_string;
use medic_lib::AppResult;
use medic_lib::AuditStep;

use std::io::{self, Write};
use std::process::Stdio;

pub fn run_steps(manifest: Manifest) -> AppResult<()> {
    match manifest.audit {
        Some(audit) => {
            for step in audit.checks {
                run_step(step)?;
            }
            Ok(())
        }
        None => Err("No test checks found in medic config.".into()),
    }
}

fn run_step(step: AuditStep) -> AppResult<()> {
    let allow_failure = match &step {
        AuditStep::Check(_) => false,
        AuditStep::Shell(config) => config.allow_failure,
        AuditStep::Step(_) => false,
    };
    let verbose = step.verbose();

    print!("\x1b[32m• \x1b[0{step}  …");
    io::stdout().flush().unwrap();
    if let Some(mut command) = step.to_command() {
        if verbose {
            print!("\r\n");
            command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        }
        match command.output() {
            Ok(result) => {
                if result.status.success() {
                    println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                    Ok(())
                } else {
                    println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                    eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                    eprint!("{}", std_to_string(result.stderr));
                    if allow_failure {
                        eprintln!("\x1b[32m(continuing)\x1b[0m");
                        Ok(())
                    } else {
                        Err("".into())
                    }
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
    } else {
        Err("Failed to parse command".into())
    }
}
