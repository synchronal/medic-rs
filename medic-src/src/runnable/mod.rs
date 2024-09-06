use crate::cli::Flags;
use crate::recoverable::{Recoverable, Remedy};
use crate::AppResult;
use console::Term;
use retrogress::Progress;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use std::thread;

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
        eprintln!();
        ask(runnable, remedy, progress, AppResult::Err(err))
      } else {
        AppResult::Err(err)
      }
    }
    Recoverable::Optional(ok, None) => AppResult::Ok(ok),
    Recoverable::Optional(ok, Some(remedy)) => {
      if flags.interactive {
        eprintln!();
        ask(runnable, remedy, progress, AppResult::Ok(ok))
      } else {
        AppResult::Ok(ok)
      }
    }
  }
}

fn ask(
  runnable: impl Runnable,
  remedy: Remedy,
  progress: &mut retrogress::ProgressBar,
  default_exit: AppResult<()>,
) -> AppResult<()> {
  match prompt("Apply this remedy") {
    PromptResult::Yes => run_remedy(remedy, progress),
    PromptResult::No => default_exit,
    PromptResult::Quit => AppResult::Err(Some("aborting".into())),
    PromptResult::Unknown => ask(runnable, remedy, progress, default_exit),
  }
}

fn prompt(msg: &str) -> PromptResult {
  eprint!("\x1b[0;94m{msg} [y,n,q]?\x1b[0m ");
  Term::stdout().read_line().unwrap().into()
}

fn run_remedy(remedy: Remedy, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
  Term::stderr().clear_line().unwrap();
  let pb = progress.append(&remedy.to_string());

  let mut command = remedy.to_command();

  command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

  let mut child = command.spawn()?;
  let stderr = child
    .stderr
    .take()
    .ok_or("Error capturing stderr of step.")?;
  let stdout = child
    .stdout
    .take()
    .ok_or("Error capturing stdout of step.")?;

  let mut out_progress = progress.clone();
  let mut err_progress = progress.clone();

  let out_thr = thread::spawn(move || {
    let reader = BufReader::new(stdout);
    reader
      .lines()
      .map_while(Result::ok)
      .for_each(|line| out_progress.println(pb, &line));
  });
  let err_thr = thread::spawn(move || {
    let reader = BufReader::new(stderr);
    reader
      .lines()
      .map_while(Result::ok)
      .for_each(|line| err_progress.println(pb, &line));
  });

  let res = child.wait_with_output();
  out_thr.join().unwrap();
  err_thr.join().unwrap();
  res.into()
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
