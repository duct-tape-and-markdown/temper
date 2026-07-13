# base-harness

A starter harness whose documentation corpus is a temper program: the
documents under `docs/` are typed members of declared kinds, their sections
and cross-references are model structure, and `temper check` gates the
whole. The corpus is spec-authoritative — documents are declared intent,
and code reconciles toward them (see `docs/decisions/authority-arrow.md`).

What it demonstrates:

- **The docs are a projected collection.** `system`, `flow`, `decision`,
  and `superseded-decision` members are typed values in the program
  (`.temper/docs/`), each with a module-adjacent markdown narrative;
  `emit` renders them under `docs/`. Editing a rendered document is drift;
  the discipline is construction, not convention.
- **Edges are imports.** A flow's `participants` and a superseded
  decision's successor are typed fields authored from member values
  (`names(...)`, `supersede(...)` in `.temper/kinds.ts`), projected to
  frontmatter, and resolved by the gate. A dangling reference fails in the
  program before the gate ever sees it.
- **Lifecycle is positional and typed.** Superseding a decision is the
  `supersede()` operation: the replaced ruling's record lands in
  `docs/decisions/superseded/`, where the successor edge is required by
  the field's own type — the conditional "`superseded_by` iff superseded"
  never needs a validator.
- **Requirements over the corpus.** The `documented-spine` requirement
  (`required: true`) is filled by the system members' own `satisfies`
  declarations and carries a set-scope `count` clause.
- **Both content faces, deliberately.** `docs/glossary.md` is the one
  layout source — the authored home for prose-first content, read under
  its declared layout, each term an addressable member. A reference
  harness should show when to project and when to read.

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
