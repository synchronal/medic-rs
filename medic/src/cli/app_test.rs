// @related [subject](medic/src/cli/app.rs)

use super::*;
use clap::CommandFactory;
use clap::error::ErrorKind;
use medic_src::theme::Theme;

// Clap falls back to these env vars; clear them for deterministic parsing.
// Nextest runs each test in its own process, so mutating env is safe here.
fn parse(args: &[&str]) -> Result<CliArgs, clap::Error> {
  unsafe {
    for var in [
      "MEDIC_CONFIG",
      "MEDIC_APPLY_REMEDIES",
      "MEDIC_INTERACTIVE",
      "MEDIC_PARALLEL",
      "MEDIC_THEME",
    ] {
      std::env::remove_var(var);
    }
  }
  CliArgs::try_parse_from(std::iter::once("medic").chain(args.iter().copied()))
}

fn manifest_args(args: &[&str]) -> ManifestArgs {
  match parse(args).expect("parse failed").command {
    Command::Audit(a)
    | Command::Doctor(a)
    | Command::Outdated(a)
    | Command::Test(a)
    | Command::Update(a)
    | Command::Shipit(a) => a,
    other => panic!("expected manifest command, got {other:?}"),
  }
}

#[test]
fn valid() {
  CliArgs::command().debug_assert();
}

#[test]
fn audit() {
  assert!(matches!(parse(&["audit"]).unwrap().command, Command::Audit(_)));
}

#[test]
fn doctor() {
  assert!(matches!(parse(&["doctor"]).unwrap().command, Command::Doctor(_)));
}

#[test]
fn outdated() {
  assert!(matches!(parse(&["outdated"]).unwrap().command, Command::Outdated(_)));
}

#[test]
fn test() {
  assert!(matches!(parse(&["test"]).unwrap().command, Command::Test(_)));
}

#[test]
fn update() {
  assert!(matches!(parse(&["update"]).unwrap().command, Command::Update(_)));
}

#[test]
fn shipit() {
  assert!(matches!(parse(&["shipit"]).unwrap().command, Command::Shipit(_)));
}

#[test]
fn init() {
  match parse(&["init"]).unwrap().command {
    Command::Init(args) => {
      assert_eq!(args.config, PathBuf::from("${PWD}/.config/medic.toml"));
      assert!(!args.force);
    }
    other => panic!("expected Init, got {other:?}"),
  }
}

#[test]
fn init_flags() {
  match parse(&["init", "--force", "-c", "custom.toml"])
    .unwrap()
    .command
  {
    Command::Init(args) => {
      assert_eq!(args.config, PathBuf::from("custom.toml"));
      assert!(args.force);
    }
    other => panic!("expected Init, got {other:?}"),
  }
}

#[test]
fn run() {
  match parse(&[
    "run",
    "--name",
    "Build",
    "--cmd",
    "make",
    "--cd",
    "sub",
    "-r",
    "retry",
    "--verbose",
  ])
  .unwrap()
  .command
  {
    Command::Run(args) => {
      assert_eq!(args.name, "Build");
      assert_eq!(args.cmd, "make");
      assert_eq!(args.cd.as_deref(), Some("sub"));
      assert_eq!(args.remedy.as_deref(), Some("retry"));
      assert!(args.verbose);
    }
    other => panic!("expected Run, got {other:?}"),
  }
}

#[test]
fn run_missing_args() {
  let err = parse(&["run"]).unwrap_err();
  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn unknown() {
  let err = parse(&["bogus"]).unwrap_err();
  assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn missing() {
  assert!(parse(&[]).is_err());
}

#[test]
fn prefix() {
  assert!(matches!(parse(&["doc"]).unwrap().command, Command::Doctor(_)));
  assert!(matches!(parse(&["sh"]).unwrap().command, Command::Shipit(_)));
}

#[test]
fn bad_prefix() {
  let err = parse(&["x"]).unwrap_err();
  assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn defaults() {
  let args = manifest_args(&["doctor"]);
  assert_eq!(args.config, PathBuf::from("${PWD}/.config/medic.toml"));
  assert!(!args.apply_remedies);
  assert!(!args.interactive);
  assert!(!args.parallel);
  assert!(matches!(args.theme, Theme::Auto));
}

#[test]
fn short_flags() {
  let args = manifest_args(&["audit", "-c", "m.toml", "-a", "-i", "-p", "-t", "dark"]);
  assert_eq!(args.config, PathBuf::from("m.toml"));
  assert!(args.apply_remedies);
  assert!(args.interactive);
  assert!(args.parallel);
  assert!(matches!(args.theme, Theme::Dark));
}

#[test]
fn long_flags() {
  let args = manifest_args(&[
    "test",
    "--config",
    "m.toml",
    "--apply-remedies",
    "--interactive",
    "--parallel",
    "--theme",
    "light",
  ]);
  assert_eq!(args.config, PathBuf::from("m.toml"));
  assert!(args.apply_remedies);
  assert!(args.interactive);
  assert!(args.parallel);
  assert!(matches!(args.theme, Theme::Light));
}

#[test]
fn bad_theme() {
  let err = parse(&["doctor", "--theme", "neon"]).unwrap_err();
  assert_eq!(err.kind(), ErrorKind::InvalidValue);
}

#[test]
fn env() {
  unsafe {
    std::env::set_var("MEDIC_CONFIG", "env.toml");
    std::env::set_var("MEDIC_INTERACTIVE", "true");
    std::env::set_var("MEDIC_THEME", "light");
  }
  let args = CliArgs::try_parse_from(["medic", "update"]).unwrap();
  match args.command {
    Command::Update(a) => {
      assert_eq!(a.config, PathBuf::from("env.toml"));
      assert!(a.interactive);
      assert!(matches!(a.theme, Theme::Light));
    }
    other => panic!("expected Update, got {other:?}"),
  }
}

#[test]
fn into_flags() {
  let flags: Flags = manifest_args(&["shipit", "-c", "m.toml", "-a", "-i", "-p"]).into();
  assert!(flags.auto_apply_remedy);
  assert_eq!(flags.config_path, PathBuf::from("m.toml"));
  assert!(flags.interactive);
  assert!(flags.parallel);
}

#[test]
fn recoverable_default() {
  let flags: Flags = manifest_args(&["doctor"]).into();
  assert_eq!(flags.recoverable, Flags::default().recoverable);
  assert!(!flags.auto_apply_remedy);
  assert!(!flags.interactive);
  assert!(!flags.parallel);
}
