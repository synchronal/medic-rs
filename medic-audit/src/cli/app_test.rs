// @related [subject](medic-audit/src/cli/app.rs)

use super::CliArgs;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use medic_src::cli::Flags;
use medic_src::theme::Theme;
use std::path::PathBuf;

/// Clap falls back to these env vars. Tests run as threads sharing one
/// process env under `cargo test`, so every read or write goes through
/// `temp_env`, which serializes access and restores values afterwards.
const ENV_VARS: [&str; 5] = [
  "MEDIC_CONFIG",
  "MEDIC_APPLY_REMEDIES",
  "MEDIC_INTERACTIVE",
  "MEDIC_PARALLEL",
  "MEDIC_THEME",
];

fn try_parse(args: &[&str]) -> Result<CliArgs, clap::Error> {
  temp_env::with_vars_unset(ENV_VARS, || {
    CliArgs::try_parse_from(std::iter::once("medic-audit").chain(args.iter().copied()))
  })
}

fn parse(args: &[&str]) -> CliArgs {
  try_parse(args).unwrap()
}

#[test]
fn valid() {
  CliArgs::command().debug_assert();
}

#[test]
fn bin_name() {
  assert_eq!(CliArgs::command().get_bin_name(), Some("medic audit"));
}

#[test]
fn defaults() {
  let args = parse(&[]);

  assert_eq!(args.config, PathBuf::from("${PWD}/.config/medic.toml"));
  assert!(args.completion.is_none());
  assert!(!args.apply_remedies);
  assert!(!args.interactive);
  assert!(!args.parallel);
  assert!(matches!(args.theme, Theme::Auto));
}

#[test]
fn long_flags() {
  let args = parse(&[
    "--config",
    "/tmp/medic.toml",
    "--apply-remedies",
    "--interactive",
    "--parallel",
    "--theme",
    "light",
  ]);

  assert_eq!(args.config, PathBuf::from("/tmp/medic.toml"));
  assert!(args.apply_remedies);
  assert!(args.interactive);
  assert!(args.parallel);
  assert!(matches!(args.theme, Theme::Light));
}

#[test]
fn short_flags() {
  let args = parse(&["-c", "/tmp/medic.toml", "-a", "-i", "-p", "-t", "dark"]);

  assert_eq!(args.config, PathBuf::from("/tmp/medic.toml"));
  assert!(args.apply_remedies);
  assert!(args.interactive);
  assert!(args.parallel);
  assert!(matches!(args.theme, Theme::Dark));
}

#[test]
fn env() {
  let args = temp_env::with_vars(
    [
      ("MEDIC_CONFIG", Some("/env/medic.toml")),
      ("MEDIC_APPLY_REMEDIES", Some("true")),
      ("MEDIC_INTERACTIVE", Some("true")),
      ("MEDIC_PARALLEL", Some("true")),
      ("MEDIC_THEME", Some("light")),
    ],
    || CliArgs::try_parse_from(["medic-audit"]).unwrap(),
  );

  assert_eq!(args.config, PathBuf::from("/env/medic.toml"));
  assert!(args.apply_remedies);
  assert!(args.interactive);
  assert!(args.parallel);
  assert!(matches!(args.theme, Theme::Light));
}

#[test]
fn completion() {
  let args = parse(&["--completion", "zsh"]);

  assert_eq!(args.completion, Some(Shell::Zsh));
}

#[test]
fn bad_theme() {
  assert!(try_parse(&["--theme", "neon"]).is_err());
}

#[test]
fn into_flags() {
  let flags: Flags = parse(&["-c", "/tmp/medic.toml", "-a", "-i", "-p"]).into();

  assert!(flags.auto_apply_remedy);
  assert_eq!(flags.config_path, PathBuf::from("/tmp/medic.toml"));
  assert!(flags.interactive);
  assert!(flags.parallel);
  assert!(flags.recoverable);
}

#[test]
fn default_flags() {
  let flags: Flags = parse(&[]).into();

  assert!(!flags.auto_apply_remedy);
  assert_eq!(flags.config_path, PathBuf::from("${PWD}/.config/medic.toml"));
  assert!(!flags.interactive);
  assert!(!flags.parallel);
  assert!(flags.recoverable);
}
