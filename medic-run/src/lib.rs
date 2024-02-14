#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_trait_v2)]

pub mod cli;

use medic_src::runnable::Runnable;
use medic_src::shell::ShellConfig;
use medic_src::AppResult;

pub fn run_shell(
    name: String,
    cmd: String,
    cd: Option<String>,
    remedy: Option<String>,
    verbose: bool,
    progress: &mut retrogress::ProgressBar,
) -> AppResult<()> {
    let shell = ShellConfig::new(name, cmd, cd, remedy, verbose);
    eprintln!();
    shell.run(progress)?;
    AppResult::Ok(())
}
