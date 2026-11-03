// @related [subject](medic-run/src/cli/app.rs)

use super::CliArgs;
use clap::{CommandFactory, Parser};
use medic_src::theme::Theme;

/// Nextest runs each test in its own process, so mutating env is safe here.
fn try_parse(args: &[&str]) -> Result<CliArgs, clap::Error> {
  unsafe { std::env::remove_var("MEDIC_THEME") };
  CliArgs::try_parse_from(std::iter::once("medic-run").chain(args.iter().copied()))
}

#[test]
fn valid() {
  CliArgs::command().debug_assert();
}

#[test]
fn bin_name() {
  assert_eq!(CliArgs::command().get_bin_name(), Some("medic run"));
}

#[test]
fn needs_name() {
  assert!(try_parse(&["--cmd", "true"]).is_err());
}

#[test]
fn needs_cmd() {
  assert!(try_parse(&["--name", "thing"]).is_err());
}

#[test]
fn defaults() {
  let args = try_parse(&["--name", "thing", "--cmd", "true"]).unwrap();

  assert_eq!(args.name, "thing");
  assert_eq!(args.cmd, "true");
  assert!(args.cd.is_none());
  assert!(args.remedy.is_none());
  assert!(!args.verbose);
  assert!(args.completion.is_none());
  assert!(matches!(args.theme, Theme::Auto));
}

#[test]
fn long_flags() {
  let args = try_parse(&[
    "--name",
    "thing",
    "--cmd",
    "make",
    "--cd",
    "sub",
    "--remedy",
    "fix it",
    "--verbose",
    "--theme",
    "light",
  ])
  .unwrap();

  assert_eq!(args.name, "thing");
  assert_eq!(args.cmd, "make");
  assert_eq!(args.cd.as_deref(), Some("sub"));
  assert_eq!(args.remedy.as_deref(), Some("fix it"));
  assert!(args.verbose);
  assert!(matches!(args.theme, Theme::Light));
}

#[test]
fn short_flags() {
  let args = try_parse(&["-n", "thing", "-c", "make", "-r", "fix it", "-t", "dark"]).unwrap();

  assert_eq!(args.name, "thing");
  assert_eq!(args.cmd, "make");
  assert_eq!(args.remedy.as_deref(), Some("fix it"));
  assert!(matches!(args.theme, Theme::Dark));
}

#[test]
fn env() {
  unsafe { std::env::set_var("MEDIC_THEME", "light") };
  let args = CliArgs::try_parse_from(["medic-run", "-n", "thing", "-c", "true"]).unwrap();

  assert!(matches!(args.theme, Theme::Light));
}
