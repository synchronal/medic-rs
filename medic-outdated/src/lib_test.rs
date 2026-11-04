// @related [subject](medic-outdated/src/lib.rs)

use super::*;
use std::path::{Path, PathBuf};

fn initialize_theme() {
  medic_src::theme::THEME.get_or_init(medic_src::theme::dark_theme);
}

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-outdated-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

fn manifest(dir: &Path, contents: &str) -> Manifest {
  let path = dir.join("medic.toml");
  std::fs::write(&path, contents).unwrap();
  // Manifest::new sets MEDIC_CONFIG; scope it so CLI tests in this binary don't see it.
  match temp_env::with_var_unset("MEDIC_CONFIG", || Manifest::new(&path)) {
    AppResult::Ok(manifest) => manifest,
    _ => panic!("unable to parse manifest"),
  }
}

/// Writes an executable `medic-outdated-{name}` into `dir` that prints `stdout`.
fn fake_outdated_check(dir: &Path, name: &str, stdout: &str) {
  use std::os::unix::fs::PermissionsExt;
  let path = dir.join(format!("medic-outdated-{name}"));
  std::fs::write(&path, format!("#!/usr/bin/env bash\nprintf '{stdout}'\n")).unwrap();
  std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

fn run_with(manifest: Manifest) -> AppResult<()> {
  initialize_theme();
  let flags = Flags::default();
  let mut progress = medic_src::progress::new(&flags);
  run_checks(manifest, &mut progress, flags, &Context::new())
}

/// Runs with `dir` prepended to PATH, restoring PATH afterwards. Goes through
/// `temp_env` so it can't race other env-touching tests under `cargo test`.
fn run_with_path(dir: &Path, manifest: Manifest) -> AppResult<()> {
  let path = format!("{}:{}", dir.display(), std::env::var("PATH").unwrap_or_default());
  temp_env::with_var("PATH", Some(path), || run_with(manifest))
}

#[test]
fn no_section() {
  let dir = tmp_dir("missing");
  let manifest = manifest(&dir, "[audit]\nchecks = []\n");

  match run_with(manifest) {
    AppResult::Err(Some(err)) => assert_eq!(err.to_string(), "No outdated checks found in medic config."),
    _ => panic!("expected an error for a missing outdated section"),
  }
}

#[test]
fn empty() {
  let dir = tmp_dir("empty");
  let manifest = manifest(&dir, "[outdated]\nchecks = []\n");

  assert!(matches!(run_with(manifest), AppResult::Ok(())));
}

#[test]
fn up_to_date() {
  let dir = tmp_dir("current");
  fake_outdated_check(&dir, "current", "");
  let manifest = manifest(&dir, "[outdated]\nchecks = [\n  { check = \"current\" },\n]\n");

  assert!(matches!(run_with_path(&dir, manifest), AppResult::Ok(())));
}

#[test]
fn outdated_ok() {
  let dir = tmp_dir("outdated");
  fake_outdated_check(
    &dir,
    "stale",
    "::outdated::name=thingy::version=1.2.3::latest=1.2.4::\\n",
  );
  let manifest = manifest(&dir, "[outdated]\nchecks = [\n  { check = \"stale\" },\n]\n");

  assert!(matches!(run_with_path(&dir, manifest), AppResult::Ok(())));
}

#[test]
fn missing_binary() {
  let dir = tmp_dir("not-found");
  let manifest = manifest(
    &dir,
    "[outdated]\nchecks = [\n  { check = \"does-not-exist-anywhere\" },\n]\n",
  );

  assert!(matches!(run_with(manifest), AppResult::Err(_)));
}

#[test]
fn stops_on_failure() {
  let dir = tmp_dir("failure");
  let marker = dir.join("marker");
  let after = dir.join("medic-outdated-after");
  std::fs::write(&after, format!("#!/usr/bin/env bash\ntouch {}\n", marker.display())).unwrap();
  {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&after, std::fs::Permissions::from_mode(0o755)).unwrap();
  }
  let manifest = manifest(
    &dir,
    "[outdated]\nchecks = [\n  { check = \"does-not-exist-anywhere\" },\n  { check = \"after\" },\n]\n",
  );

  assert!(matches!(run_with_path(&dir, manifest), AppResult::Err(_)));
  assert!(!marker.exists(), "checks after a failure should not run");
}
