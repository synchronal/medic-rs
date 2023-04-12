#![cfg_attr(feature = "strict", deny(warnings))]

use medic_doctor::cli::CliArgs;
use medic_doctor::config::Manifest;
use medic_doctor::AppResult;

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

    println!("manifest: {:?}", manifest);

    Ok(())
}
