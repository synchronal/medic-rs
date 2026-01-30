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
  match manifest.audit {
    Some(audit) => {
      for step in audit.checks {
        run(step, progress, &mut flags, context)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No audit checks found in medic config.".into())),
  }
}
