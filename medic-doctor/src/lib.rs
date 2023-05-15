#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::config::Manifest;
use medic_src::runnable::Runnable;
use medic_src::AppResult;

pub fn run_checks(manifest: Manifest) -> AppResult<()> {
    match manifest.doctor {
        Some(doctor) => {
            for check in doctor.checks {
                check.run()?;
            }
            AppResult::Ok(())
        }
        None => AppResult::Err(Some("No doctor checks found in medic config.".into())),
    }
}
