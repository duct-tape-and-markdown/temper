# Plan state

- Spec derived through: f1d97e4
- Audited through: 399d8e3
- Residue swept through: 399d8e3
- This tick: POST-SHIP RECONCILIATION of `9409a6c..399d8e3`. Both entries
  shipped and the ship commit already dropped them, so the substance is the
  gates the window falsified. **Audit.** (1) LOCAL-LOCUS-COMMITMENT-CLASS
  shipped (40619a0), verified on disk — `Commitment::Local`
  (`src/kind.rs`:46-56), `declare_local` (730), `local_locus_fault` (746),
  `LOCAL_LOCUS_RULE` = `kind.local-locus` (`src/main.rs`:1860). It was
  CLAUSE-LABEL-IS-AN-ADDRESS's **sole** blocker, leaving a **dangling
  `blockedBy`** the reference gate would reject: re-gated `open`. Its premise
  re-tested and holds — `ClauseRow` (`src/drift.rs`:2627) still carries **no
  label column**. (2) MANIFEST-SCHEMA-COVERAGE-ORACLE shipped (97e84a1) and
  **created a rider for three entries scoped before it existed**:
  `covered_rule` (`tests/manifest_schema_oracle.rs`:135) matches `Predicate`
  exhaustively, so CLOSED-KEYS-CLAUSE and SHAPE-PREDICATE cannot compile until
  their variant lands in an arm, and TYPE-ACCEPTS-A-SET breaks the `Type` arm
  outright (it destructures a single `kind`) while dropping six rows from
  `EXPECTED_LAG`. All three gained the file; the oracle's own third test
  already pins that closed-keys must *not* move the lag, so that entry states
  it rather than guessing. (3) Both parks hold: `git diff 9409a6c..HEAD` is
  empty over `src/graph.rs`, `tests/graph.rs`, `.github/`. (4) The window moved
  cites under six entries (`src/drift.rs` 2548→2627, emit 840→843, `value_type`
  2586→2672 / 3428→3511 / 3470→3553; `src/main.rs` 1713→1786, 1787→1902, Check
  105-119→105-122; `src/kind.rs` `Format` 507-522→555-564) — all re-read on
  disk, none carried forward. **Sweep.** Two records corrected, no new entry
  earned. `(local-locus-toml-face)` **strengthens**: its collision is shipped
  code now, not two readings of prose — a TOML dial trips `local_locus_fault`
  on contact. The stale-cite record's "no queued entry opens either file" claim
  is **falsified** — FIELD-ADDRESSING-RFC-9535-SUBSET (filed 6f72a3b) opens
  `Cargo.toml` and names both riders, so only `src/roster.rs`:465 stays
  orphaned. **Considered, not filed:** 40619a0's self-declared
  `local_document_rows` double-read — one reader called twice, not two
  implementations, so "One job, one home" does not reach it, and CLAUDE.md
  forbids trading clarity for a micro-opt over kilobyte files. That same commit
  *consolidated* `read_layout_document` into one reader across emit and check.
- Queue: 10 entries, **1 pickable** — CLAUSE-LABEL-IS-AN-ADDRESS, ungated this
  tick. Seven form one `blockedBy` chain behind it (join → 0033's four
  widenings → dial → announcement), serialized on shared files; DIAL-KIND also
  waits on `(local-locus-toml-face)`. Two parked on human acts.

Plan continues: no — inbox and refactor captures are empty, the spec delta is
drained (`f1d97e4` is still the last `specs/` commit), and both reconciliation
cursors are current at HEAD. One pickable entry; build takes over.
