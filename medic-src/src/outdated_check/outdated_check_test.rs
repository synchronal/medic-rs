// @related [subject](medic-src/src/outdated_check/mod.rs)

use super::*;

#[test]
fn test_deserialize_string() {
    let toml = r#"
        check = "outdated-name"
        "#;

    let result: OutdatedCheck = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        OutdatedCheck {
            args: None,
            cd: None,
            check: "outdated-name".to_string(),
            name: None,
            remedy: None,
        }
    )
}

#[test]
fn test_deserialize_args_string() {
    let toml = r#"
        args = { argument = "value" }
        check = "outdated-name"
        "#;

    let result: OutdatedCheck = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        OutdatedCheck {
            args: Some(BTreeMap::from([(
                "argument".to_string(),
                StringOrList(vec!["value".to_string()])
            )])),
            cd: None,
            check: "outdated-name".to_string(),
            name: None,
            remedy: None,
        }
    )
}

#[test]
fn test_deserialize_args_value_lsit_string() {
    let toml = r#"
        args = { argument = ["value 1", "value 2"] }
        check = "outdated-name"
        "#;

    let result: OutdatedCheck = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        OutdatedCheck {
            args: Some(BTreeMap::from([(
                "argument".to_string(),
                StringOrList(vec!["value 1".to_string(), "value 2".to_string()])
            )])),
            cd: None,
            check: "outdated-name".to_string(),
            name: None,
            remedy: None,
        }
    )
}

#[test]
fn test_deserialize_cd_string() {
    let toml = r#"
        cd = "./subdirectory"
        check = "outdated-name"
        "#;

    let result: OutdatedCheck = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        OutdatedCheck {
            args: None,
            cd: Some("./subdirectory".to_string()),
            check: "outdated-name".to_string(),
            name: None,
            remedy: None,
        }
    )
}

#[test]
fn test_deserialize_name_string() {
    let toml = r#"
        check = "outdated-name"
        name = "Check for outdated things"
        "#;

    let result: OutdatedCheck = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        OutdatedCheck {
            args: None,
            cd: None,
            check: "outdated-name".to_string(),
            name: Some("Check for outdated things".to_string()),
            remedy: None,
        }
    )
}

#[test]
fn test_to_string() {
    let check = OutdatedCheck {
        args: None,
        cd: None,
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    assert_eq!(format!("{check}"), "\u{1b}[36mthing\u{1b}[0m")
}

#[test]
fn test_to_string_name() {
    let check = OutdatedCheck {
        args: None,
        cd: None,
        check: "thing".to_string(),
        name: Some("do things".to_string()),
        remedy: None,
    };

    assert_eq!(format!("{check}"), "\u{1b}[36mdo things\u{1b}[0m")
}

#[test]
fn test_to_string_cd() {
    let check = OutdatedCheck {
        args: None,
        cd: Some("../subdirectory".to_string()),
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36mthing\u{1b}[0m \u{1b}[32m(../subdirectory)\u{1b}[0m"
    )
}
