#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::config::Manifest;
use medic_lib::runnable::Runnable;
use medic_lib::AppResult;

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
