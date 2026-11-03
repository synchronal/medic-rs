// @related [subject](medic-run/src/lib.rs)

use super::*;
use std::path::PathBuf;

fn initialize_theme() {
  if medic_src::theme::THEME.get().is_none() {
    medic_src::theme::set_theme(medic_src::theme::dark_theme());
  }
}

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-run-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

fn run(cmd: String, cd: Option<String>) -> AppResult<()> {
  initialize_theme();
  let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());
  run_shell("test".into(), cmd, cd, None, false, &mut progress)
}

#[test]
fn ok() {
  let dir = tmp_dir("success");
  let marker = dir.join("marker");

  assert!(matches!(
    run(format!("touch {}", marker.display()), None),
    AppResult::Ok(())
  ));
  assert!(marker.exists());
}

#[test]
fn fails() {
  assert!(matches!(run("false".into(), None), AppResult::Err(_)));
}

#[test]
fn fails_with_remedy() {
  initialize_theme();
  let mut progress = retrogress::ProgressBar::new(retrogress::Sync::boxed());

  let result = run_shell(
    "test".into(),
    "false".into(),
    None,
    Some("echo fix".into()),
    false,
    &mut progress,
  );
  assert!(matches!(result, AppResult::Err(_)));
}

#[test]
fn cd() {
  let dir = tmp_dir("cd");
  let out = dir.join("pwd");

  assert!(matches!(
    run(format!("pwd > {}", out.display()), Some(dir.display().to_string())),
    AppResult::Ok(())
  ));
  let pwd = std::fs::read_to_string(&out).unwrap();
  assert_eq!(
    std::fs::canonicalize(pwd.trim()).unwrap(),
    std::fs::canonicalize(&dir).unwrap()
  );
}

#[test]
fn missing_cd() {
  let dir = tmp_dir("missing-cd").join("nope");

  assert!(matches!(
    run("true".into(), Some(dir.display().to_string())),
    AppResult::Err(_)
  ));
}
