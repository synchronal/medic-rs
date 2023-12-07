#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::AppResult;
use std::io::Write;

pub fn create_config_file(path: std::path::PathBuf, force: bool) -> AppResult<()> {
    let cwd = std::env::current_dir()?.into_os_string().into_string()?;
    let mut context = std::collections::HashMap::new();
    context.insert("CWD".to_string(), cwd);
    for (key, value) in std::env::vars() {
        context.insert(key, value);
    }

    let path_expansion = envsubst::substitute(path.to_string_lossy(), &context).unwrap();
    let expanded_path = std::path::Path::new(&path_expansion);
    let config_dir = expanded_path.parent().unwrap();

    std::fs::create_dir_all(config_dir)?;

    if let Ok(metadata) = std::fs::metadata(expanded_path) {
        match (metadata.is_file(), force) {
            (false, _) => println!("Creating file: {:?}", &expanded_path),
            (true, false) => {
                return AppResult::Err(Some(
                    format!("File {:?} already exists", expanded_path).into(),
                ))
            }
            (true, true) => {
                println!("Overwriting file: {:?}", &expanded_path);
                std::fs::remove_file(expanded_path)?;
            }
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(expanded_path)?;

    writeln!(file, "[doctor]")?;
    writeln!(file, "checks = [")?;
    writeln!(file, "  # {{check = \"homebrew\"}},")?;
    writeln!(file, "  # {{check = \"tool-versions\", command = \"plugin-installed\", args = {{plugin = \"my language\"}}}},")?;
    writeln!(file, "  # {{check = \"tool-versions\", command = \"package-installed\", args = {{plugin = \"my language\"}}}},")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[test]")?;
    writeln!(file, "checks = [")?;
    writeln!(
        file,
        "  # {{name = \"Run tests\", shell = \"run my test suite\"}},"
    )?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[audit]")?;
    writeln!(file, "checks = [")?;
    writeln!(
        file,
        "  # {{name = \"Format check\", shell = \"check that code is properly formatted\"}},"
    )?;
    writeln!(
        file,
        "  # {{name = \"Lint code\", shell = \"run my code linter\"}},"
    )?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[outdated]")?;
    writeln!(file, "checks = [")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[update]")?;
    writeln!(file, "steps = [")?;
    writeln!(file, "  {{ step = \"git\", command = \"pull\" }},")?;
    writeln!(file, "  {{ doctor = {{}} }},")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[shipit]")?;
    writeln!(file, "steps = [")?;
    writeln!(file, "  {{ audit = {{}} }},")?;
    writeln!(file, "  {{ update = {{}} }},")?;
    writeln!(file, "  {{ test = {{}} }},")?;
    writeln!(file, "  {{ step = \"git\", command = \"push\" }},")?;
    writeln!(file, "]")?;
    writeln!(file)?;

    AppResult::Ok(())
}
