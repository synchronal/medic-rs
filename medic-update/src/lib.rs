#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::config::Manifest;
use medic_src::runnable::Runnable;
use medic_src::AppResult;

pub fn run_steps(manifest: Manifest, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
  match manifest.update {
    Some(test) => {
      for step in test.steps {
        step.run(progress)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No update steps found in medic config.".into())),
  }
}
