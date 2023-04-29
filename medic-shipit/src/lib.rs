#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::config::Manifest;
use medic_lib::runnable::Runnable;
use medic_lib::AppResult;

pub fn run_steps(manifest: Manifest) -> AppResult<()> {
    match manifest.shipit {
        Some(shipit) => {
            for step in shipit.steps {
                step.run()?;
            }
            AppResult::Ok(())
        }
        None => AppResult::Err(Some("No shipit checks found in medic config.".into())),
    }
}
