use clap::Parser;
use clap_complete::Shell;
use indoc::indoc;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic")]
#[clap( after_help = indoc!(
    "
    COMMON TASKS:
        You can install medic into a new repository using
            medic init
    "
))]
pub struct CliArgs {
    /// Path to a file where medic config can be found
    #[clap(
        short,
        long,
        env = "MEDIC_CONFIG",
        default_value = "$HOME/.medic/config.toml"
    )]
    pub config: String,

    /// Shell to generate completions for
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
