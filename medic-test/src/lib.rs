#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::cli::Flags;
use medic_src::config::Manifest;
use medic_src::runnable::run;
use medic_src::AppResult;

pub fn run_steps(manifest: Manifest, progress: &mut retrogress::ProgressBar, mut flags: Flags) -> AppResult<()> {
  match manifest.test {
    Some(test) => {
      for check in test.checks {
        run(check, progress, &mut flags)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No test checks found in medic config.".into())),
  }
}
