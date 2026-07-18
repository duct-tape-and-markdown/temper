# 0040 — the tension edges resolve toward the map

- **Date:** 2026-07-17 · **Status:** accepted

## Context

`specs/process/architecture.md` (663e03f) declared two edges standing
in tension with its own layering rather than hiding them — `drift →
install` (pipeline core reaching into a verb) and `frontmatter →
builtin_kind` (a format face knowing the provider) — and the foundation
posture sweep surfaced a third the map's own "holds today" claim missed:
`extract` (foundation) takes `drift::NestedMemberRow` as a parameter
type and branches on `kind::CollectionKeyPath` /
`kind::ENABLEMENT_FIELD`. Each edge was verified on disk (fork records
`(drift-install-edge)`, `(frontmatter-builtin-kind-edge)`,
`(extract-foundation-edge)`; sites re-read at eaeaae8 before ruling).
The human delegated this class — research, weigh, execute — 2026-07-17;
session-ruled under that delegation.

## Decision

**In all three cases the layering is ratified and the code moves.** The
map is amended only where its factual claim was false (foundation's
"holds today" parenthetical), never to admit an upward edge. An
invariant stated as an absence stays cheap to sweep only while it is
unqualified; each edge's job has a real home downstream.

1. **`drift → install` — the placement vocabulary gets its own home.**
   "Which lines are temper's managed metadata" is one job shared by two
   consumers: install *places and audits* the modeline/note/banner, emit
   *preserves* them. The recognizer vocabulary (the marker constants,
   `is_placement_comment`, `placement_lines`) moves to a new flat
   **`placement`** module in the pipeline subsystem; install keeps the
   placing actions and wording (`project_note`, `project_banner`) and
   imports the vocabulary downward, exactly as `drift`'s emit does.

2. **`frontmatter → builtin_kind` — the fixtures go synthetic.** The
   edge is test-only (`#[cfg(test)]` fixtures calling
   `builtin_kind::definition`), but a change to a shipped kind failing a
   *format adapter's* tests is a misplaced signal, and scoping the
   invariant to production code makes the absence unsweepable at a
   glance. The adapter's fixtures build synthetic kinds via a
   kind-builder in `test_support` (the declared shared fixture home);
   real-kind coverage through the adapter stays where it already lives,
   with the provider's own tests.

3. **`extract` sheds its accreted non-foundation jobs.** `extract` is
   foundation: the `Features` vocabulary and markdown/heading mechanics
   (plus `host_address`, pure string work). The lock-row lifters
   (`nested_members_from_rows`, `embedded_member_from_row`) are lock
   mechanics and move to `drift`, which defines the row type — pipeline
   depending on foundation, the intended direction. The
   manifest-collection grammar — both faces: `manifest_members`,
   `entry_fields`, `hook_member_fields`, `enablement_member_fields`,
   and the write twins `enablement_entry_value`, `hook_matcher_group` —
   moves to `json_manifest`: a format owns its grammar's read and write
   faces (the frontmatter precedent), and formats knowing the model is
   a sanctioned direction (`frontmatter` already takes `CustomKind`);
   the reverse is not. After the moves `extract` imports no sibling,
   and its header's narrower longstanding boundary ("no dependency on
   `crate::contract`") is subsumed by the restored subsystem invariant.

## Rejected

- **Amending the map to admit each edge.** The layering is the value
  the page exists to hold; all three edges have cheaper resolutions
  than a carve-out, and a qualified absence ("…except in tests",
  "…except extract") is exactly the remembered-list disease the sweep
  was just cured of.
- **Homing the placement vocabulary in `drift`** (no new module,
  install already imports drift for `EmitOwnedEntry`): feeds the
  cohesion-strained largest module and re-litigates the direction the
  next time a third consumer appears; splits land flat and cheap here.
- **Keeping real-kind fixtures in the frontmatter tests** for the free
  integration coverage: the coupling is the cost — provider changes
  breaking format tests point the reader at the wrong module.

## Consequences

- `src/placement.rs` — new flat module, **pipeline** subsystem, created
  by its entry per the growth rule; `install` and `drift` import it.
- `architecture.md`: the tension paragraph is replaced by the ruled
  directions, the pipeline codemap gains `placement`, and the
  foundation invariant's false "holds today" claim is corrected to name
  the in-flight moves.
- `test_support` gains a synthetic kind-builder; `frontmatter`'s test
  module drops its `builtin_kind` import.
- `extract.rs` ends with no crate-internal imports; `drift.rs:2603`'s
  doc-comment pointer to `install::matches_projection` rewords in scope
  of the placement entry.
- Pending `EXTRACT-PRIVATE-COLLECTION-DECODERS` reconciles against the
  moves: the visibility narrowing lands at the functions' new homes
  (`manifest_members` goes private to `json_manifest`; the write twins
  stay `pub(crate)` for `drift`), not at `extract`.
- The three fork records delete.
