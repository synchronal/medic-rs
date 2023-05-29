# Change log

## Unreleased

- `medic doctor` can run steps and shell steps.
- `medic test`, `medic update` can run checks.
- Checks/steps print `key: value` when running.
- Checks print output (include error description) prior to printing errors.

## 0.3.0

- Split `medic-src` (internal) from `medic-lib` (helpers for writing checks/steps in Rust) and release `medic-lib`.

## 0.2.1

- Run shell actions via `sh -c "command"` to allow for shell expansion.

## 0.2.0

- Steps and shell actions inherit STDIN from the parent shell.
- Add `output` config to checks. Defaults to `json`.
- Expand all ENV variables from parent shell when finding manifest files.
