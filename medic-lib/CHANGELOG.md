# Change log

## Unreleased

## 0.3.0

- impl `std::ops::Residual` for `try_trait_v2` structs. This includes
  the additional declaration of the unstable `try_trait_v2_residual`
  feature.

## 0.2.0

- When `std_to_string` is used to parse content including non-UTF-8
  characters, it panics with `expect` and a message rather than just
  `unwrap`.

## 0.1.0

- Initial release.
- Adds `std_to_string` for converting a `Command`'s stdout or stderr to
  a String.
- Adds `CheckResult` and `StepResult` for writing checks and steps.
