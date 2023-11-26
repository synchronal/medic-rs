# Change log

## Unreleased

- Attempt to return error rather than panicking when unable to find the
  current working directory.

## 2.0.1

- Headers for sub-commands are printed to stderr, respecting changes to
  terminal width.

## 2.0.0

- Allow shell commands to run with `inline = true` to disable progress
  bars and inherit stdio directly from the parent medic process.
- Add `medic run` for executing arbitrary shell commands with consistent
  formatting, progress spinners.
- Use spinners for progress indicators for checks and steps.

#### Breaking Changes:

- Medic output uses spinners to track progress. Scripts that wrapped
  medic may not work with the new CLI output.

## 1.1.0

- Checks can specify arguments as a list.
- Steps can specify arguments as a list.

## 1.0.0

- Add `medic init` to generate an *almost* empty manifest file.
- Fix `--config` description in medic help.

#### Breaking Changes:

- Medic config file defaults to `${PWD}/.config/medic.toml`

## 0.5.0

- Extract all but core medic to external projects.

## 0.4.0

- Shell steps may include remedies.

## 0.3.1

- Split out rust check/step to separate project.
- `medic doctor` can run steps and shell steps.
- `medic test`, `medic update` can run checks.
- Checks/steps print `key: value` when running.
- Checks print output (include error description) prior to printing
  errors.

## 0.3.0

- Split `medic-src` (internal) from `medic-lib` (helpers for writing
  checks/steps in Rust) and release `medic-lib`.

## 0.2.1

- Run shell actions via `sh -c "command"` to allow for shell expansion.

## 0.2.0

- Steps and shell actions inherit STDIN from the parent shell.
- Add `output` config to checks. Defaults to `json`.
- Expand all ENV variables from parent shell when finding manifest
  files.
