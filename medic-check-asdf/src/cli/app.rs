use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-asdf")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks whether an ASDF package for a given plugin is installed.
    PackageInstalled(AsdfPackageArgs),
    /// Checks whether an ASDF plugin is installed.
    PluginInstalled(AsdfPluginArgs),
}

#[derive(Args, Debug)]
pub struct AsdfPackageArgs {
    /// Name of an ASDF plugin.
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub plugin: String,

    #[arg(short, long, default_value = "", value_hint = clap::ValueHint::CommandString)]
    pub version: String,
}

#[derive(Args, Debug)]
pub struct AsdfPluginArgs {
    /// Name of an ASDF plugin.
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub plugin: String,
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
