// @related [tests](medic-src/src/outdated/summary_test.rs)

use std::collections::HashMap;

use console::{pad_str, style, Alignment};

#[derive(Debug, Eq, PartialEq)]
pub struct OutdatedSummary {
  pub deps: Vec<OutdatedDep>,
  pub remedy: Option<String>,
  pub(crate) max_name_length: usize,
  pub(crate) max_latest_length: usize,
  pub(crate) max_version_length: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct OutdatedDep {
  pub latest: String,
  pub name: String,
  pub parent: Option<String>,
  pub version: String,
}

#[derive(Debug)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for ParseError {}

impl std::str::FromStr for OutdatedSummary {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut deps: Vec<OutdatedDep> = vec![];
    let mut remedy: Option<String> = None;
    let mut max_name_length = 4;
    let mut max_latest_length = 6;
    let mut max_version_length = 7;

    for line in s.lines() {
      let mut split = line.split("::");
      split.next();
      match split.next() {
        Some("remedy") => {
          remedy = Some(split.next().unwrap().to_owned());
        }
        Some("outdated") => {
          let outdated = split.collect::<Vec<&str>>().join("::");
          let dep = outdated.parse::<OutdatedDep>()?;

          if dep.name.len() > max_name_length {
            max_name_length = dep.name.len();
          }
          if dep.latest.len() > max_latest_length {
            max_latest_length = dep.latest.len();
          }
          if dep.version.len() > max_version_length {
            max_version_length = dep.version.len();
          }
          deps.push(dep);
        }
        Some(_) => {}
        None => {}
      }
    }

    Ok(Self {
      deps,
      remedy,
      max_name_length,
      max_latest_length,
      max_version_length,
    })
  }
}

impl OutdatedDep {
  pub fn new(name: &str, version: &str, latest: &str, parent: Option<&str>) -> Self {
    Self {
      latest: latest.into(),
      name: name.into(),
      parent: parent.map(|s| s.into()),
      version: version.into(),
    }
  }
}

impl std::str::FromStr for OutdatedDep {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut inputs: HashMap<&str, &str> = HashMap::with_capacity(4);

    for mut value_pair in s.trim().split("::") {
      value_pair = value_pair.trim();

      if value_pair.is_empty() {
        continue;
      }

      match value_pair.split_once('=') {
        Some((key, value)) => inputs.insert(key, value),
        None => {
          return Err(ParseError(format!(
            "Expected key=value\r\nFound: {value_pair}\r\nOutdated: {s}"
          )));
        }
      };
    }

    let mut keys: Vec<&&str> = inputs.keys().collect();
    keys.sort();
    if keys == vec![&"latest", &"name", &"version"] {
      Ok(Self::new(
        inputs.get("name").unwrap(),
        inputs.get("version").unwrap(),
        inputs.get("latest").unwrap(),
        None,
      ))
    } else if keys == vec![&"latest", &"name", &"parent", &"version"] {
      Ok(Self::new(
        inputs.get("name").unwrap(),
        inputs.get("version").unwrap(),
        inputs.get("latest").unwrap(),
        Some(inputs.get("parent").unwrap()),
      ))
    } else {
      Err(ParseError(format!(
        "Expected name, version, latest, <parent>, found: {keys:?}"
      )))
    }
  }
}

impl std::fmt::Display for OutdatedSummary {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "    ")?;
    write!(f, "{}", style("Name").bold().underlined())?;
    if self.max_name_length > 4 {
      for _space in 4..self.max_name_length {
        write!(f, " ")?;
      }
    }
    write!(f, "  ")?;
    write!(f, "{}", style("Version").bold().underlined())?;
    if self.max_version_length > 7 {
      for _space in 7..self.max_version_length {
        write!(f, " ")?;
      }
    }
    write!(f, "  ")?;
    write!(f, "{}", style("Latest").bold().underlined())?;
    if self.max_latest_length > 6 {
      for _space in 6..self.max_latest_length {
        write!(f, " ")?;
      }
    }
    write!(f, "  ")?;
    write!(f, "{}", style("Parent").bold().underlined())?;
    writeln!(f)?;
    for dep in &self.deps {
      write!(f, "    ")?;
      write!(f, "{}", pad_str(&dep.name, self.max_name_length, Alignment::Left, None))?;
      write!(f, "  ")?;
      write!(
        f,
        "{}",
        pad_str(&dep.version, self.max_version_length, Alignment::Left, None)
      )?;
      write!(f, "  ")?;
      write!(f, "{}", &dep.latest)?;
      if dep.parent.is_some() {
        if dep.latest.len() < self.max_latest_length {
          write!(
            f,
            "{}",
            pad_str("", self.max_latest_length - dep.latest.len(), Alignment::Left, None)
          )?;
        }
        write!(f, "  {}", dep.parent.clone().unwrap())?;
      }
      writeln!(f)?;
    }
    write!(f, "")
  }
}
