#![cfg_attr(feature = "strict", deny(warnings))]

use medic_lib::config::Manifest;
use medic_lib::std_to_string;
use medic_lib::AppResult;
use medic_step::Step;

pub fn run_steps(manifest: Manifest) -> AppResult<()> {
    match manifest.test {
        Some(test) => {
            for check in test.checks {
                run_step(check)?;
            }
            Ok(())
        }
        None => Err("No test checks found in medic config.".into()),
    }
}

fn run_step(step: Step) -> AppResult<()> {
    print!("\x1b[32m• \x1b[0");
    print!("{step}  …");
    if let Some(mut command) = step.to_command() {
        match command.output() {
            Ok(result) => {
                if result.status.success() {
                    println!("{}\x1b[32;1mOK\x1b[0m", (8u8 as char));
                    Ok(())
                } else {
                    println!("{}\x1b[31;1mFAILED\x1b[0m", (8u8 as char));
                    eprintln!("\x1b[0;31m== Step output ==\x1b[0m\r\n");
                    eprint!("{}", std_to_string(result.stderr));
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
    } else {
        Err("Failed to parse command".into())
    }
}
