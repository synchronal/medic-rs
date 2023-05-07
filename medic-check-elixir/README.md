# medic-check-elixir

Checks for whether an Elixir project is configured.

## unused deps?

Are there any hex deps listed in `mix.lock` that are not explicitly
or implicitly listed in `mix.exs`?

```shell
medic-check-elixir unused-deps
medic-check-elixir unused-deps --cd path/to/project
```
