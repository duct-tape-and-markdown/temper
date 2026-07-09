<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- The TS<->Rust seam's hand-synchronization is a hand-roll beside an
  installed mechanism (`specs/process/engineering.md`): Cargo.toml carries
  `ts-rs = "12"` with its own comment declaring the intent ("schemars
  derives the JSON Schema 2020-12, ts-rs the TypeScript types both
  implementations"), it is derived on exactly one type (`src/extract.rs`
  `Section`), and `sdk/src` consumes generated bindings nowhere —
  `declarations.ts` hand-restates every row shape `drift.rs` reads, with
  `#[derive(Deserialize)]` field names as the only contract and
  `tests/builtin_lock_frozen.rs` the only tripwire. Cost this week alone:
  the wire-naming red loop (drained friction), 0003's `Requirement.kind`
  stranding, `ExpectBinding.kind` (same bug, second field). Entry shape:
  derive `TS` on the declaration-row family — `RequirementRow`,
  `SatisfiesRow`, `MentionRow`, `NestedMemberRow`, `KindFactRow`,
  `ClauseRow`, the payload envelope — emit bindings into
  `sdk/src/generated/` as a build step, and recut `declarations.ts`'s row
  types to import them; the authored builder API stays. Honest bound:
  kills shape drift as a class (a Rust-side rename becomes an SDK compile
  error, not a three-day silence); does NOT catch semantic drift (0001's
  dropped `means` was shape-legal). Sequencing ask: land before
  KIND-CONTENT-FACT/LAYOUT-READER so 0019's new row families are born
  generated, not hand-mirrored — their gating behind the prose chain
  leaves the window. Observed at 0af7bb7.
