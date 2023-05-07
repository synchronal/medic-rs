# Medic

Medic is a development workflow management tool. This project is a
re-implementation of [Elixir medic](https://github.com/synchronal/medic)
in Rust, with the intention that it can be installed as a set of
stand-alone binaries and scripts with minimal tool-chain dependencies.

The medic workflow is intended to quickly bootstrap development on a code
project. Steps and checks are run to verify that all dependencies are
satisfied, with suggested remedies. Checks should be written such that they
only care about the dependency being met, rather than how it's met—all
suggested remedies might be applied as-is on a shared workstation, while
an individual might choose alternate solutions on their personal hardware.

Rather than proscribe, medic should assist.


## Installation

```shell
brew install synchronal/tap/medic

brew install synchronal/tap/medic-elixir
brew install synchronal/tap/medic-node
brew install synchronal/tap/medic-rust
```


## Usage

Medic provides five commands, each of reads its configuration from a
TOML-formatted file, which defaults to `.medic/config.toml`.

```shell
medic doctor  # -- ensure the project is fully set up for development.
medic test    # -- run all test suites.
medic audit   # -- run lints, type checks, dependency audits, etc.
medic update  # -- update the project with upstream changes.
medic shipit  # -- run all checks and ship your changes.
```

Each command runs a set of checks and/or steps, with some commands optionally
accepting configuration indicating that medic should run another of its commands.

- `check`: Run the shell command `medic-check-{name}` with an optional subcommand and
  optional args. If the check provides a remedy (see below), then upon failure the remedy
  will be added to the system clipboard.
- `shell`: Will run the specified shell command as-is.
  - `verbose` - print all stdout/stderr to the terminal as it happens.
  - `allow_failure` - continue medic even if the command fails.
- `step`: Runs the shell command `medic-step-{name}` with optional subcommand and args.
  - `verbose` - print all stdout/stderr to the terminal as it happens.
  - `allow_failure` - continue medic even if the command fails.

```toml
[doctor]

checks = [
  # medic-check-asdf plugin-installed --plugin rust
  { check = "asdf", command = "plugin-installed", args = { plugin = "rust" } },
  # medic-check-asdf package-installed --plugin rust
  { check = "asdf", command = "package-installed", args = { plugin = "rust" } },
  # medic-check-homebrew
  { check = "homebrew", verbose = true, output= "stdio" },
  # ... etc
  { check = "rust", command = "crate-installed", args = { name = "cargo-audit" } },
  { check = "rust", command = "target-installed", args = { target = "aarch64-apple-darwin" } },
  { check = "rust", command = "target-installed", args = { target = "x86_64-apple-darwin" } },
]

[test]

checks = [
  { name = "Check for warnings", shell = "cargo build --workspace --features strict" },
  # medic-step-cargo test
  { step = "cargo", command = "test", verbose = true },
]

[audit]

checks = [
  { name = "Audit crates", shell = "cargo audit", allow_failure = true, verbose = true },
  { check = "rust", command = "format-check" },
  { step = "cargo", command = "clippy" },
]

[update]

steps = [
  { step = "git", command = "pull" },
  { doctor = {} },
]

[shipit]

steps = [
  { audit = {} },
  { update = {} },
  { test = {} },
  { name = "Release", shell = "bin/dev/release", verbose = true },
  { step = "git", command = "push" },
  { step = "github", command = "link-to-actions", verbose = true },
]
```


### Checks

Custom checks may be run, so long as they are named `medic-check-{name}` and are available
in the PATH.

- `check` - REQUIRED - name of the check. Shells out to `medic-check-{name}`, which is expected to be
  available in the PATH.
- `command` - an optional subcommand to pass as the first argument to the check.
- `args` - a map of keys and values to pass to the check in the format `--key value`.
- `output` - the output format used by the check.
- `verbose` - when `true`, STDERR of the check is redirected to STDERR of the current medic process.

Checks must follow one or more output format, which is provided to the check in the environment:
variable `MEDIC_OUTPUT_FORMAT`:

#### json

- Informational output may be written to STDERR in any format. If the check is configured
  with `verbose = true`, this output will be written directly to the STDERR of medic as
  it happens.
- JSON should be written to STDOUT with the following optional keys:
  ```json
  {
    "output": "Output to display to the user, for example STDOUT captured from internal commands",
    "error": "Error to display to the user",
    "remedy": "suggested remedy to resolve the problem"
  }
  ```
- If the check fails, the process must exit with a non-zero exit status.

Note that upon failure, the `error` key in the output JSON takes priority over STDERR.

#### stdio (default)

- Informational output may only be written to STDERR.
- The suggested remedy (if available) must be written to STDOUT.
- If the check fails, the process must exit with a non-zero exit status.


### Steps

Custom steps may be run, so long as they are named `medic-step-{name}` and are available
in the PATH. Steps must follow:

- Informational output may be written to STDERR or STDOUT.
- If the step fails, the process must exit with a non-zero exit status.


### Colorization

Output from steps and checks can be colorized using unicode or hexadecimal ANSI escape sequences.

```shell
echo "\u001b[1;31mHere is some red text\u001b[0m" >&2
echo "\x1b[1;33mHere is some yellow text\x1b[0m" >&2
```


## Development

```shell
bin/dev/doctor

cargo run --bin medic-doctor -- -c fixtures/medic.toml
```


## Notes

- This project uses the unstable feature `try_trait_v2`, which requires
  nightly Rust. Until the feature is made stable, things could break at
  any moment with changes to Rust.
