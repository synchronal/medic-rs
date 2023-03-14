#![cfg_attr(feature = "strict", deny(warnings))]

mod cli;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::error;
use std::io::stdout;

use crate::cli::app::CliArgs;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> AppResult<()> {
    let cli_args = CliArgs::parse();

    if let Some(completion) = cli_args.completion {
        let mut cmd = CliArgs::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    Ok(())
}
