<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Decision 0001's "requirement as a member kind (fixes its prose never
  persisting)" never shipped — a requirement's authored intent is silently
  discarded today, and the code justifies it with a phantom citation. The
  spec body claims it as done: `specs/model/contract.md` ("requirement — a
  shipped kind, not a primitive... prose — the authored intent the
  requirement exists to carry; never interpreted") and `specs/builtins.md`
  agree. On disk: no `requirement` kind exists (`src/builtin_kind.rs`'s own
  test asserts exactly `[agent, command, memory, rule, skill]`);
  `RequirementRow` (`src/drift.rs:1204`) carries `name/kind/required/
  clauses/verified_by` — no prose column; the SDK still exposes the
  pre-0001 field 0001's retire list names for renaming (`means: string`,
  `sdk/src/contract.ts:152-158`), and `requirementRows()`
  (`sdk/src/declarations.ts:302-330`) verifiably never reads it — an
  authored `means` reaches neither the lock nor `explain`. Compounding:
  `src/main.rs:1063`'s doc comment rationalizes the drop by quoting a
  ruling — "`temper` never interprets `means`" — that exists nowhere in
  `specs/` (grep "never interprets": zero hits; contract.md says prose is
  carried-never-interpreted — a statement about carrying it, not license
  to drop it). Same shape as the routed 0003 stranding: the consequence
  crossed the noun boundary (requirement: field-bag → member kind) and no
  entry ever owned the crossing. Fix is a design ruling first (the
  embedded-locus requirement kind per contract.md's template: identity +
  prose + verifier edge + attached clauses), then the mechanical halves.
  Untracked in pending/open-questions/refactor before this note. Observed
  at 9c3b1c1.

- Decision 0013's consequences are three-quarters unshipped. Shipped:
  `format` became a typed closed vocabulary (`CustomKind::format:
  Option<Format>`, `src/kind.rs`). Unshipped, all three verified: (1) **the
  engine never composes renderer/extractor from `format`** — its sole
  consumer beyond lock round-trip is the relocation-collision equality
  check (`src/main.rs:1013`); `resolve_kind_units` (`src/main.rs:807-819`)
  calls `frontmatter::Member::from_source_rooted` unconditionally for
  every kind, so a `format: None` kind is still YAML-frontmatter-split;
  (2) **no template-admissibility rules exist** — no slot/disjointness/
  injectivity checks anywhere in `src/` (0013 assigns them to
  admissibility); (3) **extraction failures carry no source positions** —
  `frontmatter::parse_frontmatter` (`src/frontmatter.rs:265-282`) silently
  returns an empty field map on malformed YAML, and `src/extract.rs` has
  no error or span type at all. Related residue on the same surface:
  0017's no-middles inbox routing (4c256d9) named "the declared-frontmatter
  adapter" as its item-1 inert column, but the drain (e920fa8 →
  RETIRE-DEAD-DECLARED-SURFACE 90aa57b) resolved a *different* dead
  surface (`Template.leaves/collections`) and never returned to `format`;
  and two sibling `src/kind.rs` doc comments are now factually false in
  the opposite direction — `unit_shape`'s "Inert alongside format"
  (consumed by `src/frontmatter.rs:175`) and `registration`'s "nothing
  else consumes it yet" (consumed by `src/main.rs:452-455` →
  `graph.rs::live_members` since 207e701). Comment staleness rides
  whichever entry opens `src/kind.rs` (rust.md exit clause) — named here
  so that entry knows. SUPERSEDED IN PART by 0019 (ratified after this
  note): the content fact now owns the body's story — derive the reader,
  admissibility, and spans against 0019's layout and the recut
  `representation.md`/`pipeline.md` text, not 0013's format-template
  framing; what survives of this note independently is the
  extraction-span gap (frontmatter parse failures still silent) and the
  two stale kind.rs comments. Observed at 9c3b1c1; amended at 6a04322.

- Decision 0014's "the skill kind's stale profile re-verifies against the
  same fetch" was asserted and never executed. Every clause in
  `skillDefaultContract` (`sdk/src/builtins.ts:182-252`) still cites
  `retrieved 2026-07-01`; the two shipping commits (efd6caa COMMAND-KIND,
  81b6ab4 AGENT-KIND) only added the new kinds — COMMAND-KIND derives
  `commandDefaultContract` from skill's by filter without re-checking the
  source clauses, and no commit since touches skill's citations (706139a
  is a pure rename). The external-facts bar (collaboration rule: verify
  before encoding, cite at the point of claim) makes a stale citation on
  the most-used kind's contract a real exposure — the 07-07 fetch
  demonstrably changed the neighboring kinds. Work shape: one entry —
  re-fetch code.claude.com/docs/en/skills + agentskills.io/specification,
  diff against `skillDefaultContract`'s clauses, restamp cites (fixing any
  drifted clause); the built-in lock re-derives via the existing frozen
  test. Untracked before this note. Observed at 9c3b1c1.

- Small, for the `(manifest-authoring-surface)` fork record rather than an
  entry: 0015's consequence "bundle's manifests become general-write
  instances" is not named in that fork's machinery list — `src/bundle.rs`
  still hand-builds `plugin.json`/`marketplace.json`/`hooks.json` via
  bespoke serde_json writes (lines ~158-291), correctly so today (the
  general write doesn't exist yet), but nothing ties its conversion to the
  fork's resolution. One line appended to the fork record keeps it from
  being forgotten when the fork resolves. Observed at 9c3b1c1.
