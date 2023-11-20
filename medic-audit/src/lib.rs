#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::config::Manifest;
use medic_src::runnable::Runnable;
use medic_src::AppResult;

pub fn run_steps(manifest: Manifest, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
    match manifest.audit {
        Some(audit) => {
            for step in audit.checks {
                step.run(progress)?;
            }
            AppResult::Ok(())
        }
        None => AppResult::Err(Some("No audit checks found in medic config.".into())),
    }
}
