#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::config::Manifest;
use medic_src::runnable::run;
use medic_src::AppResult;

pub fn run_steps(manifest: Manifest, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
  match manifest.shipit {
    Some(shipit) => {
      for step in shipit.steps {
        run(step, progress)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No shipit checks found in medic config.".into())),
  }
}
