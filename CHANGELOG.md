# Change log

## Unreleased

- Cascade config file to nested sub-commands.
- Capture interactive quit from nested sub-commands.

## 2.13.0

- Add `platform` configuration option to steps, checks, and shell
  actions. When present, limits the actions to the given platforms.

## 2.12.0

- Reduce noise when failures do not produce output.
- Interactive mode allows steps to be rerun.
- Tests without remedies are not recoverable in interactive mode.
- Fix spacing in interactive help.

## 2.11.0

- *Slightly* better handling of errors in interactive mode.
- Checks and steps print more complete error messages when executables
  are not found in PATH.

## 2.10.0

- Set `MEDIC_APPLY_REMEDIES` when running remedies from interactive
  mode.

## 2.9.2

- Update dependencies.

## 2.9.1

- Fix: applying a check remedy with a `cd` configuration would attempt
  to change directories twice, usually resulting in a directory not
  found error.

## 2.9.0

- Add `--apply-remedies` flag.

## 2.8.0

- Add `?` help option to interactive prompt.

## 2.7.0

- Fix potential panic in `medic outdated`.
- Add `--interactive` flag.

## 2.6.2

- Exclude variables that include `{}` characters when interpolating the
  manifest path.

## 2.6.1

- Reset cursor when interrupting commands with Ctrl-C.

## 2.6.0

- Add `env` configuration to checks, steps, and shell steps.

## 2.5.0

- Disable cursor when running steps or checks.

## 2.4.2

- Checks and shell steps include the `cd` option in remedies.

## 2.4.1

- Update deps.

## 2.4.0

- steps can be configured with `cd` to change directory.

## 2.3.0

- `medic run` accepts a `--cd` option.
- checks and shells actions can be configured via `cd` to change
  directories before running their commands.

## 2.2.0

- `medic init` can `--force` overwrite the config file.
- Include outdated in `medic init`

## 2.1.2

- Show STDERR from outdated checks in progress bar.
- Error handling when parsing outdated STDOUT.

## 2.1.1

- Checks and steps verify that their commands exist in PATH before
  running them.
- Catch more possible errors instead of panicking. Where panics may
  still be the best approach, prefer `expect` in place of `unwrap`.

## 2.1.0

- Adds `medic outdated`.
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
