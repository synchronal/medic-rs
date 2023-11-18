use super::*;
use std::collections::BTreeMap;

#[test]
fn test_check_deserialize_arg_string() {
    let toml = r#"
        check = "check-name"
        command = "subcommand"
        args = { name = "first" }
        "#;

    let result: Check = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        Check {
            args: Some(BTreeMap::from([(
                "name".to_owned(),
                ArgValues(vec!["first".to_owned()])
            )])),
            check: "check-name".to_owned(),
            command: Some("subcommand".to_owned()),
            output: OutputFormat::Json,
            verbose: false
        }
    )
}
#[test]
fn test_check_deserialize_arg_list() {
    let toml = r#"
        check = "check-name"
        command = "subcommand"
        args = { name = ["first", "second"] }
        "#;

    let result: Check = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        Check {
            args: Some(BTreeMap::from([(
                "name".to_owned(),
                ArgValues(vec!["first".to_owned(), "second".to_owned()])
            )])),
            check: "check-name".to_owned(),
            command: Some("subcommand".to_owned()),
            output: OutputFormat::Json,
            verbose: false
        }
    )
}

#[test]
fn test_check_to_string_single_arg() {
    let check = Check {
        args: Some(BTreeMap::from([(
            "name".to_owned(),
            ArgValues(vec!["first".to_owned()]),
        )])),
        check: "check-name".to_owned(),
        command: None,
        output: OutputFormat::Json,
        verbose: false,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36mcheck-name \u{1b}[0;33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn test_check_to_string_subcommand_single_arg() {
    let check = Check {
        args: Some(BTreeMap::from([(
            "name".to_owned(),
            ArgValues(vec!["first".to_owned()]),
        )])),
        check: "check-name".to_owned(),
        command: Some("subcommand".to_owned()),
        output: OutputFormat::Json,
        verbose: false,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36mcheck-name: \u{1b}[0;36msubcommand? \u{1b}[0;33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn test_check_to_string_subcommand_multiple_args() {
    let check = Check {
        args: Some(BTreeMap::from([
            ("name".into(), ArgValues(vec!["first".into()])),
            ("version".into(), ArgValues(vec!["second".into()])),
        ])),
        check: "check-name".to_owned(),
        command: Some("subcommand".to_owned()),
        output: OutputFormat::Json,
        verbose: false,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36mcheck-name: \u{1b}[0;36msubcommand? \u{1b}[0;33m(name: first, version: second)\u{1b}[0m"
    )
}

#[test]
fn test_check_to_string_subcommand_multiple_arg_values() {
    let check = Check {
        args: Some(BTreeMap::from([(
            "name".into(),
            ArgValues(vec!["first".into(), "second".into()]),
        )])),
        check: "check-name".to_owned(),
        command: Some("subcommand".to_owned()),
        output: OutputFormat::Json,
        verbose: false,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36mcheck-name: \u{1b}[0;36msubcommand? \u{1b}[0;33m(name: first, name: second)\u{1b}[0m"
    )
}

#[test]
fn test_check_to_string_subcommand_multiple_arg_values_and_args() {
    let check = Check {
        args: Some(BTreeMap::from([
            (
                "name".into(),
                ArgValues(vec!["first".into(), "second".into()]),
            ),
            ("other".into(), ArgValues(vec!["third".into()])),
        ])),
        check: "check-name".to_owned(),
        command: Some("subcommand".to_owned()),
        output: OutputFormat::Json,
        verbose: false,
    };

    assert_eq!(
        format!("{check}"),
        "\u{1b}[36mcheck-name: \u{1b}[0;36msubcommand? \u{1b}[0;33m(name: first, name: second, other: third)\u{1b}[0m"
    )
}
