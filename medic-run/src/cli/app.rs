use clap::Parser;
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic run")]
pub struct CliArgs {
    #[clap(long)]
    pub cd: Option<String>,
    #[clap(short, long)]
    pub name: String,
    #[clap(short, long)]
    pub cmd: String,
    #[clap(short, long)]
    pub remedy: Option<String>,
    #[clap(long, action)]
    pub verbose: bool,

    /// Shell to generate completions for
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
