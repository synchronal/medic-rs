// @related [subject](medic-src/src/runnable/mod.rs)

use super::*;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::path::PathBuf;
use std::sync::Once;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static INIT: Once = Once::new();
pub fn initialize() {
  INIT.call_once(|| {
    if crate::theme::THEME.get().is_none() {
      let theme = crate::theme::dark_theme();
      crate::theme::set_theme(theme);
    }
  });
}

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-runnable-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

#[derive(Clone)]
enum Outcome {
  Ok,
  Err,
  ErrWithRemedy(String),
  Manual(String),
  ManualWithoutRemedy,
  Nonrecoverable(&'static str),
  Optional,
  OptionalWithRemedy(String),
  Quit,
}

impl Outcome {
  fn into_recoverable(self) -> Recoverable<()> {
    let remedy = |cmd: String| Some(Remedy::new(cmd, None, BTreeMap::new()));
    match self {
      Outcome::Ok => Recoverable::Ok(()),
      Outcome::Err => Recoverable::Err(Some("failed".into()), None),
      Outcome::ErrWithRemedy(cmd) => Recoverable::Err(Some("failed".into()), remedy(cmd)),
      Outcome::Manual(cmd) => Recoverable::Manual(Some("manual".into()), remedy(cmd)),
      Outcome::ManualWithoutRemedy => Recoverable::Manual(Some("manual".into()), None),
      Outcome::Nonrecoverable(msg) => Recoverable::Nonrecoverable(msg.into()),
      Outcome::Optional => Recoverable::Optional((), None),
      Outcome::OptionalWithRemedy(cmd) => Recoverable::Optional((), remedy(cmd)),
      Outcome::Quit => Recoverable::Quit,
    }
  }
}

/// A Runnable that returns a scripted sequence of outcomes, one per call to
/// `run`, and counts how many times it was run.
#[derive(Clone)]
struct FakeRunnable {
  calls: Arc<AtomicUsize>,
  outcomes: Arc<Mutex<VecDeque<Outcome>>>,
  platform: Option<Vec<String>>,
}

impl FakeRunnable {
  fn new(outcomes: Vec<Outcome>) -> Self {
    Self {
      calls: Arc::new(AtomicUsize::new(0)),
      outcomes: Arc::new(Mutex::new(outcomes.into())),
      platform: None,
    }
  }

  fn calls(&self) -> usize {
    self.calls.load(Ordering::SeqCst)
  }
}

impl fmt::Display for FakeRunnable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "fake runnable")
  }
}

impl Runnable for FakeRunnable {
  fn platform(&self) -> &Option<Vec<String>> {
    &self.platform
  }

  fn run(&self, _progress: &mut ProgressBar, _flags: &mut Flags, _context: &Context) -> Recoverable<()> {
    self.calls.fetch_add(1, Ordering::SeqCst);
    self
      .outcomes
      .lock()
      .unwrap()
      .pop_front()
      .expect("FakeRunnable ran more times than scripted")
      .into_recoverable()
  }

  fn to_command(&self) -> Result<std::process::Command, MedicError> {
    Err("not a command".into())
  }
}

fn run_fake(runnable: &FakeRunnable, flags: &mut Flags) -> AppResult<()> {
  initialize();
  let mut progress = crate::progress::new(flags);
  run(runnable.clone(), &mut progress, flags, &Context::new())
}

fn prompt_result(input: &str) -> PromptResult {
  Ok::<String, retrogress::Error>(input.to_string()).into()
}

// // // Runnable trait defaults

#[test]
fn defaults() {
  #[derive(Clone)]
  struct Bare;
  impl fmt::Display for Bare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "bare")
    }
  }
  impl Runnable for Bare {
    fn run(&self, _: &mut ProgressBar, _: &mut Flags, _: &Context) -> Recoverable<()> {
      Recoverable::Ok(())
    }
    fn to_command(&self) -> Result<std::process::Command, MedicError> {
      Err("no".into())
    }
  }
  assert!(!Bare.allow_failure());
  assert!(Bare.platform().is_none());
  assert!(!Bare.verbose());
}

// // // run

#[test]
fn skips_other_platform() {
  let mut runnable = FakeRunnable::new(vec![]);
  runnable.platform = Some(vec!["NotARealPlatform".into()]);

  let result = run_fake(&runnable, &mut Flags::default());

  assert!(matches!(result, AppResult::Ok(())));
  assert_eq!(runnable.calls(), 0);
}

#[test]
fn runs_matching_platform() {
  let mut runnable = FakeRunnable::new(vec![Outcome::Ok]);
  runnable.platform = Some(vec![Context::new().platform]);

  let result = run_fake(&runnable, &mut Flags::default());

  assert!(matches!(result, AppResult::Ok(())));
  assert_eq!(runnable.calls(), 1);
}

#[test]
fn ok() {
  let runnable = FakeRunnable::new(vec![Outcome::Ok]);

  assert!(matches!(run_fake(&runnable, &mut Flags::default()), AppResult::Ok(())));
  assert_eq!(runnable.calls(), 1);
}

#[test]
fn interactive_env() {
  let runnable = FakeRunnable::new(vec![Outcome::Ok]);
  let mut flags = Flags {
    interactive: true,
    ..Flags::default()
  };

  run_fake(&runnable, &mut flags);

  assert_eq!(std::env::var("MEDIC_INTERACTIVE").as_deref(), Ok("true"));
}

#[test]
fn auto_apply_env() {
  let runnable = FakeRunnable::new(vec![Outcome::Ok]);
  let mut flags = Flags {
    auto_apply_remedy: true,
    ..Flags::default()
  };

  run_fake(&runnable, &mut flags);

  assert_eq!(std::env::var("MEDIC_APPLY_REMEDIES").as_deref(), Ok("true"));
}

#[test]
fn err() {
  let runnable = FakeRunnable::new(vec![Outcome::Err]);

  let result = run_fake(&runnable, &mut Flags::default());

  match result {
    AppResult::Err(Some(err)) => assert_eq!(err.to_string(), "failed"),
    _ => panic!("expected AppResult::Err(Some(_))"),
  }
  assert_eq!(runnable.calls(), 1);
}

#[test]
fn err_skips_remedy() {
  let dir = tmp_dir("not-applied");
  let marker = dir.join("marker");
  let runnable = FakeRunnable::new(vec![Outcome::ErrWithRemedy(format!("touch {}", marker.display()))]);

  let result = run_fake(&runnable, &mut Flags::default());

  assert!(matches!(result, AppResult::Err(Some(_))));
  assert!(!marker.exists(), "remedy must not run without auto-apply");
  assert_eq!(runnable.calls(), 1);
}

#[test]
fn auto_apply_reruns() {
  let dir = tmp_dir("auto-apply-success");
  let marker = dir.join("marker");
  let runnable = FakeRunnable::new(vec![
    Outcome::ErrWithRemedy(format!("touch {}", marker.display())),
    Outcome::Ok,
  ]);
  let mut flags = Flags {
    auto_apply_remedy: true,
    ..Flags::default()
  };

  let result = run_fake(&runnable, &mut flags);

  assert!(matches!(result, AppResult::Ok(())));
  assert!(marker.exists(), "remedy should have been applied");
  assert_eq!(runnable.calls(), 2);
}

#[test]
fn auto_apply_fails_again() {
  let dir = tmp_dir("auto-apply-fails-again");
  let marker = dir.join("marker");
  let remedy = format!("echo applied >> {}", marker.display());
  let runnable = FakeRunnable::new(vec![
    Outcome::ErrWithRemedy(remedy.clone()),
    Outcome::ErrWithRemedy(remedy),
  ]);
  let mut flags = Flags {
    auto_apply_remedy: true,
    ..Flags::default()
  };

  let result = run_fake(&runnable, &mut flags);

  assert!(matches!(result, AppResult::Err(Some(_))));
  assert_eq!(runnable.calls(), 2);
  assert_eq!(
    std::fs::read_to_string(&marker).unwrap(),
    "applied\n",
    "remedy should be applied exactly once"
  );
}

#[test]
fn auto_apply_resets_rerun() {
  let dir = tmp_dir("auto-apply-reset");
  let marker = dir.join("marker");
  let remedy = format!("echo applied >> {}", marker.display());
  let mut flags = Flags {
    auto_apply_remedy: true,
    ..Flags::default()
  };

  let first = FakeRunnable::new(vec![Outcome::ErrWithRemedy(remedy.clone()), Outcome::Ok]);
  assert!(matches!(run_fake(&first, &mut flags), AppResult::Ok(())));

  let second = FakeRunnable::new(vec![Outcome::ErrWithRemedy(remedy), Outcome::Ok]);
  assert!(matches!(run_fake(&second, &mut flags), AppResult::Ok(())));

  assert_eq!(std::fs::read_to_string(&marker).unwrap(), "applied\napplied\n");
}

#[test]
fn manual_quits() {
  let runnable = FakeRunnable::new(vec![Outcome::Manual("do it yourself".into())]);

  assert!(matches!(run_fake(&runnable, &mut Flags::default()), AppResult::Quit));
}

#[test]
#[ignore = "bug: runnable::run unwraps the remedy of Recoverable::Manual, which ShellConfig produces as None when manual = true and no remedy is set"]
fn manual_without_remedy() {
  let runnable = FakeRunnable::new(vec![Outcome::ManualWithoutRemedy]);

  assert!(matches!(run_fake(&runnable, &mut Flags::default()), AppResult::Quit));
}

#[test]
fn nonrecoverable() {
  let runnable = FakeRunnable::new(vec![Outcome::Nonrecoverable("boom")]);
  let mut flags = Flags {
    interactive: true,
    ..Flags::default()
  };

  match run_fake(&runnable, &mut flags) {
    AppResult::Err(Some(err)) => assert_eq!(err.to_string(), "boom"),
    _ => panic!("expected AppResult::Err(Some(_))"),
  }
}

#[test]
fn optional() {
  let runnable = FakeRunnable::new(vec![Outcome::Optional]);

  assert!(matches!(run_fake(&runnable, &mut Flags::default()), AppResult::Ok(())));
}

#[test]
fn optional_skips_remedy() {
  let dir = tmp_dir("optional-remedy");
  let marker = dir.join("marker");
  let runnable = FakeRunnable::new(vec![Outcome::OptionalWithRemedy(format!("touch {}", marker.display()))]);

  assert!(matches!(run_fake(&runnable, &mut Flags::default()), AppResult::Ok(())));
  assert!(!marker.exists(), "optional remedy must not run when not interactive");
}

#[test]
fn quit() {
  let runnable = FakeRunnable::new(vec![Outcome::Quit]);

  assert!(matches!(run_fake(&runnable, &mut Flags::default()), AppResult::Quit));
}

// // // run_remedy

#[test]
fn remedy_cd_and_env() {
  initialize();
  let dir = tmp_dir("run-remedy");
  let remedy = Remedy::new(
    "echo \"$MEDIC_APPLY_REMEDIES $EXTRA\" > out".into(),
    Some(dir.to_string_lossy().into_owned()),
    BTreeMap::from([("EXTRA".to_string(), "extra".to_string())]),
  );
  let mut progress = crate::progress::new(&Flags::default());

  let result = run_remedy(remedy, &mut progress);

  assert!(matches!(result, AppResult::Ok(())));
  assert_eq!(std::fs::read_to_string(dir.join("out")).unwrap(), "true extra\n");
}

// // // PromptResult

#[test]
fn prompt_answers() {
  for (input, expected) in [
    ("a", "All"),
    ("A", "All"),
    ("n", "No"),
    ("N", "No"),
    ("q", "Quit"),
    ("Q", "Quit"),
    ("r", "Rerun"),
    ("R", "Rerun"),
    ("s", "Skip"),
    ("y", "Yes"),
    ("Y", "Yes"),
    ("?", "Help"),
  ] {
    let actual = match prompt_result(input) {
      PromptResult::All => "All",
      PromptResult::Help => "Help",
      PromptResult::No => "No",
      PromptResult::Quit => "Quit",
      PromptResult::Rerun => "Rerun",
      PromptResult::Skip => "Skip",
      PromptResult::Unknown => "Unknown",
      PromptResult::Yes => "Yes",
      PromptResult::Err(_) => "Err",
    };
    assert_eq!(actual, expected, "input {input:?}");
  }
}

#[test]
#[ignore = "bug: uppercase S is not accepted as skip, unlike every other prompt answer"]
fn prompt_uppercase_skip() {
  assert!(matches!(prompt_result("S"), PromptResult::Skip));
}

#[test]
fn prompt_unknown() {
  for input in ["", "yes", "x", " y"] {
    assert!(matches!(prompt_result(input), PromptResult::Unknown), "input {input:?}");
  }
}

#[test]
fn missing_remedy_cd() {
  initialize();
  let missing = std::env::temp_dir()
    .join(format!("medic-runnable-{}-missing", std::process::id()))
    .display()
    .to_string();
  let remedy = Remedy::new("true".into(), Some(missing.clone()), BTreeMap::new());
  let mut progress = crate::progress::new(&Flags::default());

  match run_remedy(remedy, &mut progress) {
    AppResult::Err(Some(err)) => assert_eq!(err.to_string(), format!("directory {missing} does not exist")),
    _ => panic!("expected AppResult::Err(Some(_))"),
  }
}
