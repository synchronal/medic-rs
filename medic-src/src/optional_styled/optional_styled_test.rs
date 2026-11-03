// @related [subject](medic-src/src/optional_styled/mod.rs)

use super::*;

fn red() -> Style {
  Style::new().force_styling(true).red()
}

#[test]
fn styled() {
  assert_eq!(
    OptionalStyled::new("text", red()).to_string(),
    "\u{1b}[31mtext\u{1b}[0m"
  );
}

#[test]
fn styled_empty() {
  assert_eq!(OptionalStyled::new("", red()).to_string(), "\u{1b}[31m\u{1b}[0m");
}

#[test]
fn unset() {
  assert_eq!(OptionalStyled::with_style(red()).to_string(), "");
}

#[test]
fn unset_prefixed() {
  assert_eq!(OptionalStyled::with_style(red()).prefixed(" ").to_string(), "");
}

#[test]
fn push() {
  let mut styled = OptionalStyled::with_style(red());

  styled.push('(');
  styled.push(')');

  assert_eq!(styled.to_string(), "\u{1b}[31m()\u{1b}[0m");
}

#[test]
fn push_str() {
  let mut styled = OptionalStyled::with_style(red());

  styled.push_str("ab");
  styled.push_str("cd");

  assert_eq!(styled.to_string(), "\u{1b}[31mabcd\u{1b}[0m");
}

#[test]
fn prefix() {
  let mut styled = OptionalStyled::with_style(red()).prefixed(" ");

  styled.push_str("dir");

  assert_eq!(styled.to_string(), " \u{1b}[31mdir\u{1b}[0m");
}

#[test]
fn prefixed_new() {
  assert_eq!(
    OptionalStyled::new("x", red()).prefixed("> ").to_string(),
    "> \u{1b}[31mx\u{1b}[0m"
  );
}
