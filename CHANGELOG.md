# Change log

## 0.2.1

- Run shell actions via `sh -c "command"` to allow for shell expansion.

## 0.2.0

- Steps and shell actions inherit STDIN from the parent shell.
- Add `output` config to checks. Defaults to `json`.
- Expand all ENV variables from parent shell when finding manifest files.
