#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::cli::Flags;
use medic_src::config::Manifest;
use medic_src::context::Context;
use medic_src::runnable::run;
use medic_src::AppResult;

pub fn run_steps(
  manifest: Manifest,
  progress: &mut retrogress::ProgressBar,
  flags: Flags,
  context: &Context,
) -> AppResult<()> {
  match manifest.test {
    Some(test) => {
      let mut flags = flags.clone();
      flags.recoverable = false;

      for check in test.checks {
        run(check, progress, &mut flags, context)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No test checks found in medic config.".into())),
  }
}
