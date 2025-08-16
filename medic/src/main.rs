#![cfg_attr(feature = "strict", deny(warnings))]

use clap::Parser;
use medic::cli::app::{CliArgs, Command};
use medic_src::cli::Flags;
use medic_src::config::Manifest;
use medic_src::context::Context;
use medic_src::progress;
use medic_src::theme;
use medic_src::AppResult;
use std::panic;

fn main() -> AppResult<()> {
  let context = Context::new();
  let cli = CliArgs::parse();

  ctrlc::set_handler(interrupt).expect("Unable to set Ctrl-C handler");

  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);

  let result = panic::catch_unwind(|| match cli.command {
    Command::Audit(args) => {
      theme::set_theme((&args.theme).into());
      let manifest = Manifest::new(&args.config)?;
      let flags = args.into();
      let mut progress = progress::new(&flags);
      medic_audit::run_steps(manifest, &mut progress, flags, &context)
    }
    Command::Doctor(args) => {
      theme::set_theme((&args.theme).into());
      let manifest = Manifest::new(&args.config)?;
      let flags = args.into();
      let mut progress = progress::new(&flags);
      medic_doctor::run_checks(manifest, &mut progress, flags, &context)
    }
    Command::Init(args) => medic_init::create_config_file(args.config, args.force),
    Command::Outdated(args) => {
      theme::set_theme((&args.theme).into());
      let manifest = Manifest::new(&args.config)?;
      let flags = args.into();
      let mut progress = progress::new(&flags);
      medic_outdated::run_checks(manifest, &mut progress, flags, &context)
    }
    Command::Run(args) => {
      theme::set_theme((&args.theme).into());
      let flags = Flags::default();
      let mut progress = progress::new(&flags);
      medic_run::run_shell(args.name, args.cmd, args.cd, args.remedy, args.verbose, &mut progress)
    }
    Command::Test(args) => {
      theme::set_theme((&args.theme).into());
      let manifest = Manifest::new(&args.config)?;
      let flags = args.into();
      let mut progress = progress::new(&flags);
      medic_test::run_steps(manifest, &mut progress, flags, &context)
    }
    Command::Update(args) => {
      theme::set_theme((&args.theme).into());
      let manifest = Manifest::new(&args.config)?;
      let flags = args.into();
      let mut progress = progress::new(&flags);
      medic_update::run_steps(manifest, &mut progress, flags, &context)
    }
    Command::Shipit(args) => {
      theme::set_theme((&args.theme).into());
      let manifest = Manifest::new(&args.config)?;
      let flags = args.into();
      let mut progress = progress::new(&flags);
      medic_shipit::run_steps(manifest, &mut progress, flags, &context)
    }
  });

  match result {
    Ok(inner) => inner,
    Err(_) => std::process::exit(1),
  }
}

fn interrupt() {
  std::process::exit(1);
}
