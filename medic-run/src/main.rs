use clap::{CommandFactory, Parser};
use clap_complete::generate;
use medic_run::cli::CliArgs;
use medic_src::AppResult;
use std::io::stdout;

fn main() -> AppResult<()> {
    let cli_args = CliArgs::parse();

    if let Some(completion) = cli_args.completion {
        let mut cmd = CliArgs::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());
    medic_run::run_shell(
        cli_args.name,
        cli_args.cmd,
        cli_args.remedy,
        cli_args.verbose,
        &mut progress,
    )
}
