// @related [test](medic-run/src/lib_test.rs)

#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

#[cfg(test)]
mod lib_test;

use medic_src::AppResult;
use medic_src::cli::Flags;
use medic_src::context::Context;
use medic_src::runnable::Runnable;
use medic_src::shell::ShellConfig;

pub fn run_shell(
  name: String,
  cmd: String,
  cd: Option<String>,
  remedy: Option<String>,
  verbose: bool,
  progress: &mut retrogress::ProgressBar,
) -> AppResult<()> {
  let context = Context::new();
  let shell = ShellConfig::new(name, cmd, cd, remedy, verbose);
  let mut flags = Flags::default();
  shell.run(progress, &mut flags, &context).into()
}
