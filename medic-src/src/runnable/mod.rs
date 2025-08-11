use crate::cli::Flags;
use crate::context::Context;
use crate::error::MedicError;
use crate::optional_styled::OptionalStyled;
use crate::recoverable::{Recoverable, Remedy};
use crate::theme::current_theme;
use crate::AppResult;
use arboard::Clipboard;
use console::Term;
use retrogress::ProgressBar;
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use std::sync::{LazyLock, Mutex};
use std::thread;

// In auto-apply mode, medic should exit when a check fails after the remedy
// has been applied.
static RERUN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

pub trait Runnable: std::fmt::Display + Clone {
  fn allow_failure(&self) -> bool {
    false
  }

  fn platform(&self) -> &Option<Vec<String>> {
    &None
  }

  fn run(self, progress: &mut ProgressBar, flags: &Flags) -> Recoverable<()>;
  fn to_command(&self) -> Result<std::process::Command, MedicError>;
  fn verbose(&self) -> bool {
    false
  }
}

pub fn run(runnable: impl Runnable, progress: &mut ProgressBar, flags: &mut Flags, context: &Context) -> AppResult<()> {
  if !context.matches_platform(runnable.platform()) {
    let pb = progress.append(&format!(
      "{} {}",
      &runnable.to_string(),
      OptionalStyled::new("(skipped)", current_theme().warning_style.clone())
    ));
    progress.succeeded(pb);
    return AppResult::Ok(());
  }

  if flags.auto_apply_remedy {
    std::env::set_var("MEDIC_APPLY_REMEDIES", "true");
  }
  if flags.interactive {
    std::env::set_var("MEDIC_INTERACTIVE", "true");
  }

  match runnable.clone().run(progress, flags) {
    Recoverable::Ok(ok) => {
      let mut rerun = LazyLock::force(&RERUN).lock().unwrap();
      *rerun = false;
      AppResult::Ok(ok)
    }
    Recoverable::Err(err, None) => {
      if flags.interactive && flags.recoverable {
        eprintln!();
        ask(runnable, None, progress, AppResult::Err(err), flags, context)
      } else {
        AppResult::Err(err)
      }
    }
    Recoverable::Err(err, Some(remedy)) => {
      if flags.auto_apply_remedy {
        let mut rerun = LazyLock::force(&RERUN).lock().unwrap();
        if *rerun {
          eprintln!(
            "{}",
            OptionalStyled::new(
              "! Check failed again after remedy was applied",
              current_theme().error_style.clone()
            )
          );
          return AppResult::Err(err);
        }
        eprintln!(
          "— {} —",
          OptionalStyled::new("Automatically applying remedy", current_theme().warning_style.clone())
        );
        run_remedy(remedy, progress)?;
        *rerun = true;
        drop(rerun);
        return run(runnable, progress, flags, context);
      }

      eprint!(
        "{} {}",
        OptionalStyled::new("Suggested remedy:", current_theme().text_style.clone()),
        OptionalStyled::new(remedy.to_string(), current_theme().warning_style.clone()),
      );

      let _ = Clipboard::new()
        .and_then(|mut clipboard| clipboard.set_text(remedy.to_string()))
        .map(|_| {
          eprintln!(
            "  {}",
            OptionalStyled::new("(it's in the clipboard)", current_theme().dim_style.clone()),
          );
        });

      if flags.interactive {
        eprintln!();
        ask(runnable, Some(remedy), progress, AppResult::Err(err), flags, context)
      } else {
        AppResult::Err(err)
      }
    }
    Recoverable::Manual(_err, remedy) => {
      let remedy = remedy.unwrap().to_string();
      eprint!(
        "{} {}",
        OptionalStyled::new("Manual remedy:", current_theme().text_style.clone()),
        OptionalStyled::new(&remedy, current_theme().warning_style.clone()),
      );
      let _ = Clipboard::new()
        .and_then(|mut clipboard| clipboard.set_text(&remedy))
        .map(|_| {
          eprintln!(
            "  {}",
            OptionalStyled::new("(it's in the clipboard)", current_theme().dim_style.clone()),
          );
        });
      AppResult::Quit
    }
    Recoverable::Optional(ok, None) => {
      eprintln!();
      eprintln!(
        "{}",
        OptionalStyled::new("(continuing)", current_theme().success_style.clone()),
      );
      let mut rerun = LazyLock::force(&RERUN).lock().unwrap();
      *rerun = false;
      AppResult::Ok(ok)
    }
    Recoverable::Optional(ok, Some(remedy)) => {
      if flags.interactive {
        eprintln!();
        ask(runnable, Some(remedy), progress, AppResult::Ok(ok), flags, context)
      } else {
        let mut rerun = LazyLock::force(&RERUN).lock().unwrap();
        *rerun = false;
        AppResult::Ok(ok)
      }
    }
  }
}

fn ask(
  runnable: impl Runnable,
  remedy: Option<Remedy>,
  progress: &mut ProgressBar,
  default_exit: AppResult<()>,
  flags: &mut Flags,
  context: &Context,
) -> AppResult<()> {
  match prompt(&remedy, &default_exit, progress) {
    PromptResult::Help => {
      let mut msg = vec![];
      if remedy.is_some() {
        msg.push("  - a - all   - apply this and all future remedies.");
        msg.push("  - y - yes   - apply the remedy.");
        msg.push("  - n - no    - do not run this remedy; if the check is optional continue, otherwise exit.");
      }
      msg.push("  - r - rerun - re-run the step.");
      msg.push("  - s - skip  - skip this step, continuing with future checks and steps.");
      msg.push("  - q - quit  - abort medic with a non-zero exit code.");
      msg.push("  - ? - help  - print this message.");

      msg.sort();
      eprintln!("\n{}", msg.join("\n"));
      ask(runnable, remedy, progress, default_exit, flags, context)
    }
    PromptResult::All => {
      flags.auto_apply_remedy = true;
      run_remedy(remedy.unwrap(), progress)?;
      run(runnable, progress, flags, context)
    }
    PromptResult::No => default_exit,
    PromptResult::Quit => AppResult::Quit,
    PromptResult::Rerun => run(runnable, progress, flags, context),
    PromptResult::Skip => {
      let mut rerun = LazyLock::force(&RERUN).lock().unwrap();
      *rerun = false;
      AppResult::Ok(())
    }
    PromptResult::Unknown => ask(runnable, remedy, progress, default_exit, flags, context),
    PromptResult::Yes => {
      run_remedy(remedy.unwrap(), progress)?;
      let mut rerun = LazyLock::force(&RERUN).lock().unwrap();
      *rerun = true;
      drop(rerun);
      run(runnable, progress, flags, context)
    }
  }
}

fn prompt(remedy: &Option<Remedy>, result: &AppResult<()>, progress: &mut ProgressBar) -> PromptResult {
  let mut prompt = String::new();

  if let AppResult::Err(Some(err)) = result {
    if err.to_string().trim() != "" {
      prompt.push_str(&format!(
        "{} {}\n",
        OptionalStyled::new("Error:", current_theme().error_style.clone()),
        err
      ));
    }
  }
  let msg = if remedy.is_some() {
    "Apply this remedy"
  } else {
    "The last step encountered a problem"
  };
  let options = if remedy.is_some() {
    "[y,n,a,r,s,q,?]"
  } else {
    "[r,s,q,?]"
  };
  prompt.push_str(&format!(
    "{} {}{} ",
    OptionalStyled::new(msg, current_theme().text_style.clone()),
    OptionalStyled::new(options, current_theme().highlight_style.clone()),
    OptionalStyled::new("?", current_theme().text_style.clone()),
  ));

  progress.prompt(&prompt).into()
}

fn run_remedy(remedy: Remedy, progress: &mut ProgressBar) -> AppResult<()> {
  console::set_colors_enabled(true);
  console::set_colors_enabled_stderr(true);
  Term::stderr().clear_line().unwrap();

  let mut command = remedy.to_command();

  command
    .env("MEDIC_APPLY_REMEDIES", "true")
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
  All,
  Help,
  No,
  Quit,
  Rerun,
  Skip,
  Unknown,
  Yes,
}

impl From<String> for PromptResult {
  fn from(value: String) -> Self {
    let str = value.as_str();
    match str {
      "a" | "A" => Self::All,
      "n" | "N" => Self::No,
      "q" | "Q" => Self::Quit,
      "r" | "R" => Self::Rerun,
      "s" => Self::Skip,
      "y" | "Y" => Self::Yes,
      "?" => Self::Help,
      _ => Self::Unknown,
    }
  }
}
