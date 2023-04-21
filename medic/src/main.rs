#![cfg_attr(feature = "strict", deny(warnings))]

use medic::cli::app::{CliArgs, Command};
use medic_lib::config::Manifest;
use medic_lib::AppResult;

use clap::Parser;

fn main() -> AppResult<()> {
    let cli = CliArgs::parse();

    match cli.command {
        Command::Doctor(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_doctor::run_checks(manifest)
        }
        Command::Test(args) => {
            let manifest = Manifest::new(args.config)?;
            medic_test::run_steps(manifest)
        }
    }
}
