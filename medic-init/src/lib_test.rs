// @related [subject](medic-init/src/lib.rs)

use super::*;
use medic_src::config::Manifest;
use std::path::PathBuf;

fn tmp_dir(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("medic-init-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

#[test]
fn creates_dirs() {
  let dir = tmp_dir("create");
  let path = dir.join("nested/.config/medic.toml");

  assert!(matches!(create_config_file(path.clone(), false), AppResult::Ok(())));
  assert!(path.is_file());
}

#[test]
fn valid_manifest() {
  let dir = tmp_dir("valid");
  let path = dir.join("medic.toml");

  assert!(matches!(create_config_file(path.clone(), false), AppResult::Ok(())));

  let manifest = match Manifest::new(&path) {
    AppResult::Ok(manifest) => manifest,
    _ => panic!("generated config should parse"),
  };
  assert!(manifest.audit.unwrap().checks.is_empty());
  assert!(manifest.doctor.unwrap().checks.is_empty());
  assert!(manifest.outdated.unwrap().checks.is_empty());
  assert!(manifest.test.unwrap().checks.is_empty());
  assert_eq!(manifest.update.unwrap().steps.len(), 2);
  assert_eq!(manifest.shipit.unwrap().steps.len(), 4);
}

#[test]
fn no_overwrite() {
  let dir = tmp_dir("exists");
  let path = dir.join("medic.toml");
  std::fs::write(&path, "original").unwrap();

  match create_config_file(path.clone(), false) {
    AppResult::Err(Some(err)) => assert!(err.to_string().contains("already exists"), "{err}"),
    _ => panic!("expected an error when the file exists"),
  }
  assert_eq!(std::fs::read_to_string(&path).unwrap(), "original");
}

#[test]
fn force() {
  let dir = tmp_dir("force");
  let path = dir.join("medic.toml");
  std::fs::write(&path, "original").unwrap();

  assert!(matches!(create_config_file(path.clone(), true), AppResult::Ok(())));
  let contents = std::fs::read_to_string(&path).unwrap();
  assert!(contents.starts_with("[doctor]\n"), "{contents}");
}

#[test]
fn dir_path() {
  let dir = tmp_dir("directory");
  let path = dir.join("medic.toml");
  std::fs::create_dir_all(&path).unwrap();

  assert!(matches!(create_config_file(path, true), AppResult::Err(_)));
}

#[test]
fn envsubst() {
  let dir = tmp_dir("envsubst");
  // Nextest runs each test in its own process, so mutating env is safe here.
  unsafe { std::env::set_var("MEDIC_INIT_TEST_DIR", &dir) };

  let path = PathBuf::from("${MEDIC_INIT_TEST_DIR}/medic.toml");
  assert!(matches!(create_config_file(path, false), AppResult::Ok(())));
  assert!(dir.join("medic.toml").is_file());
}
