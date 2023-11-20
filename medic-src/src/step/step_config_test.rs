// @related [subject](medic-src/src/step/step_config.rs)

use super::*;
use crate::string_or_list::StringOrList;
use std::collections::BTreeMap;

#[test]
fn test_step_deserialize_arg_string() {
    let toml = r#"
        step = "step-name"
        command = "subcommand"
        args = { name = "first" }
        "#;

    let result: StepConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        StepConfig {
            args: Some(BTreeMap::from([(
                "name".to_owned(),
                StringOrList(vec!["first".to_owned()])
            )])),
            name: None,
            step: "step-name".to_owned(),
            command: Some("subcommand".to_owned()),
            verbose: false
        }
    )
}
#[test]
fn test_step_deserialize_arg_list() {
    let toml = r#"
        step = "step-name"
        command = "subcommand"
        args = { name = ["first", "second"] }
        "#;

    let result: StepConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        StepConfig {
            args: Some(BTreeMap::from([(
                "name".to_owned(),
                StringOrList(vec!["first".to_owned(), "second".to_owned()])
            )])),
            name: None,
            step: "step-name".to_owned(),
            command: Some("subcommand".to_owned()),
            verbose: false
        }
    )
}

#[test]
fn test_step_to_string_single_arg() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_owned(),
            StringOrList(vec!["first".to_owned()]),
        )])),
        step: "step-name".to_owned(),
        name: None,
        command: None,
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn test_step_to_string_subcommand_single_arg() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_owned(),
            StringOrList(vec!["first".to_owned()]),
        )])),
        name: None,
        step: "step-name".to_owned(),
        command: Some("subcommand".to_owned()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn test_step_to_string_subcommand_multiple_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([
            ("name".into(), StringOrList(vec!["first".into()])),
            ("version".into(), StringOrList(vec!["second".into()])),
        ])),
        name: None,
        step: "step-name".to_owned(),
        command: Some("subcommand".to_owned()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, version: second)\u{1b}[0m"
    )
}

#[test]
fn test_step_to_string_subcommand_multiple_arg_values() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".into(),
            StringOrList(vec!["first".into(), "second".into()]),
        )])),
        name: None,
        step: "step-name".to_owned(),
        command: Some("subcommand".to_owned()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second)\u{1b}[0m"
    )
}

#[test]
fn test_step_to_string_subcommand_multiple_arg_values_and_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([
            (
                "name".into(),
                StringOrList(vec!["first".into(), "second".into()]),
            ),
            ("other".into(), StringOrList(vec!["third".into()])),
        ])),
        name: None,
        step: "step-name".to_owned(),
        command: Some("subcommand".to_owned()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second, other: third)\u{1b}[0m"
    )
}
