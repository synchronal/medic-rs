use crate::cli::Flags;
use crate::recoverable::{Recoverable, Remedy};
use crate::AppResult;
use console::{style, Term};
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
  if flags.interactive {
    std::env::set_var("MEDIC_INTERACTIVE", "true");
  }

  match runnable.clone().run(progress) {
    Recoverable::Ok(ok) => AppResult::Ok(ok),
    Recoverable::Err(err, None) => AppResult::Err(err),
    Recoverable::Err(err, Some(remedy)) => {
      if flags.interactive {
        eprintln!();
        ask(runnable, remedy, progress, AppResult::Err(err), flags)
      } else {
        AppResult::Err(err)
      }
    }
    Recoverable::Optional(ok, None) => AppResult::Ok(ok),
    Recoverable::Optional(ok, Some(remedy)) => {
      if flags.interactive {
        eprintln!();
        ask(runnable, remedy, progress, AppResult::Ok(ok), flags)
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
  flags: &Flags,
) -> AppResult<()> {
  match prompt("Apply this remedy") {
    PromptResult::Help => {
      eprintln!(
        r#"
  - y - yes  - apply the remedy.
  - n - no   - do not run this remedy; if the check is optional continue, otherwise exit.
  - s - skip - skip this step, continuing with future checks and steps.
  - q - quit - abort medic with a non-zero exit code.
  - ? - help - print this message.
"#
      );
      ask(runnable, remedy, progress, default_exit, flags)
    }
    PromptResult::No => default_exit,
    PromptResult::Quit => AppResult::Err(Some("aborting".into())),
    PromptResult::Skip => AppResult::Ok(()),
    PromptResult::Unknown => ask(runnable, remedy, progress, default_exit, flags),
    PromptResult::Yes => {
      run_remedy(remedy, progress)?;
      run(runnable, progress, flags)
    }
  }
}

fn prompt(msg: &str) -> PromptResult {
  let prompt = format!(
    "{} {}{}",
    style(msg).force_styling(true).cyan(),
    style("[y,n,s,q,?]").force_styling(true).cyan().bold(),
    style("?").cyan()
  );
  eprint!("— {prompt} ");
  Term::stdout().read_line().unwrap().into()
}

fn run_remedy(remedy: Remedy, progress: &mut retrogress::ProgressBar) -> AppResult<()> {
  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);
  Term::stderr().clear_line().unwrap();

  let mut command = remedy.to_command();

  command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

  let pb = progress.append(&remedy.to_string());
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

  match res {
    Ok(_) => progress.succeeded(pb),
    Err(_) => progress.failed(pb),
  }
  res.into()
}

enum PromptResult {
  Help,
  No,
  Quit,
  Skip,
  Unknown,
  Yes,
}

impl From<String> for PromptResult {
  fn from(value: String) -> Self {
    let str = value.as_str();
    match str {
      "n" | "N" => Self::No,
      "q" | "Q" => Self::Quit,
      "s" | "S" => Self::Skip,
      "y" | "Y" => Self::Yes,
      "?" => Self::Help,
      _ => Self::Unknown,
    }
  }
}
