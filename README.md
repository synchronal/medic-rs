# Medic

Medic is a development workflow management tool. This project is a
re-implementation of [Elixir medic](https://github.com/synchronal/medic)
in Rust, with the intention that it can be installed as a set of
stand-alone binaries and scripts with minimal tool-chain dependencies.

The medic workflow is intended to quickly bootstrap development on a
code project. Steps and checks are run to verify that all dependencies
are satisfied, with suggested remedies. Checks should be written such
that they only care about the dependency being met, rather than how it's
met—all suggested remedies might be applied as-is on a shared
workstation, while an individual might choose alternate solutions on
their personal hardware.

Rather than proscribe, medic should assist.

## Installation

``` shell
brew install synchronal/tap/medic

# optionally add extensions which include steps, checks, and/or outdated checks:
brew install synchronal/tap/medic-ext-elixir
brew install synchronal/tap/medic-ext-git
brew install synchronal/tap/medic-ext-homebrew
brew install synchronal/tap/medic-ext-node
brew install synchronal/tap/medic-ext-rust
brew install synchronal/tap/medic-ext-tool-versions
brew install synchronal/tap/medic-ext-postgres
```

## Usage

Medic provides five commands, each of reads its configuration from a
TOML-formatted file, which defaults to `.config/medic/medic.toml`.

``` shell
medic init     # -- add a medic config manifest to a project.
medic doctor   # -- ensure a project is fully set up for development.
medic test     # -- run all test commands.
medic audit    # -- run lints, type checks, dependency audits, etc.
medic outdated # -- check for outdated project dependencies.
medic update   # -- update the project with upstream changes.
medic shipit   # -- run all checks and ship your changes.
medic run      # -- runs a shell command with medic progress output.
```

#### init

`medic init` creates a medic config manifest in the current directory,
at `.config/medic.toml`.

#### doctor

`medic doctor` runs checks to ensure the project is ready for a
developer to work on a project.

Examples:

- Are all dependencies installed?
- Are all required databases installed and accessible?
- Are development services and fakes running?
- Are test services available?

Valid actions:

- checks

#### test

`medic test` runs all tests. Useful not just for documenting *the* test
suite used for your project, but for when multiple test suites are used
(`ExUnut + bats`, etc)

Valid actions:

- steps
- shell actions
- doctor

#### audit

`medic audit` is intended for anything that has to do with formatting or
security.

Examples:

- is the code properly formatted (`mix format --check-formatting`,
  `prettier --check`, etc)?
- linters (`cargo clippy`, etc)
- dependency audits (`mix_audit`, `cargo-audit`, `npm audit`, etc)

Valid actions:

- checks
- steps
- shell actions

#### outdated

`medic outdated` checks for dependencies that might be updatable.

Examples:

- language
- runtime version manager (asdf, rtx)
- packages (cargo, mix, pip)

#### update

`medic update` updates the project with upstream changes.

Examples:

- git pull
- update dependencies (without checking)
- compile dependencies
- run database migrations
- run `medic doctor`

Valid actions:

- steps
- shell actions
- doctor

#### shipit

`medic shipit` runs all necessary actions to ship new code in a safe
manner.

Valid actions:

- checks
- steps
- shell actions
- audit
- test
- update

#### run

`medic run` executes an arbitrary shell command.

Arguments:

- `--name <name>` - used in progress indicators.
- `--cmd <cmd>` - the shell command to execute.
- `--remedy <remedy>` - an optional remedy to output if the command
  fails.
- `--verbose` - optionally writes output to the terminal alongside
  running progress.

## Configuration

Each command runs a set of checks and/or steps, with some commands
optionally accepting configuration indicating that medic should run
another of its commands.

- `check`: Run the shell command `medic-check-{name}` with an optional
  subcommand and optional args. If the check provides a remedy (see
  below), then upon failure the remedy will be added to the system
  clipboard.
- `shell`: Will run the specified shell command as-is.
  - `verbose` - print all stdout/stderr to the terminal as it happens.
  - `allow_failure` - continue medic even if the command fails.
- `step`: Runs the shell command `medic-step-{name}` with optional
  subcommand and args.
  - `verbose` - print all stdout/stderr to the terminal as it happens.
  - `allow_failure` - continue medic even if the command fails.

``` toml
[doctor]
checks = [
  # medic-check-tool-versions plugin-installed --plugin rust
  { check = "tool-versions", command = "plugin-installed", args = { plugin = "rust" } },
  # medic-check-tool-versions package-installed --plugin rust
  { check = "tool-versions", command = "package-installed", args = { plugin = "rust" } },
  # medic-check-homebrew
  { check = "homebrew", verbose = true, output= "stdio" },
  # ... etc
  { check = "rust", command = "crate-installed", args = { name = ["cargo-audit", "cargo-outdated"] } },
  { check = "rust", command = "target-installed", args = { target = "aarch64-apple-darwin" } },
  { check = "rust", command = "target-installed", args = { target = "x86_64-apple-darwin" } },
]

[test]
checks = [
  { name = "Check for warnings", shell = "cargo build --workspace --features strict" },
  # medic-step-rust test
  { step = "rust", command = "test", verbose = true },
]

[audit]
checks = [
  { name = "Audit crates", shell = "cargo audit", allow_failure = true, verbose = true },
  { check = "rust", command = "format-check" },
  { name = "Shell format check", shell = "cargo fmt --check", remedy = "cargo fmt" },
  { step = "rust", command = "clippy" },
]

[outdated]
checks = [
  # medic-outdated-rust
  { check = "rust" },
  # cd crates/sub-crate && medic-outdated-rust
  { check = "rust", cd: "crates/sub-crate" },
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

Custom checks may be run, so long as they are named `medic-check-{name}`
and are available in the PATH.

- `check` - REQUIRED - name of the check. Shells out to
  `medic-check-{name}`, which is expected to be available in the PATH.
- `command` - an optional subcommand to pass as the first argument to
  the check.
- `args` - a map of flag to value(s). When running the command, the flag
  name will be translated to `--flag <value>`. When the value is
  specified as a list, the flag will be output once per value.
- `cd` - change directory before running checks.
- `output` - the output format used by the check.
- `verbose` - when `true`, STDERR of the check is redirected to STDERR
  of the current medic process.

Checks must follow one or more output format, which is provided to the
check in the environment: variable `MEDIC_OUTPUT_FORMAT`:

#### json (default)

- Informational output may be written to STDERR in any format. If the
  check is configured with `verbose = true`, this output will be written
  directly to the STDERR of medic as it happens.
- JSON should be written to STDOUT with the following optional keys:
  ``` json
  {
    "output": "Output to display to the user, for example STDOUT captured from internal commands",
    "error": "Error to display to the user",
    "remedy": "suggested remedy to resolve the problem"
  }
  ```
- If the check fails, the process must exit with a non-zero exit status.

Note that upon failure, the `error` key in the output JSON takes
priority over STDERR.

#### stdio

- Informational output may only be written to STDERR.
- The suggested remedy (if available) must be written to STDOUT.
- If the check fails, the process must exit with a non-zero exit status.

### Steps

Custom steps may be run, so long as they are named `medic-step-{name}`
and are available in the PATH. Steps must follow:

- Informational output may be written to STDERR or STDOUT.
- If the step fails, the process must exit with a non-zero exit status.
- `command` - an optional subcommand to pass as the first argument to
  the check.
- `args` - a map of flag to value(s). When running the command, the flag
  name will be translated to `--flag <value>`. When the value is
  specified as a list, the flag will be output once per value.
- `verbose` - print all stdout/stderr to the terminal as it happens.
- `allow_failure` - continue medic even if the command fails.

### Shell actions

Arbitrary shell actions can be run. If the shell command returns a
non-zero exit status, then the action is deemed a failure.

Note that pipes and redirections are not handled, so complex shell
commands may be better suited to be written into shell scripts.

- `name` - the description to be shown to the user when run.
- `shell` - the command to run
- `cd` - change directory before running commands.
- `inline` - when `true`, disables running progress bars and prints all
  output directly to the terminal. This flag takes priority over
  `verbose`, and is useful when running scripts that handle their own
  progress indicators, for example when using `medic run`.
- `verbose`- when `true`, STDOUT and STDERR of the action are printed as
  to the console alongside running progress.
- `allow_failure` - allow medic to continue even when the process fails.

### Outdated checks

Outdated checks work differently from other types of checks.

These checks run commands named `medic-outdated-{check}` that must be
found in the PATH. These commands must follow these rules:

- Informational output must be written to STDERR.
- An outdated dependency must be written to STDOUT in one of the
  following formats:
  `::outdated::name=<name>::version=<version>::latest=<latest>`
  `::outdated::name=<name>::version=<version>::latest=<latest>::parent=<parent>`
- An optional remedy for updating dependencies may be output to STDOUT
  in the following format: `::remedy::<command>`

Values to be included:

- `name` - the name of the dependency.
- `version` - the version of the dependency currently used.
- `latest` - the most current available version of the dependency.
- `parent` - if the project does not explicitly declare this dependency,
  `parent` may be set to show why this is appearing in outdated content.

### Colorization

Output from steps and checks can be colorized using unicode or
hexadecimal ANSI escape sequences.

``` shell
echo "\u001b[1;31mHere is some red text\u001b[0m" >&2
echo "\x1b[1;33mHere is some yellow text\x1b[0m" >&2
```

## Development

``` shell
brew bundle
bin/dev/doctor

cargo run --bin medic-doctor -- -c fixtures/medic.toml
```

## Notes

- This project uses the unstable feature `try_trait_v2`, which requires
  nightly Rust. Until the feature is made stable, compilation of this
  project could break at any time with changes to nightly Rust.
