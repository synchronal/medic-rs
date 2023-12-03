# Change log

## Unreleased

- When `std_to_string` is used to parse content including non-UTF-8
  characters, it panics with `expect` and a message rather than just
  `unwrap`.

## 0.1.0

- Initial release.
- Adds `std_to_string` for converting a `Command`'s stdout or stderr to
  a String.
- Adds `CheckResult` and `StepResult` for writing checks and steps.
