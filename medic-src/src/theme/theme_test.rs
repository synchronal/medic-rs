// @related [subject](medic-src/src/theme/mod.rs)

use super::*;

#[derive(Deserialize)]
struct Wrapper {
  theme: Theme,
}

fn render(style: &Style) -> String {
  style.apply_to("x").to_string()
}

#[test]
fn display() {
  assert_eq!(Theme::Auto.to_string(), "auto");
  assert_eq!(Theme::Dark.to_string(), "dark");
  assert_eq!(Theme::Light.to_string(), "light");
}

#[test]
fn default_auto() {
  assert!(matches!(Theme::default(), Theme::Auto));
}

#[test]
fn deserialize() {
  assert!(matches!(
    toml::from_str::<Wrapper>(r#"theme = "auto""#)
      .unwrap()
      .theme,
    Theme::Auto
  ));
  assert!(matches!(
    toml::from_str::<Wrapper>(r#"theme = "dark""#)
      .unwrap()
      .theme,
    Theme::Dark
  ));
  assert!(matches!(
    toml::from_str::<Wrapper>(r#"theme = "light""#)
      .unwrap()
      .theme,
    Theme::Light
  ));
  assert!(toml::from_str::<Wrapper>(r#"theme = "Dark""#).is_err());
}

#[test]
fn from_dark() {
  let theme = ColorTheme::from(&Theme::Dark);
  let expected = dark_theme();

  assert_eq!(render(&theme.text_style), render(&expected.text_style));
  assert_eq!(render(&theme.args_style), render(&expected.args_style));
  assert_eq!(render(&theme.highlight_style), render(&expected.highlight_style));
}

#[test]
fn from_light() {
  let theme = ColorTheme::from(&Theme::Light);
  let expected = light_theme();

  assert_eq!(render(&theme.text_style), render(&expected.text_style));
  assert_eq!(render(&theme.args_style), render(&expected.args_style));
  assert_eq!(render(&theme.highlight_style), render(&expected.highlight_style));
}

#[test]
fn dark_differs_from_light() {
  assert_ne!(render(&dark_theme().text_style), render(&light_theme().text_style));
}

#[test]
fn forced_styling() {
  let theme = dark_theme();

  assert_eq!(render(&theme.text_style), "\u{1b}[36mx\u{1b}[0m");
  assert_eq!(render(&theme.error_style), "\u{1b}[31mx\u{1b}[0m");
  assert_eq!(render(&theme.warning_style), "\u{1b}[33mx\u{1b}[0m");
  assert_eq!(render(&light_theme().text_style), "\u{1b}[34mx\u{1b}[0m");
}

#[test]
fn current() {
  THEME.get_or_init(light_theme);

  assert!(std::ptr::eq(current_theme(), THEME.get().unwrap()));
}
