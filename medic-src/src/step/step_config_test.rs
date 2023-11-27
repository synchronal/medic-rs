// @related [subject](medic-src/src/step/step_config.rs)

use super::*;
use crate::util::StringOrList;
use std::collections::BTreeMap;

#[test]
fn test_deserialize_arg_string() {
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
                "name".to_string(),
                StringOrList(vec!["first".to_string()])
            )])),
            name: None,
            step: "step-name".to_string(),
            command: Some("subcommand".to_string()),
            verbose: false
        }
    )
}
#[test]
fn test_deserialize_arg_list() {
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
                "name".to_string(),
                StringOrList(vec!["first".to_string(), "second".to_string()])
            )])),
            name: None,
            step: "step-name".to_string(),
            command: Some("subcommand".to_string()),
            verbose: false
        }
    )
}

#[test]
fn test_to_string_single_arg() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string()]),
        )])),
        step: "step-name".to_string(),
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
fn test_to_string_subcommand_single_arg() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string()]),
        )])),
        name: None,
        step: "step-name".to_string(),
        command: Some("subcommand".to_string()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn test_to_string_subcommand_multiple_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([
            ("name".to_string(), StringOrList(vec!["first".to_string()])),
            (
                "version".to_string(),
                StringOrList(vec!["second".to_string()]),
            ),
        ])),
        name: None,
        step: "step-name".to_string(),
        command: Some("subcommand".to_string()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, version: second)\u{1b}[0m"
    )
}

#[test]
fn test_to_string_subcommand_multiple_arg_values() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string(), "second".to_string()]),
        )])),
        name: None,
        step: "step-name".to_string(),
        command: Some("subcommand".to_string()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second)\u{1b}[0m"
    )
}

#[test]
fn test_to_string_subcommand_multiple_arg_values_and_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([
            (
                "name".to_string(),
                StringOrList(vec!["first".to_string(), "second".to_string()]),
            ),
            ("other".to_string(), StringOrList(vec!["third".to_string()])),
        ])),
        name: None,
        step: "step-name".to_string(),
        command: Some("subcommand".to_string()),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second, other: third)\u{1b}[0m"
    )
}
