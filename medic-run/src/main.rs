use clap::{CommandFactory, Parser};
use clap_complete::generate;
use console::Term;
use medic_run::cli::CliArgs;
use medic_src::{theme, AppResult};
use std::io::stdout;
use std::panic;

fn main() -> AppResult<()> {
  let cli_args = CliArgs::parse();
  theme::set_theme((&cli_args.theme).into());

  if let Some(completion) = cli_args.completion {
    let mut cmd = CliArgs::command();
    let name = cmd.get_name().to_string();
    generate(completion, &mut cmd, name, &mut stdout());

    std::process::exit(0);
  }

  ctrlc::set_handler(interrupt).expect("Unable to set Ctrl-C handler");

  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);
  let _ = Term::stderr().hide_cursor();
  let _ = Term::stdout().hide_cursor();

  let result = panic::catch_unwind(|| {
    let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());
    medic_run::run_shell(
      cli_args.name,
      cli_args.cmd,
      cli_args.cd,
      cli_args.remedy,
      cli_args.verbose,
      &mut progress,
    )
  });

  reset_term();

  match result {
    Ok(inner) => inner,
    Err(_) => std::process::exit(1),
  }
}

fn interrupt() {
  reset_term();
  std::process::exit(1);
}

fn reset_term() {
  let _ = Term::stderr().show_cursor();
  let _ = Term::stdout().show_cursor();
}
