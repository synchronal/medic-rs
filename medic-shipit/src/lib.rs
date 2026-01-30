#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::AppResult;
use medic_src::cli::Flags;
use medic_src::config::Manifest;
use medic_src::context::Context;
use medic_src::runnable::run;

pub fn run_steps(
  manifest: Manifest,
  progress: &mut retrogress::ProgressBar,
  mut flags: Flags,
  context: &Context,
) -> AppResult<()> {
  match manifest.shipit {
    Some(shipit) => {
      for step in shipit.steps {
        run(step, progress, &mut flags, context)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No shipit checks found in medic config.".into())),
  }
}
