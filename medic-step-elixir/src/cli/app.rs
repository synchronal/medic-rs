use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, about)]
#[clap(bin_name = "medic-step-cargo")]
/// Steps for validating a Rust cargo project.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Check for known vulnerabilities in deps. Requires that `mix_audit` be
    /// added to a project.
    AuditDeps(MixArgs),
    /// Run lints with credo. Requires that `credo` be added to a project.
    Credo(MixArgs),
    /// Run dialyzer static analysis. Requires that `dialyxir` be added to a project.
    Dialyzer(MixArgs),
    /// Get new dependencies.
    GetDeps(MixArgs),
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
