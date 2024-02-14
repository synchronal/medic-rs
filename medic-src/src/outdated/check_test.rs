// @related [subject](medic-src/src/outdated/check.rs)

use crate::util::StringOrList;

use super::*;
use crate::runnable::Runnable;
use std::collections::BTreeMap;

#[test]
fn deserialize_string() {
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
fn deserialize_args_string() {
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
fn deserialize_args_value_list_string() {
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
fn deserialize_cd_string() {
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
fn deserialize_name_string() {
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
fn to_command() {
    let check = OutdatedCheck {
        args: None,
        cd: None,
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    let cmd = check.to_command().unwrap();
    let cmd_str = format!("{cmd:?}");
    assert_eq!(cmd_str, "\"medic-outdated-thing\"");
}

#[test]
fn to_command_cd_relative() -> Result<(), Box<dyn std::error::Error>> {
    let check = OutdatedCheck {
        args: None,
        cd: Some("../fixtures/bin".to_string()),
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    let mut context = std::collections::HashMap::new();
    for (key, value) in std::env::vars() {
        context.insert(key, value);
    }
    let path_expansion = envsubst::substitute("${PWD}/fixtures/bin", &context).unwrap();
    let expected_cmd_str = format!("cd \"{path_expansion}\" && \"medic-outdated-thing\"");

    let cmd = check.to_command()?;
    let cmd_str = format!("{cmd:?}");
    assert_eq!(cmd_str, expected_cmd_str);
    Ok(())
}

#[test]
fn to_command_cd_absolute() -> Result<(), Box<dyn std::error::Error>> {
    let check = OutdatedCheck {
        args: None,
        cd: Some("/tmp".to_string()),
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    let cmd = check.to_command()?;
    let cmd_str = &format!("{cmd:?}");
    let cmd_str = Some(cmd_str.as_str());

    assert!(matches!(
        cmd_str,
        Some("cd \"/tmp\" && \"medic-outdated-thing\"")
            | Some("cd \"/private/tmp\" && \"medic-outdated-thing\"")
    ));
    Ok(())
}

#[test]
fn to_command_cd_bad_directory() {
    let check = OutdatedCheck {
        args: None,
        cd: Some("does-not-exist".to_string()),
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    let e = check.to_command().err().unwrap();
    assert_eq!(
        format!("{e}"),
        "directory does-not-exist does not exist".to_string()
    );
}

#[test]
fn to_command_missing_command() {
    let check = OutdatedCheck {
        args: None,
        cd: None,
        check: "missing".to_string(),
        name: None,
        remedy: None,
    };

    let e = check.to_command().err().unwrap();
    assert_eq!(
        format!("{e}"),
        "executable medic-outdated-missing not found in PATH".to_string()
    );
}

#[test]
fn to_string() {
    let check = OutdatedCheck {
        args: None,
        cd: None,
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36moutdated:\u{1b}[0m \u{1b}[38;5;14m\u{1b}[1mthing\u{1b}[0m"
    )
}

#[test]
fn to_string_name() {
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
fn to_string_cd() {
    let check = OutdatedCheck {
        args: None,
        cd: Some("../subdirectory".to_string()),
        check: "thing".to_string(),
        name: None,
        remedy: None,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36moutdated:\u{1b}[0m \u{1b}[38;5;14m\u{1b}[1mthing\u{1b}[0m \u{1b}[32m(../subdirectory)\u{1b}[0m"
    )
}
