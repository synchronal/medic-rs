use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-homebrew")]
/// Ensures that Homebrew dependencies declared in a Brewfile
/// are installed and up to date.
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
