// @related [subject](medic-test/src/lib.rs)

use super::*;
use std::path::{Path, PathBuf};

fn initialize_theme() {
  if medic_src::theme::THEME.get().is_none() {
    medic_src::theme::set_theme(medic_src::theme::dark_theme());
  }
}

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-test-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

fn manifest(dir: &Path, contents: &str) -> Manifest {
  let path = dir.join("medic.toml");
  std::fs::write(&path, contents).unwrap();
  match Manifest::new(&path) {
    AppResult::Ok(manifest) => manifest,
    _ => panic!("unable to parse manifest"),
  }
}

fn run_with(manifest: Manifest) -> AppResult<()> {
  initialize_theme();
  let flags = Flags::default();
  let mut progress = medic_src::progress::new(&flags);
  run_steps(manifest, &mut progress, flags, &Context::new())
}

#[test]
fn no_section() {
  let dir = tmp_dir("missing");
  let manifest = manifest(&dir, "[audit]\nchecks = []\n");

  match run_with(manifest) {
    AppResult::Err(Some(err)) => assert_eq!(err.to_string(), "No test checks found in medic config."),
    _ => panic!("expected an error for a missing test section"),
  }
}

#[test]
fn interactive_no_prompt() {
  initialize_theme();
  let dir = tmp_dir("interactive");
  let manifest = manifest(
    &dir,
    "[test]\nchecks = [\n  { name = \"fails\", shell = \"false\" },\n]\n",
  );
  let flags = Flags {
    interactive: true,
    ..Flags::default()
  };
  let mut progress = medic_src::progress::new(&flags);

  assert!(matches!(
    run_steps(manifest, &mut progress, flags, &Context::new()),
    AppResult::Err(_)
  ));
}

#[test]
fn empty() {
  let dir = tmp_dir("empty");
  let manifest = manifest(&dir, "[test]\nchecks = []\n");

  assert!(matches!(run_with(manifest), AppResult::Ok(())));
}

#[test]
fn in_order() {
  let dir = tmp_dir("success");
  let log = dir.join("log");
  let manifest = manifest(
    &dir,
    &format!(
      "[test]\nchecks = [\n  {{ name = \"one\", shell = \"echo one >> {log}\" }},\n  {{ name = \"two\", shell = \"echo two >> {log}\" }},\n]\n",
      log = log.display()
    ),
  );

  assert!(matches!(run_with(manifest), AppResult::Ok(())));
  assert_eq!(std::fs::read_to_string(&log).unwrap(), "one\ntwo\n");
}

#[test]
fn stops_on_failure() {
  let dir = tmp_dir("failure");
  let marker = dir.join("marker");
  let manifest = manifest(
    &dir,
    &format!(
      "[test]\nchecks = [\n  {{ name = \"fails\", shell = \"false\" }},\n  {{ name = \"after\", shell = \"touch {}\" }},\n]\n",
      marker.display()
    ),
  );

  assert!(matches!(run_with(manifest), AppResult::Err(_)));
  assert!(!marker.exists(), "checks after a failure should not run");
}
