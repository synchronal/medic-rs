#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::cli::Flags;
use medic_src::config::Manifest;
use medic_src::runnable::run;
use medic_src::AppResult;

pub fn run_checks(manifest: Manifest, progress: &mut retrogress::ProgressBar, flags: Vec<Flags>) -> AppResult<()> {
  match manifest.outdated {
    Some(outdated) => {
      for check in outdated.checks {
        run(check, progress, &flags)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No outdated checks found in medic config.".into())),
  }
}
