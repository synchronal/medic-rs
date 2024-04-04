use console::{style, Style};

pub struct OptionalStyled {
  content: String,
  prefix: String,
  some: bool,
  style: Style,
}

impl OptionalStyled {
  pub fn with_style(style: Style) -> Self {
    Self {
      content: "".into(),
      prefix: "".into(),
      some: false,
      style,
    }
  }

  pub fn prefixed(mut self, prefix: &str) -> Self {
    self.prefix = prefix.into();
    self
  }

  pub fn push(&mut self, ch: char) {
    self.some = true;
    self.content.push(ch);
  }

  pub fn push_str(&mut self, string: &str) {
    self.some = true;
    self.content.push_str(string);
  }
}

impl std::fmt::Display for OptionalStyled {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.some {
      write!(f, "{}{}", self.prefix, self.style.apply_to(style(&self.content)))
    } else {
      write!(f, "")
    }
  }
}
