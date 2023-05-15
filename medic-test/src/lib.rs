#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::config::Manifest;
use medic_src::runnable::Runnable;
use medic_src::AppResult;

pub fn run_steps(manifest: Manifest) -> AppResult<()> {
    match manifest.test {
        Some(test) => {
            for check in test.checks {
                check.run()?;
            }
            AppResult::Ok(())
        }
        None => AppResult::Err(Some("No test checks found in medic config.".into())),
    }
}
