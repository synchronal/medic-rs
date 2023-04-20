use std::path::PathBuf;

use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic")]
/// Run medic workflow managedment commands.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Runs a series of checks to ensure that a project is fully set up
    /// for development.
    Doctor(DoctorArgs),
}

#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Name of a hex package.
    #[clap(value_parser)]
    #[arg(short, long, env = "MEDIC_CONFIG", default_value = "$PWD/.medic/config.toml", value_hint = clap::ValueHint::FilePath)]
    pub config: PathBuf,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}
