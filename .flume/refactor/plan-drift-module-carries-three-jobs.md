## Surface

`src/drift.rs` is 6703 lines carrying three jobs; its own header doc (1-15)
describes one of them.

- **The emit compiler** — `emit_program` (1380) → `emit` (1490) →
  `emit_one` (2986) / `emit_manifest` (2170) / `project_bytes` (3135), plus
  `place` (3287). The header's declared occupant.
- **The lock declaration codec** — ~2060 lines (4298-6360): every `*Row`
  struct, the `*_table` / `*_from_table` TOML round-trip pairs, the
  `req_str`/`opt_str`/`opt_table` column readers, `read_lock_document`
  (5111), `parse_declarations` (5191), `declarations_from_doc` (5213).
  Consumed from outside by name, never through emit: `Declarations` 74
  sites, `read_declarations` 70, `KindFactRow` 49, `ClauseRow` 32,
  `LockRowError` 16, `read_lock_document` 14 (`rg -a 'drift::<sym>' src/
  tests/`, drift.rs excluded) — and 20 of `src/`'s modules `use
  crate::drift`, most of them for this codec alone.
- **The lock read verbs** — `config_stale` (3374), the five
  `*_stale_from_doc` faces (4011-4198), `undeclared_layout_members_from_doc`
  (3480), `undeclared_locus_members_from_doc` (3573), `emit_owned_targets`
  (4222), the source-dep families (3628-4010).

## Observed at

ee7de9f9 (plan's posture sweep, drift.rs neighborhood)

## Suggested consolidation

The codec is the clean cut: a flat `src/lock.rs` (`architecture.md`, "The
tree stays flat" — a split lands as a new flat module) owning the row types,
the TOML round-trip and the read/parse door, with `drift.rs` keeping emit +
place and importing it. Where the row→model lifts (`clause_from_row`,
`requirement_from_row`, `edges_from_declarations`) land is the judged half.

**Route, don't mint.** The module list is a boundary call
(`architecture.md`: adjudicated boundary calls live in `specs/decisions/`),
so the drain's honest shape is a `parked` entry whose reason names the human
call — never an `open` one an autonomous build tick takes on its own.
