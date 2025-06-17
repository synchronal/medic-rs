#![cfg_attr(feature = "strict", deny(warnings))]

use clap::Parser;
use console::Term;
use medic::cli::app::{CliArgs, Command};
use medic_src::config::Manifest;
use medic_src::context::Context;
use medic_src::AppResult;
use std::panic;

fn main() -> AppResult<()> {
  let context = Context::new();
  let cli = CliArgs::parse();

  ctrlc::set_handler(reset_term).expect("Unable to set Ctrl-C handler");

  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);
  let _ = Term::stderr().hide_cursor();
  let _ = Term::stdout().hide_cursor();

  let result = panic::catch_unwind(|| {
    let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());
    match cli.command {
      Command::Audit(args) => {
        let manifest = Manifest::new(&args.config)?;
        medic_audit::run_steps(manifest, &mut progress, args.into(), &context)
      }
      Command::Doctor(args) => {
        let manifest = Manifest::new(&args.config)?;
        medic_doctor::run_checks(manifest, &mut progress, args.into(), &context)
      }
      Command::Init(args) => medic_init::create_config_file(args.config, args.force),
      Command::Outdated(args) => {
        let manifest = Manifest::new(&args.config)?;
        medic_outdated::run_checks(manifest, &mut progress, args.into(), &context)
      }
      Command::Run(args) => {
        medic_run::run_shell(args.name, args.cmd, args.cd, args.remedy, args.verbose, &mut progress)
      }
      Command::Test(args) => {
        let manifest = Manifest::new(&args.config)?;
        medic_test::run_steps(manifest, &mut progress, args.into(), &context)
      }
      Command::Update(args) => {
        let manifest = Manifest::new(&args.config)?;
        medic_update::run_steps(manifest, &mut progress, args.into(), &context)
      }
      Command::Shipit(args) => {
        let manifest = Manifest::new(&args.config)?;
        medic_shipit::run_steps(manifest, &mut progress, args.into(), &context)
      }
    }
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
