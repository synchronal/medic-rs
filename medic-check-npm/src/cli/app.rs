use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-npm")]
/// Checks for ensuring that NPM dependencies are
/// properly installed.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks that NPM exists in the PATH.
    Exists,
    /// Checks that all NPM dependencies are installed.
    PackagesInstalled(PackageArgs),
}

#[derive(Args, Debug)]
pub struct PackageArgs {
    /// Path to a node project
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub cd: Option<String>,

    /// Npm prefix
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub prefix: Option<String>,
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
