// @related [subject](medic-init/src/cli/app.rs)

use super::CliArgs;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use std::path::PathBuf;

/// Clap falls back to MEDIC_CONFIG. Tests run as threads sharing one process
/// env under `cargo test`, so every read or write goes through `temp_env`,
/// which serializes access and restores values afterwards.
fn parse(args: &[&str]) -> CliArgs {
  temp_env::with_var_unset("MEDIC_CONFIG", || {
    CliArgs::try_parse_from(std::iter::once("medic-init").chain(args.iter().copied())).unwrap()
  })
}

#[test]
fn valid() {
  CliArgs::command().debug_assert();
}

#[test]
fn bin_name() {
  assert_eq!(CliArgs::command().get_bin_name(), Some("medic init"));
}

#[test]
fn defaults() {
  let args = parse(&[]);

  assert_eq!(args.config, PathBuf::from("${PWD}/.config/medic.toml"));
  assert!(!args.force);
  assert!(args.completion.is_none());
}

#[test]
fn long_flags() {
  let args = parse(&["--config", "/tmp/medic.toml", "--force", "--completion", "bash"]);

  assert_eq!(args.config, PathBuf::from("/tmp/medic.toml"));
  assert!(args.force);
  assert_eq!(args.completion, Some(Shell::Bash));
}

#[test]
fn short_flags() {
  let args = parse(&["-c", "/tmp/medic.toml", "-f"]);

  assert_eq!(args.config, PathBuf::from("/tmp/medic.toml"));
  assert!(args.force);
}

#[test]
fn env() {
  let args = temp_env::with_var("MEDIC_CONFIG", Some("/env/medic.toml"), || {
    CliArgs::try_parse_from(["medic-init"]).unwrap()
  });

  assert_eq!(args.config, PathBuf::from("/env/medic.toml"));
}
