// @related [subject](medic-src/src/step/step_config.rs)

use super::*;
use crate::util::StringOrList;
use std::collections::BTreeMap;

#[test]
fn deserialize_cd() {
    let toml = r#"
        cd = "./subdirectory"
        step = "step-name"
        "#;

    let result: StepConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        StepConfig {
            args: None,
            cd: Some("./subdirectory".to_string()),
            name: None,
            step: "step-name".to_string(),
            command: None,
            verbose: false
        }
    )
}
#[test]
fn deserialize_arg_string() {
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
            cd: None,
            name: None,
            step: "step-name".to_string(),
            command: Some("subcommand".to_string()),
            verbose: false
        }
    )
}
#[test]
fn deserialize_arg_list() {
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
            cd: None,
            name: None,
            step: "step-name".to_string(),
            command: Some("subcommand".to_string()),
            verbose: false
        }
    )
}

#[test]
fn to_command() {
    let step = StepConfig {
        args: None,
        cd: None,
        command: None,
        name: None,
        step: "thing".to_string(),
        verbose: false,
    };

    let cmd = step.to_command().unwrap();
    let cmd_str = format!("{cmd:?}");
    assert_eq!(cmd_str, "\"medic-step-thing\"");
}

#[test]
fn to_command_cd() {
    let step = StepConfig {
        args: None,
        cd: Some("../fixtures/bin".to_string()),
        command: None,
        name: None,
        step: "thing".to_string(),
        verbose: false,
    };

    let mut context = std::collections::HashMap::new();
    for (key, value) in std::env::vars() {
        context.insert(key, value);
    }
    let path_expansion = envsubst::substitute("${PWD}/fixtures/bin", &context).unwrap();
    let expected_cmd_str = format!("cd \"{path_expansion}\" && \"medic-step-thing\"");

    let cmd = step.to_command().unwrap();
    let cmd_str = format!("{cmd:?}");
    assert_eq!(cmd_str, expected_cmd_str);
}

#[test]
fn to_command_subcommand() {
    let step = StepConfig {
        args: None,
        cd: None,
        command: Some("sub-command".to_string()),
        name: None,
        step: "thing".to_string(),
        verbose: false,
    };

    let cmd = step.to_command().unwrap();
    let cmd_str = format!("{cmd:?}");
    assert_eq!(cmd_str, "\"medic-step-thing\" \"sub-command\"");
}

#[test]
fn to_command_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string()]),
        )])),
        cd: None,
        command: None,
        name: None,
        step: "thing".to_string(),
        verbose: false,
    };

    let cmd = step.to_command().unwrap();
    let cmd_str = format!("{cmd:?}");
    assert_eq!(cmd_str, "\"medic-step-thing\" \"--name\" \"first\"");
}

#[test]
fn to_command_args_list() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string(), "second".to_string()]),
        )])),
        cd: None,
        command: None,
        name: None,
        step: "thing".to_string(),
        verbose: false,
    };

    let cmd = step.to_command().unwrap();
    let cmd_str = format!("{cmd:?}");
    assert_eq!(
        cmd_str,
        "\"medic-step-thing\" \"--name\" \"first\" \"--name\" \"second\""
    );
}

#[test]
fn to_command_missing_command() {
    let step = StepConfig {
        args: None,
        cd: None,
        command: None,
        name: None,
        step: "missing".to_string(),
        verbose: false,
    };

    let e = step.to_command().err().unwrap();
    assert_eq!(
        format!("{e}"),
        "executable medic-step-missing not found in PATH".to_string()
    );
}

#[test]
fn to_string_cd() {
    let step = StepConfig {
        args: None,
        cd: Some("./subdirectory".to_string()),
        command: None,
        name: None,
        step: "step-name".to_string(),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name\u{1b}[0m \u{1b}[32m(./subdirectory)\u{1b}[0m"
    )
}

#[test]
fn to_string_single_arg() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string()]),
        )])),
        cd: None,
        command: None,
        name: None,
        step: "step-name".to_string(),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn to_string_subcommand_single_arg() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string()]),
        )])),
        cd: None,
        command: Some("subcommand".to_string()),
        name: None,
        step: "step-name".to_string(),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first)\u{1b}[0m"
    )
}

#[test]
fn to_string_subcommand_multiple_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([
            ("name".to_string(), StringOrList(vec!["first".to_string()])),
            (
                "version".to_string(),
                StringOrList(vec!["second".to_string()]),
            ),
        ])),
        cd: None,
        command: Some("subcommand".to_string()),
        name: None,
        step: "step-name".to_string(),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, version: second)\u{1b}[0m"
    )
}

#[test]
fn to_string_subcommand_multiple_arg_values() {
    let step = StepConfig {
        args: Some(BTreeMap::from([(
            "name".to_string(),
            StringOrList(vec!["first".to_string(), "second".to_string()]),
        )])),
        cd: None,
        command: Some("subcommand".to_string()),
        name: None,
        step: "step-name".to_string(),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second)\u{1b}[0m"
    )
}

#[test]
fn to_string_subcommand_multiple_arg_values_and_args() {
    let step = StepConfig {
        args: Some(BTreeMap::from([
            (
                "name".to_string(),
                StringOrList(vec!["first".to_string(), "second".to_string()]),
            ),
            ("other".to_string(), StringOrList(vec!["third".to_string()])),
        ])),
        cd: None,
        command: Some("subcommand".to_string()),
        name: None,
        step: "step-name".to_string(),
        verbose: false,
    };

    assert_eq!(
        format!("{step}"),
        "\u{1b}[36mstep-name: subcommand!\u{1b}[0m \u{1b}[33m(name: first, name: second, other: third)\u{1b}[0m"
    )
}
