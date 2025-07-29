use console::Style;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use terminal_colorsaurus::{color_palette, QueryOptions, ThemeMode};

pub static THEME: OnceCell<ColorTheme> = OnceCell::new();
pub fn current_theme() -> &'static ColorTheme {
  THEME.get().expect("ColorTheme not set")
}
pub fn set_theme(theme: ColorTheme) {
  THEME.set(theme).expect("Unable to set ColorTheme");
}

#[derive(clap:: ValueEnum, Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
  #[default]
  Auto,
  Dark,
  Light,
}

impl From<&Theme> for ColorTheme {
  fn from(theme: &Theme) -> Self {
    match theme {
      Theme::Auto => detect_colortheme().unwrap_or(dark_theme()),
      Theme::Dark => dark_theme(),
      Theme::Light => light_theme(),
    }
  }
}

pub fn detect_colortheme() -> Result<ColorTheme, Box<dyn std::error::Error>> {
  let colors = color_palette(QueryOptions::default())?;
  match colors.theme_mode() {
    ThemeMode::Dark => Ok(dark_theme()),
    ThemeMode::Light => Ok(light_theme()),
  }
}

#[derive(Debug)]
pub struct ColorTheme {
  pub args_style: Style,
  pub cd_style: Style,
  pub dim_style: Style,
  pub error_style: Style,
  pub highlight_style: Style,
  pub success_style: Style,
  pub text_style: Style,
  pub warning_style: Style,
}

pub fn dark_theme() -> ColorTheme {
  ColorTheme {
    args_style: Style::new().force_styling(true).yellow(),
    cd_style: Style::new().force_styling(true).green(),
    dim_style: Style::new().force_styling(true).cyan().green().bold(),
    error_style: Style::new().force_styling(true).red(),
    highlight_style: Style::new().force_styling(true).cyan().bright().bold(),
    success_style: Style::new().force_styling(true).green(),
    text_style: Style::new().force_styling(true).cyan(),
    warning_style: Style::new().force_styling(true).yellow(),
  }
}

pub fn light_theme() -> ColorTheme {
  ColorTheme {
    args_style: Style::new().force_styling(true).magenta(),
    cd_style: Style::new().force_styling(true).green(),
    dim_style: Style::new().force_styling(true).cyan().green().bold(),
    error_style: Style::new().force_styling(true).red().bold(),
    highlight_style: Style::new().force_styling(true).black().bold(),
    success_style: Style::new().force_styling(true).green(),
    text_style: Style::new().force_styling(true).blue(),
    warning_style: Style::new().force_styling(true).yellow(),
  }
}
