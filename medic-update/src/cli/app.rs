use clap::Parser;
use clap_complete::Shell;
use medic_src::cli::Flags;
use medic_src::theme::Theme;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic update")]
pub struct CliArgs {
  /// Path to a file where medic config can be found
  #[clap(short, long, env = "MEDIC_CONFIG", default_value = "${PWD}/.config/medic.toml")]
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

  /// Run sub-lists of steps in parallel (experimental)
  #[arg(short, long, env = "MEDIC_PARALLEL", action)]
  pub parallel: bool,

  /// Color theme
  #[arg(short, long, env = "MEDIC_THEME", default_value = "auto")]
  pub theme: Theme,
}

impl From<CliArgs> for Flags {
  fn from(args: CliArgs) -> Self {
    Self {
      auto_apply_remedy: args.apply_remedies,
      config_path: args.config,
      interactive: args.interactive,
      parallel: args.parallel,
      ..Self::default()
    }
  }
}
