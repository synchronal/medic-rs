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
    /// Checks that an archive is installed locally.
    ArchiveInstalled(ArchiveArgs),
    /// Checks that hex is installed locally.
    LocalHex,
    /// Checks that rebar is installed locally.
    LocalRebar,
    /// Checks that all Mix dependencies are compiled.
    PackagesCompiled(PackageArgs),
    /// Checks that all Mix dependencies are installed.
    PackagesInstalled(PackageArgs),
    /// Checks that no dependencies exist in mix.lock that are not explicitly
    /// or implicitly required in mix.exs.
    UnusedDeps(MixArgs),
}

#[derive(Args, Debug)]
pub struct ArchiveArgs {
    /// Name of a hex package.
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct MixArgs {
    /// Path to a mix project
    #[clap(value_parser)]
    #[arg(short, long, default_value = ".", value_hint = clap::ValueHint::DirPath)]
    pub cd: String,
}

#[derive(Args, Debug)]
pub struct PackageArgs {
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
