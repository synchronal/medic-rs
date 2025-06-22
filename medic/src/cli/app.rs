use clap::Args;
use clap::Parser;
use clap::Subcommand;
use medic_run::cli::CliArgs as RunArgs;
use medic_src::cli::Flags;
use medic_src::theme::Theme;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic")]
/// Run medic workflow management commands.
pub struct CliArgs {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Debug, Subcommand)]
#[clap(infer_subcommands = true)]
pub enum Command {
  /// Runs checks to audit the application. Checks might include linters
  /// and dep audits.
  Audit(ManifestArgs),
  /// Runs checks to ensure that a project is fully set up for development.
  Doctor(ManifestArgs),
  /// Creates the shell of a medic manifest file.
  Init(InitArgs),
  /// Runs checks for outdated dependencies
  Outdated(ManifestArgs),
  /// Runs an arbitrary shell command.
  Run(RunArgs),
  /// Runs an application's tests.
  Test(ManifestArgs),
  /// Update the current application
  Update(ManifestArgs),
  /// Ship changes. Typically configured to audit, update, test, then release.
  Shipit(ManifestArgs),
}

#[derive(Args, Debug)]
pub struct ManifestArgs {
  /// Path to a file where medic config can be found
  #[clap(value_parser)]
  #[arg(short, long, env = "MEDIC_CONFIG", default_value = "${PWD}/.config/medic.toml", value_hint = clap::ValueHint::FilePath)]
  pub config: PathBuf,

  /// Automatically apply suggested remedies
  #[arg(short, long, env = "MEDIC_APPLY_REMEDIES", action)]
  pub apply_remedies: bool,

  /// Provide interactive prompts when possible instead of failing
  #[arg(short, long, env = "MEDIC_INTERACTIVE", action)]
  pub interactive: bool,

  /// Color theme
  #[arg(short, long, env = "MEDIC_THEME", default_value = "auto")]
  pub theme: Theme,
}

#[derive(Args, Debug)]
pub struct InitArgs {
  /// Path to a file where medic config can be found
  #[clap(value_parser)]
  #[arg(short, long, env = "MEDIC_CONFIG", default_value = "${PWD}/.config/medic.toml", value_hint = clap::ValueHint::FilePath)]
  pub config: PathBuf,
  #[clap(short, long, action)]
  pub force: bool,
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

impl From<ManifestArgs> for Flags {
  fn from(args: ManifestArgs) -> Self {
    Self {
      auto_apply_remedy: args.apply_remedies,
      interactive: args.interactive,
      ..Self::default()
    }
  }
}
