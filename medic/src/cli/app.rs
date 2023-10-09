use std::path::PathBuf;

use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic")]
/// Run medic workflow management commands.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
#[clap(infer_subcommands = true)]
pub enum Command {
    /// Runs checks to audit the application. Checks might include linters
    /// and dep audits.
    Audit(ManifestArgs),
    /// Runs checks to ensure that a project is fully set up for development.
    Doctor(ManifestArgs),
    /// Runs an application's tests.
    Test(ManifestArgs),
    /// Update the current application
    Update(ManifestArgs),
    /// Ship changes. Typically configured to audit, update, test, then release.
    Shipit(ManifestArgs),
}

#[derive(Args, Debug)]
pub struct ManifestArgs {
    /// Name of a hex package.
    #[clap(value_parser)]
    #[arg(short, long, env = "MEDIC_CONFIG", default_value = "${PWD}/.config/medic.toml", value_hint = clap::ValueHint::FilePath)]
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
