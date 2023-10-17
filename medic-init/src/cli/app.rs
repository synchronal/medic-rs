use clap::Parser;
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic init")]
pub struct CliArgs {
    /// Path to a file where medic config can be found
    #[clap(
        short,
        long,
        env = "MEDIC_CONFIG",
        default_value = "${PWD}/.config/medic.toml"
    )]
    pub config: std::path::PathBuf,

    /// Shell to generate completions for
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
