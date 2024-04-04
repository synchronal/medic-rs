use medic_init::cli::CliArgs;
use medic_init::create_config_file;
use medic_src::AppResult;

use clap::Parser;

fn main() -> AppResult<()> {
  let cli_args = CliArgs::parse();

  create_config_file(cli_args.config, cli_args.force)
}
