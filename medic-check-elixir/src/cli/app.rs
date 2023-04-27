use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-elixir")]
/// Checks for ensuring that an Elixir project is in a good state.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks that no dependencies exist in mix.lock that are not explicitly
    /// or implicitly required in mix.exs.
    UnusedDeps(MixArgs),
}

#[derive(Args, Debug)]
pub struct MixArgs {
    /// Path to a mix project
    #[clap(value_parser)]
    #[arg(short, long, default_value = ".", value_hint = clap::ValueHint::DirPath)]
    pub cd: String,
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
