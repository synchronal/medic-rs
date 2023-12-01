use super::outdated::*;

#[test]
fn deserialize_empty() {
    let string = r#""#;
    let outdated: Outdated = Outdated::from_str(string).unwrap();
    assert_eq!(outdated.remedy, None);
    assert_eq!(outdated.deps, vec![]);
}

#[test]
fn deserialize_with_deps() {
    let string = r#"
    ::outdated::name=my-dep::version=1.2.3::latest=2.3.4
    ::outdated::name=other-dep::version=1.2.3::latest=2.3.4::parent=my-dep
    "#;

    let outdated: Outdated = Outdated::from_str(string).unwrap();
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
fn deserialize_with_remedy() {
    let string = r#"
    ::outdated::name=my-dep::version=1.2.3::latest=2.3.4
    ::remedy::update deps
    "#;

    let outdated: Outdated = Outdated::from_str(string).unwrap();
    assert_eq!(outdated.remedy, Some("update deps".to_string()));
}

#[test]
fn deserialize_dep() {
    let string = r#"name=my-dep::version=1.2.3::latest=2.3.4"#;

    let dep: OutdatedDep = OutdatedDep::from_str(string).unwrap();
    assert_eq!(dep.name, "my-dep".to_string());
    assert_eq!(dep.version, "1.2.3".to_string());
    assert_eq!(dep.latest, "2.3.4".to_string());
    assert_eq!(dep.parent, None);
}

#[test]
fn deserialize_dep_remedy() {
    let string = r#"name=my-dep::version=1.2.3::latest=2.3.4::parent=update deps"#;

    let dep: OutdatedDep = OutdatedDep::from_str(string).unwrap();
    assert_eq!(dep.name, "my-dep".to_string());
    assert_eq!(dep.version, "1.2.3".to_string());
    assert_eq!(dep.latest, "2.3.4".to_string());
    assert_eq!(dep.parent, Some("update deps".to_string()));
}
