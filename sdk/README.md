# temper-sdk — the authoring face

The typed module library the ratified corpus names as temper's authoring
medium (`specs/intent.md`; `specs/model/pipeline.md`, "The SDK"). A harness
author composes members as typed
values in the **six-noun model**; `emit` compiles the whole into the declaration
rows the engine reads, a byte-faithful `.claude/**` projection, and the lock. The
SDK implements **no semantics** — every type erases at the seam, and the engine
consumes only declared data, offline, no Node.

## The six-noun face

- **`harness()`** — the assembly as one typed value: `members · expect ·
  require · settings` (`specs/model/pipeline.md`, "The SDK").
- **`kind<T>()`** — the engine room: a kind is a typed
  constructor plus five facts of runtime residue (label, locus, layout,
  registration, edge fields — `specs/model/representation.md`). The built-in
  Claude Code kinds `rule` / `skill` / `memory` are ordinary `kind<T>()` values;
  an embedded child kind is the same constructor at the `embedded` locus.
- **Clause values** — `clause(predicate, { severity, guidance, cite })` over the
  closed predicate algebra (`required`, `maxLines`, …); a floor is an exported
  clause array, adopted by spread in `expect` (`specs/model/contract.md`).
- **`needs`** — the capabilities a member uses (`bash("git diff")`); emit derives
  the permission union, so a permission is never authored twice.
- **`file()` / `` text`…` `` / `blocks()`** — the three prose constructors, one
  field type; the author's words land byte-identical to their authored text.

## What `emit` produces

One deterministic pass over the harness, double-emit verified
(`specs/model/pipeline.md`, "Emit"):

- **Declaration rows** — the erased program (kind facts, clauses, requirements,
  assembly facts) on the internal versioned JSON pipe and in the lock's
  `[declaration]` families, byte-matching the Rust lock shape (`src/drift.rs`) —
  the byte-parity lockstep two writers keep until single-writer lands.
- **A byte-faithful projection** — each `rule` / `skill` / `memory` member
  compiled whole to its `.claude/**` locus; install's placement lines round-trip.
- **The lock** — rollup provenance/emit fingerprints plus the declaration rows.

Emit is **total** (members are the only source), **refuses** before it writes on
a broken source (a dangling `satisfies`, an unfilled `required`, an unresolved
mention), and is **byte-reproducible**. `writeEmit` lands the lock and the
projection on disk; the JSON pipe is in-flight, not a committed artifact.

## Stated bounds — each a named follow-on, never silently faked

- **The permission union is carried as data** — the fold into the settings
  artifact lands with the hook/MCP kinds it folds many-to-one.

## Tests

`pnpm --dir sdk test` — `tsc` (the keystroke wall) then `node --test`, including
projection byte-parity and lock fingerprints against real Rust output, and the
declaration-row byte shape against the Rust `[declaration]` families.
