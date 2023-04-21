#![cfg_attr(feature = "strict", deny(warnings))]

use medic_lib::config::Manifest;
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

fn run_step(_step: Step) -> AppResult<()> {
    Ok(())
}
