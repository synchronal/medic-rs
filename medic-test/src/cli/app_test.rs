// @related [subject](medic-test/src/cli/app.rs)

use super::CliArgs;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use medic_src::cli::Flags;
use medic_src::theme::Theme;
use std::path::PathBuf;

fn parse(args: &[&str]) -> CliArgs {
  clear_env();
  CliArgs::try_parse_from(std::iter::once("medic-test").chain(args.iter().copied())).unwrap()
}

/// Nextest runs each test in its own process, so mutating env is safe here.
fn clear_env() {
  for var in [
    "MEDIC_CONFIG",
    "MEDIC_APPLY_REMEDIES",
    "MEDIC_INTERACTIVE",
    "MEDIC_PARALLEL",
    "MEDIC_THEME",
  ] {
    unsafe { std::env::remove_var(var) };
  }
}

#[test]
fn valid() {
  CliArgs::command().debug_assert();
}

#[test]
fn bin_name() {
  assert_eq!(CliArgs::command().get_bin_name(), Some("medic test"));
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
  clear_env();
  unsafe {
    std::env::set_var("MEDIC_CONFIG", "/env/medic.toml");
    std::env::set_var("MEDIC_APPLY_REMEDIES", "true");
    std::env::set_var("MEDIC_INTERACTIVE", "true");
    std::env::set_var("MEDIC_PARALLEL", "true");
    std::env::set_var("MEDIC_THEME", "light");
  }
  let args = CliArgs::try_parse_from(["medic-test"]).unwrap();

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
  clear_env();
  assert!(CliArgs::try_parse_from(["medic-test", "--theme", "neon"]).is_err());
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
