# medic-check-elixir

Checks for whether an Elixir project is configured.

## archive installed?

Is the given package installed as a mix archive?

```shell
medic-check-elixir archive-installed --name <name>
```

## local hex installed?

Is hex installed locally?

```shell
medic-check-elixir local-hex
```

## local rebar installed?

Is rebar installed locally?

```shell
medic-check-elixir local-rebar
```

## packages compiled?

Are all mix deps compiled for a project?

```shell
medic-check-elixir packages-compiled
medic-check-elixir packages-compiled --cd path/to/project
```

## packages installed?

Are all mix deps installed for a project?

```shell
medic-check-elixir packages-installed
medic-check-elixir packages-installed --cd path/to/project
```

## unused deps?

Are there any hex deps listed in `mix.lock` that are not explicitly
or implicitly listed in `mix.exs`?

```shell
medic-check-elixir unused-deps
medic-check-elixir unused-deps --cd path/to/project
```
