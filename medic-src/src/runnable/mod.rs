use crate::cli::Flags;
use crate::recoverable::Recoverable;
use crate::AppResult;
use console::Term;

pub trait Runnable: std::fmt::Display + Clone {
  fn allow_failure(&self) -> bool {
    false
  }

  fn run(self, progress: &mut retrogress::ProgressBar) -> Recoverable<()>;
  fn to_command(&self) -> Result<std::process::Command, Box<dyn std::error::Error>>;
  fn verbose(&self) -> bool {
    false
  }
}

pub fn run(runnable: impl Runnable, progress: &mut retrogress::ProgressBar, flags: &Flags) -> AppResult<()> {
  let _ = flags;
  match runnable.clone().run(progress) {
    Recoverable::Ok(ok) => AppResult::Ok(ok),
    Recoverable::Err(err, None) => AppResult::Err(err),
    Recoverable::Err(err, Some(remedy)) => {
      if flags.interactive {
        ask(runnable, remedy, AppResult::Err(err))
      } else {
        AppResult::Err(err)
      }
    }
    Recoverable::Optional(ok, None) => AppResult::Ok(ok),
    Recoverable::Optional(ok, Some(remedy)) => {
      if flags.interactive {
        ask(runnable, remedy, AppResult::Ok(ok))
      } else {
        AppResult::Ok(ok)
      }
    }
  }
}

fn ask(runnable: impl Runnable, remedy: String, default_exit: AppResult<()>) -> AppResult<()> {
  match prompt("Apply this remedy") {
    PromptResult::Yes => todo!(),
    PromptResult::No => default_exit,
    PromptResult::Quit => AppResult::Err(Some("aborting".into())),
    PromptResult::Unknown => ask(runnable, remedy, default_exit),
  }
}

fn prompt(msg: &str) -> PromptResult {
  eprint!("\x1b[0;94m{msg} [y,n,q]?\x1b[0m ");
  Term::stdout().read_line().unwrap().into()
}

enum PromptResult {
  Yes,
  No,
  Quit,
  Unknown,
}

impl From<String> for PromptResult {
  fn from(value: String) -> Self {
    let str = value.as_str();
    match str {
      "y" | "Y" => Self::Yes,
      "n" | "N" => Self::No,
      "q" | "Q" => Self::Quit,
      _ => Self::Unknown,
    }
  }
}
