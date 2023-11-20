use medic_shipit::cli::CliArgs;
use medic_shipit::run_steps;
use medic_src::config::Manifest;
use medic_src::AppResult;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io::stdout;

fn main() -> AppResult<()> {
    let cli_args = CliArgs::parse();

    if let Some(completion) = cli_args.completion {
        let mut cmd = CliArgs::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    let manifest = Manifest::new(cli_args.config)?;
    let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());

    run_steps(manifest, &mut progress)
}
