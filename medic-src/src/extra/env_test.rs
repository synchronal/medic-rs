// @related [subject](medic-src/src/extra/env.rs)

use super::env::*;

#[test]
fn cwd() {
  let cwd = std::env::current_dir().unwrap();

  assert_eq!(
    subst("${CWD}/medic.toml").unwrap(),
    format!("{}/medic.toml", cwd.display())
  );
}

#[test]
fn env() {
  unsafe {
    std::env::set_var("MEDIC_ENV_TEST_VAR", "value");
  }

  assert_eq!(subst("a-${MEDIC_ENV_TEST_VAR}-b").unwrap(), "a-value-b");
}

#[test]
fn unknown_var() {
  assert_eq!(
    subst("${MEDIC_ENV_TEST_DOES_NOT_EXIST}").unwrap(),
    "${MEDIC_ENV_TEST_DOES_NOT_EXIST}"
  );
}

#[test]
fn unbraced_var() {
  unsafe {
    std::env::set_var("MEDIC_ENV_TEST_UNBRACED", "value");
  }

  assert_eq!(subst("$MEDIC_ENV_TEST_UNBRACED").unwrap(), "$MEDIC_ENV_TEST_UNBRACED");
}

#[test]
fn skips_invalid_values() {
  unsafe {
    std::env::set_var("MEDIC_ENV_TEST_DOLLAR", "has $dollar");
    std::env::set_var("MEDIC_ENV_TEST_BRACE", "has {brace}");
  }

  assert_eq!(
    subst("${MEDIC_ENV_TEST_DOLLAR} ${MEDIC_ENV_TEST_BRACE}").unwrap(),
    "${MEDIC_ENV_TEST_DOLLAR} ${MEDIC_ENV_TEST_BRACE}"
  );
}

#[test]
fn plain() {
  assert_eq!(subst("no variables here").unwrap(), "no variables here");
}
