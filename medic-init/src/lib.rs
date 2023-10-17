#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_src::AppResult;
use std::io::Write;

pub fn create_config_file(path: std::path::PathBuf) -> AppResult<()> {
    let cwd = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut context = std::collections::HashMap::new();
    context.insert("CWD".to_string(), cwd);
    for (key, value) in std::env::vars() {
        context.insert(key, value);
    }

    let path_expansion = envsubst::substitute(path.to_string_lossy(), &context).unwrap();
    let expanded_path = std::path::Path::new(&path_expansion);
    let config_dir = expanded_path.parent().unwrap();

    std::fs::create_dir_all(config_dir)?;

    println!("Creating file: {:?}", &expanded_path);

    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(expanded_path)?;

    writeln!(file, "[doctor]")?;
    writeln!(file)?;
    writeln!(file, "checks = [")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[test]")?;
    writeln!(file)?;
    writeln!(file, "checks = [")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[audit]")?;
    writeln!(file)?;
    writeln!(file, "checks = [")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[update]")?;
    writeln!(file)?;
    writeln!(file, "steps = [")?;
    writeln!(file, "  {{ step = \"git\", command = \"pull\" }},")?;
    writeln!(file, "  {{ doctor = {{}} }},")?;
    writeln!(file, "]")?;
    writeln!(file)?;
    writeln!(file, "[shipit]")?;
    writeln!(file)?;
    writeln!(file, "steps = [")?;
    writeln!(file, "  {{ audit = {{}} }},")?;
    writeln!(file, "  {{ update = {{}} }},")?;
    writeln!(file, "  {{ test = {{}} }},")?;
    writeln!(file, "  {{ step = \"git\", command = \"push\" }},")?;
    writeln!(file, "]")?;
    writeln!(file)?;

    AppResult::Ok(())
}
