# Medic

Medic is a development workflow management tool. This project is a
re-implementation of [Elixir medic](https://github.com/synchronal/medic)
in Rust, with the intention that it can be installed as a set of
stand-alone binaries and scripts with minimal tool-chain dependencies.

## Status

TBD. This project is currently a work in progress, and is not ready for
general use.

## Usage

```shell
bin/dev/doctor

cargo run --bin medic-doctor -- -c fixtures/medic.toml
```

## Notes

- This project uses the unstable feature `try_trait_v2`, which requires
  nightly Rust. Until the feature is made stable, things could break at
  any moment with changes to Rust.
