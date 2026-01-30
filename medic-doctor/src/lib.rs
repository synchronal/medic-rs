#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

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
  match manifest.doctor {
    Some(doctor) => {
      for check in doctor.checks {
        run(check, progress, &mut flags, context)?;
      }
      AppResult::Ok(())
    }
    None => AppResult::Err(Some("No doctor checks found in medic config.".into())),
  }
}
