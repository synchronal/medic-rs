// @related [subject](medic-src/src/outdated/summary.rs)

use super::summary::*;
use console::style;
use indoc::{formatdoc, indoc};

#[test]
fn deserialize_empty() {
  let string = r#""#;
  let outdated: OutdatedSummary = string.parse::<OutdatedSummary>().unwrap();
  assert_eq!(outdated.remedy, None);
  assert_eq!(outdated.deps, vec![]);
}

#[test]
fn deserialize_with_deps() {
  let string = indoc! {r#"
    ::outdated::name=my-dep::version=1.2.3::latest=2.3.4
    ::outdated::name=other-dep::version=1.2.3::latest=2.3.4::parent=my-dep
    "#};

  let outdated: OutdatedSummary = string.parse::<OutdatedSummary>().unwrap();
  assert_eq!(outdated.remedy, None);

  let deps = outdated.deps;

  assert_eq!(deps[0].name, "my-dep".to_string());
  assert_eq!(deps[0].version, "1.2.3".to_string());
  assert_eq!(deps[0].latest, "2.3.4".to_string());
  assert_eq!(deps[0].parent, None);

  assert_eq!(deps[1].name, "other-dep".to_string());
  assert_eq!(deps[1].version, "1.2.3".to_string());
  assert_eq!(deps[1].latest, "2.3.4".to_string());
  assert_eq!(deps[1].parent, Some("my-dep".to_string()));
}

#[test]
fn outdated_to_string_long_name() {
  let summary = OutdatedSummary {
    deps: vec![
      OutdatedDep::new("my-dep", "0.1.2", "1.2.3", None),
      OutdatedDep::new("my-other-long-dep", "1.1.2", "5.4.3", None),
    ],
    remedy: None,
    max_name_length: 17,
    max_latest_length: 6,
    max_version_length: 7,
  };

  let summary_string = format!("{summary}");
  assert_eq!(
    &summary_string,
    &formatdoc! {"
          {}{}               {}  {}  {}
          {}my-dep             0.1.2    1.2.3
          {}my-other-long-dep  1.1.2    5.4.3
        ",
        "    ",
        format!("{}", style("Name").bold().underlined()),
        style("Version").bold().underlined(),
        style("Latest").bold().underlined(),
        style("Parent").bold().underlined(),
        "    ",
        "    ",
    }
  );
}

#[test]
fn outdated_to_string_long_version() {
  let summary = OutdatedSummary {
    deps: vec![
      OutdatedDep::new("dep", "0.1.2", "1.2.3", None),
      OutdatedDep::new("other", "1.1.2-rc.1.2.3", "5.4.3", None),
    ],
    remedy: None,
    max_name_length: 5,
    max_latest_length: 6,
    max_version_length: 14,
  };

  let summary_string = format!("{summary}");
  assert_eq!(
    &summary_string,
    &formatdoc! {"
          {}{}   {}         {}  {}
          {}dep    0.1.2           1.2.3
          {}other  1.1.2-rc.1.2.3  5.4.3
        ",
        "    ",
        format!("{}", style("Name").bold().underlined()),
        style("Version").bold().underlined(),
        style("Latest").bold().underlined(),
        style("Parent").bold().underlined(),
        "    ",
        "    ",
    }
  );
}

#[test]
fn outdated_to_string_long_latest() {
  let summary = OutdatedSummary {
    deps: vec![
      OutdatedDep::new("dep", "0.1.2", "1.2.3", Some("other")),
      OutdatedDep::new("other", "1.1.2", "5.4.3.2.1.3.4.5", None),
    ],
    remedy: None,
    max_name_length: 5,
    max_latest_length: 15,
    max_version_length: 7,
  };

  let summary_string = format!("{summary}");
  assert_eq!(
    &summary_string,
    &formatdoc! {"
          {}{}   {}  {}           {}
          {}dep    0.1.2    1.2.3            other
          {}other  1.1.2    5.4.3.2.1.3.4.5
        ",
        "    ",
        format!("{}", style("Name").bold().underlined()),
        style("Version").bold().underlined(),
        style("Latest").bold().underlined(),
        style("Parent").bold().underlined(),
        "    ",
        "    ",
    }
  );
}

#[test]
fn outdated_to_string_parent() {
  let summary = OutdatedSummary {
    deps: vec![
      OutdatedDep::new("my-dep", "0.1.2", "1.2.3", None),
      OutdatedDep::new("other-dep", "1.1.2", "5.4.3", Some("my-dep")),
    ],
    remedy: None,
    max_name_length: 9,
    max_latest_length: 6,
    max_version_length: 7,
  };

  let summary_string = format!("{summary}");
  assert_eq!(
    &summary_string,
    &formatdoc! {"
          {}{}       {}  {}  {}
          {}my-dep     0.1.2    1.2.3
          {}other-dep  1.1.2    5.4.3   my-dep
        ",
        "    ",
        format!("{}", style("Name").bold().underlined()),
        style("Version").bold().underlined(),
        style("Latest").bold().underlined(),
        style("Parent").bold().underlined(),
        "    ",
        "    ",
    }
  );
}

#[test]
fn deserialize_with_remedy() {
  let string = r#"
    ::outdated::name=my-dep::version=1.2.3::latest=2.3.4
    ::remedy::update deps
    "#;

  let outdated: OutdatedSummary = string.parse::<OutdatedSummary>().unwrap();
  assert_eq!(outdated.remedy, Some("update deps".to_string()));
}

#[test]
fn deserialize_dep() {
  let string = r#"name=my-dep::version=1.2.3::latest=2.3.4"#;

  let dep: OutdatedDep = string.parse::<OutdatedDep>().unwrap();
  assert_eq!(dep.name, "my-dep".to_string());
  assert_eq!(dep.version, "1.2.3".to_string());
  assert_eq!(dep.latest, "2.3.4".to_string());
  assert_eq!(dep.parent, None);
}

#[test]
fn deserialize_dep_remedy() {
  let string = r#"name=my-dep::version=1.2.3::latest=2.3.4::parent=update deps"#;

  let dep: OutdatedDep = string.parse::<OutdatedDep>().unwrap();
  assert_eq!(dep.name, "my-dep".to_string());
  assert_eq!(dep.version, "1.2.3".to_string());
  assert_eq!(dep.latest, "2.3.4".to_string());
  assert_eq!(dep.parent, Some("update deps".to_string()));
}
