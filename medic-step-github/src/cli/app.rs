use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-step-github")]
/// Steps for interacting with github.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Look at the project's git config to link to github actions.
    /// This should always be run verbose.
    LinkToActions(GithubActionsArgs),
}

#[derive(Args, Debug)]
pub struct GithubActionsArgs {
    /// Name of a git remote.
    #[clap(value_parser)]
    #[arg(short, long, default_value = "origin", value_hint = clap::ValueHint::CommandString)]
    pub remote: String,
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
