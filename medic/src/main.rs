#![cfg_attr(feature = "strict", deny(warnings))]

use medic::cli::app::{CliArgs, Command};
use medic_lib::config::Manifest;
use medic_lib::AppResult;

use clap::Parser;

fn main() -> AppResult<()> {
    let cli = CliArgs::parse();

    match cli.command {
        Command::Audit(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_audit::run_steps(manifest)
        }
        Command::Doctor(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_doctor::run_checks(manifest)
        }
        Command::Test(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_test::run_steps(manifest)
        }
        Command::Update(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_update::run_steps(manifest)
        }
        Command::Shipit(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_shipit::run_steps(manifest)
        }
    }
}
