use medic_audit::cli::CliArgs;
use medic_audit::run_steps;
use medic_src::config::Manifest;
use medic_src::context::Context;
use medic_src::{AppResult, progress, theme};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io::stdout;
use std::panic;

fn main() -> AppResult<()> {
  let context = Context::new();
  let cli_args = CliArgs::parse();
  theme::set_theme((&cli_args.theme).into());

  if let Some(completion) = cli_args.completion {
    let mut cmd = CliArgs::command();
    let name = cmd.get_name().to_string();
    generate(completion, &mut cmd, name, &mut stdout());

    std::process::exit(0);
  }

  let manifest = Manifest::new(&cli_args.config)?;

  ctrlc::set_handler(interrupt).expect("Unable to set Ctrl-C handler");

  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);

  let result = panic::catch_unwind(|| {
    let flags = cli_args.into();
    let mut progress = progress::new(&flags);
    run_steps(manifest, &mut progress, flags, &context)
  });

  match result {
    Ok(inner) => inner,
    Err(_) => std::process::exit(1),
  }
}

fn interrupt() {
  std::process::exit(1);
}
