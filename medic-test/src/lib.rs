#![cfg_attr(feature = "strict", deny(warnings))]

use medic_lib::config::Manifest;
use medic_lib::runnable::Runnable;
use medic_lib::AppResult;

pub fn run_steps(manifest: Manifest) -> AppResult<()> {
    match manifest.test {
        Some(test) => {
            for check in test.checks {
                check.run()?;
            }
            Ok(())
        }
        None => Err("No test checks found in medic config.".into()),
    }
}
