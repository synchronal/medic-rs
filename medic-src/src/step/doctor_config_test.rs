// @related [subject](medic-src/src/step/mod.rs)

use super::*;
use std::path::{Path, PathBuf};
use std::sync::Once;

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
  let dir = std::env::temp_dir().join(format!("medic-doctor-config-{}-{name}", std::process::id()));
  let _ = std::fs::remove_dir_all(&dir);
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

fn write_manifest(dir: &Path, checks: &str) -> PathBuf {
  let path = dir.join("medic.toml");
  std::fs::write(&path, format!("[doctor]\nchecks = [\n{checks}\n]\n")).unwrap();
  path
}

fn run_doctor(flags: &mut Flags) -> Recoverable<()> {
  initialize();
  let mut progress = crate::progress::new(flags);
  let config = DoctorConfig { doctor: NoopConfig {} };
  config.run(&mut progress, flags, &Context::new())
}

#[test]
fn run_succeeds_when_all_checks_pass() {
  let dir = tmp_dir("success");
  let manifest = write_manifest(&dir, r#"{ name = "passes", shell = "true" },"#);

  let mut flags = Flags {
    config_path: manifest,
    ..Flags::default()
  };

  assert!(matches!(run_doctor(&mut flags), Recoverable::Ok(())));
}

#[test]
fn run_stops_on_failed_check_when_not_interactive() {
  let dir = tmp_dir("failure");
  let second = dir.join("second");
  let manifest = write_manifest(
    &dir,
    &format!(
      r#"{{ name = "fails", shell = "false" }},
{{ name = "never runs", shell = "touch {}" }},"#,
      second.display()
    ),
  );

  let mut flags = Flags {
    config_path: manifest,
    ..Flags::default()
  };

  assert!(matches!(run_doctor(&mut flags), Recoverable::Nonrecoverable(_)));
  assert!(!second.exists(), "doctor continued after a failed check");
}

#[test]
fn run_stops_when_check_fails_after_auto_applied_remedy() {
  let dir = tmp_dir("remedy");
  let remedied = dir.join("remedied");
  let second = dir.join("second");
  let manifest = write_manifest(
    &dir,
    &format!(
      r#"{{ name = "fails", shell = "false", remedy = "touch {}" }},
{{ name = "never runs", shell = "touch {}" }},"#,
      remedied.display(),
      second.display()
    ),
  );

  let mut flags = Flags {
    auto_apply_remedy: true,
    config_path: manifest,
    ..Flags::default()
  };

  assert!(matches!(run_doctor(&mut flags), Recoverable::Nonrecoverable(_)));
  assert!(remedied.exists(), "remedy was not applied");
  assert!(!second.exists(), "doctor continued after remedy failed to fix check");
}
