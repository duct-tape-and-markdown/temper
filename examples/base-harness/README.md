# base-harness

A starter harness whose documentation corpus is a temper program: the
documents under `docs/` are typed members of declared kinds, their sections
and cross-references are model structure, and `temper check` gates the
whole. The corpus is spec-authoritative — documents are declared intent,
and code reconciles toward them (see `docs/decisions/authority-arrow.md`).

What it demonstrates:

- **Doc kinds with declared layouts.** `system`, `flow`, `decision`,
  `superseded-decision`, and `glossary` are declared in
  `.temper/kinds.ts` with the same `kind()` constructor the built-ins use.
  Each document is a source, read under its layout; a heading the layout
  does not admit refuses the document whole.
- **Edges instead of links.** A flow's participants, a superseded
  decision's successor, and a system's `satisfies` claims are addresses the
  gate resolves. Lifecycle is positional: superseding a decision moves it
  to `docs/decisions/superseded/`, where the successor edge is
  unconditionally required.
- **Requirements over the corpus.** The `documented-spine` requirement is
  filled by the system documents' own `Satisfies` sections and carries a
  set-scope `count` clause.
- **Sources and projections, split.** The docs are sources; `CLAUDE.md` and
  `.claude/rules/` are projections emitted from `.temper/` modules. Editing
  a projection directly is drift.

## Run it

From the temper repository root, with the engine built (`cargo build`) and
the SDK built (`pnpm -C sdk install && pnpm -C sdk build`):

```sh
npm -C examples/base-harness/.temper install
./target/debug/temper emit --into examples/base-harness/.temper
./target/debug/temper check examples/base-harness/.temper
./target/debug/temper explain member:corpus
```

Standalone (outside this repository), replace the `file:../../../sdk`
dependency in `.temper/package.json` with the published `@dtmd/temper` and
use the installed `temper` binary.
