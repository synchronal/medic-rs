// @related [subject](medic-src/src/step/step_config.rs)

use super::*;

#[test]
fn test_deserialize() {
    let toml = r#"
        shell = "some command"
        name = "Run some command"
        "#;

    let result: ShellConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        ShellConfig {
            allow_failure: false,
            name: "Run some command".into(),
            remedy: None,
            shell: "some command".into(),
            verbose: false,
        }
    );
}

#[test]
fn test_deserialize_verbose() {
    let toml = r#"
        shell = "some command"
        name = "Run some command"
        verbose = true
        "#;

    let result: ShellConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        ShellConfig {
            allow_failure: false,
            name: "Run some command".into(),
            remedy: None,
            shell: "some command".into(),
            verbose: true,
        }
    );
}

#[test]
fn test_deserialize_allow_failure() {
    let toml = r#"
        shell = "some command"
        name = "Run some command"
        allow_failure = true
        "#;

    let result: ShellConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        ShellConfig {
            allow_failure: true,
            name: "Run some command".into(),
            remedy: None,
            shell: "some command".into(),
            verbose: false,
        }
    );
}

#[test]
fn test_deserialize_remedy() {
    let toml = r#"
        shell = "some command"
        name = "Run some command"
        remedy = "do something"
        "#;

    let result: ShellConfig = toml::from_str(toml).unwrap();
    assert_eq!(
        result,
        ShellConfig {
            allow_failure: false,
            name: "Run some command".into(),
            remedy: Some("do something".into()),
            shell: "some command".into(),
            verbose: false,
        }
    );
}

#[test]
fn test_to_string() {
    let shell = ShellConfig {
        allow_failure: false,
        name: "Run some command".into(),
        remedy: Some("do something".into()),
        shell: "some command".into(),
        verbose: false,
    };

    assert_eq!(
        format!("{shell}"),
        "\u{1b}[36mRun some command\u{1b}[0m \u{1b}[33m(some command)\u{1b}[0m"
    );
}