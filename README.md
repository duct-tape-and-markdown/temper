# author

A typed maintenance surface for the Claude Code harness.

`author` treats your Claude Code customization — skills, commands, agents, hooks,
MCP/LSP servers, `CLAUDE.md` rules, plugin & marketplace manifests, settings — as
a **typed codebase you compile**. It imports the whole harness into one validated
config surface, lints it against the documented schemas and best practices, lets
you reorganize and compose it, and writes changes back with drift-aware `apply`.

> Positioning: tools like `rulesync` make your harness *portable* across
> assistants. `author` makes your harness *good* — quality, composition,
> maintenance, in a Claude-Code-native object model.

- **Design:** [`SPEC.md`](SPEC.md)
- **Active ship target:** [`spec/RELEASE-v0.1.md`](spec/RELEASE-v0.1.md)
- **Long-range intent:** [`docs/INTENT.md`](docs/INTENT.md)

## Status

Early scaffold. Built tick-by-tick by the [flume](https://github.com/duct-tape-and-markdown/flume)
harness in [`.flume/`](.flume/) — `flume` plans work from the spec corpus and
ships it to the trunk one validated commit at a time.

## Develop

```sh
cargo build           # compile
cargo test            # run tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check

pnpm install          # one-time: install the flume control plane
pnpm exec flume status
pnpm exec flume render plan   # preview the next plan prompt (no agent call)
```
