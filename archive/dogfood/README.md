# Archived dogfood surface — parked pending the model reconciliation

These files — the root **`temper.toml`** (temper's own assembly) and the **`.temper/`**
imported+authored surface — are temper applying itself to its own harness (the
recursive dogfood). They are parked here, not deleted: `git mv`'d intact, history
preserved.

## Why they're here

They embody the **pre-reconciliation** model:

- `temper.toml` inlines the whole `[kind.spec]` definition (extraction + a
  `[[kind.spec.clause]]`) and frames itself as "the contract."

The `specs/` corpus has since moved to a new model (see `specs/00-intent.md`,
`05-model.md`, `10-contracts.md`, `15-kinds.md`, `20-surface.md`, `40-composition.md`):

- `temper.toml` is a **thin assembly** (binds packages to kinds, declares
  requirements + relationships) — it no longer inlines kind definitions or clauses;
- a custom kind's declare-side lives under `.temper/kinds/<name>/`;
- **clauses live only in packages** under `.temper/packages/<name>/`; every kind binds
  a package (`[kind.<name>] package = "<name>"`), none inlines its contract;
- the `contracts/` embedded std-lib **retires** — a built-in package's authoritative
  home becomes `.temper/packages/<name>/`, embedded into the binary at build.

The surface can't be hand-forward-ported, because the code that reads it
(`src/compose.rs` and the `contracts/` embedding in `src/bundle.rs`) still implements
the old model. Surface and code are **one coordinated reconciliation**.

## Revisit when

The code implements the new spec model — `src/compose.rs` reads kinds from
`.temper/kinds/` and packages from `.temper/packages/`, clauses-only-in-packages, and
the std-lib embeds from `.temper/packages/` instead of `contracts/`. Then re-establish
the dogfood on the new surface.

## To restore

```
git mv archive/dogfood/temper.toml temper.toml
git mv archive/dogfood/.temper .temper
```
