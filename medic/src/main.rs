#![cfg_attr(feature = "strict", deny(warnings))]

use medic::cli::app::{CliArgs, Command};
use medic_src::config::Manifest;
use medic_src::AppResult;

use clap::Parser;

fn main() -> AppResult<()> {
    let cli = CliArgs::parse();
    let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());

    match cli.command {
        Command::Audit(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_audit::run_steps(manifest, &mut progress)
        }
        Command::Doctor(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_doctor::run_checks(manifest, &mut progress)
        }
        Command::Init(args) => medic_init::create_config_file(args.config, args.force),
        Command::Outdated(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_outdated::run_checks(manifest, &mut progress)
        }
        Command::Run(args) => medic_run::run_shell(
            args.name,
            args.cmd,
            args.cd,
            args.remedy,
            args.verbose,
            &mut progress,
        ),
        Command::Test(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_test::run_steps(manifest, &mut progress)
        }
        Command::Update(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_update::run_steps(manifest, &mut progress)
        }
        Command::Shipit(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_shipit::run_steps(manifest, &mut progress)
        }
    }
}
