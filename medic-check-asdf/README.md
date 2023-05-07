# medic-check-asdf

Checks for whether plugins and specific plugin packages are installed
via the `asdf` runtime version manager.

## plugin installed?

Checks whether an ASDF plugin is installed.

```shell
medic-check-asdf plugin-installed --plugin rust
```

## package installed?

Checks whether a package is installed for a specific plugin. If
`--version` is not passed, the version configured with `.tool-versions`
or `asdf global` is used.

```shell
medic-check-asdf package-installed --plugin rust
medic-check-asdf package-installed --plugin rust --version nightly
```
