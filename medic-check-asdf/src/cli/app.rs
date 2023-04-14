use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-homebrew")]
pub struct CliArgs {}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}

impl Default for CliArgs {
    fn default() -> Self {
        Self::new()
    }
}
