use clap::Parser;
use clap_complete::Shell;
use medic_src::cli::Flags;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic shipit")]
pub struct CliArgs {
  /// Path to a file where medic config can be found
  #[arg(short, long, env = "MEDIC_CONFIG", default_value = "${PWD}/.config/medic.toml", value_hint = clap::ValueHint::FilePath)]
  pub config: std::path::PathBuf,

  /// Shell to generate completions for
  #[clap(long, value_enum, value_parser)]
  pub completion: Option<Shell>,

  /// Automatically apply suggested remedies
  #[arg(short, long, env = "MEDIC_APPLY_REMEDIES", action)]
  pub apply_remedies: bool,

  /// Provide interactive prompts when possible instead of failing
  #[arg(short, long, env = "MEDIC_INTERACTIVE", action)]
  pub interactive: bool,
}

impl From<CliArgs> for Flags {
  fn from(args: CliArgs) -> Self {
    Self {
      auto_apply_remedy: args.apply_remedies,
      interactive: args.interactive,
    }
  }
}
