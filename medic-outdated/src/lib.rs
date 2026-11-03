// @related [test](medic-outdated/src/lib_test.rs)

#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

#[cfg(test)]
mod lib_test;

use medic_src::AppResult;
use medic_src::cli::Flags;
use medic_src::config::Manifest;
use medic_src::context::Context;
use medic_src::runnable::run;

pub fn run_checks(
  manifest: Manifest,
  progress: &mut retrogress::ProgressBar,
  mut flags: Flags,
  context: &Context,
) -> AppResult<()> {
  match manifest.outdated {
    Some(outdated) => {
      for check in outdated.checks {
        run(check, progress, &mut flags, context)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No outdated checks found in medic config.".into())),
  }
}
