#![cfg_attr(feature = "strict", deny(warnings))]

use medic_doctor::cli::CliArgs;
use medic_doctor::run_checks;
use medic_src::config::Manifest;
use medic_src::AppResult;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use console::Term;
use std::io::stdout;
use std::panic;

fn main() -> AppResult<()> {
  let cli_args = CliArgs::parse();

  if let Some(completion) = cli_args.completion {
    let mut cmd = CliArgs::command();
    let name = cmd.get_name().to_string();
    generate(completion, &mut cmd, name, &mut stdout());

    std::process::exit(0);
  }

  let manifest = Manifest::new(cli_args.config)?;

  ctrlc::set_handler(reset_term).expect("Unable to set Ctrl-C handler");

  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);
  let _ = Term::stderr().hide_cursor();
  let _ = Term::stdout().hide_cursor();

  let result = panic::catch_unwind(|| {
    let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());
    run_checks(manifest, &mut progress)
  });

  reset_term();

  match result {
    Ok(inner) => inner,
    Err(_) => std::process::exit(1),
  }
}

fn reset_term() {
  let _ = Term::stderr().show_cursor();
  let _ = Term::stdout().show_cursor();
}
