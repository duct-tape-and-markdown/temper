# Plan state

- Spec derived through: abe5d5d
- Audited through: 8978596
- Residue swept through: 8978596
- This tick: INBOX — routed the one note, the `(seam-rows-public-face)`
  ruling, into **SEAM-EXPORTS-RETRACT** (`open`, `per` pipeline.md/"Emit" —
  the payload is internal, not a designed public interchange), and deleted
  the resolved fork record. The ruling's premise was re-verified at HEAD
  rather than taken from the note (observed at 2b6bd57; the window since is
  specs/ + plan/chore only, no `sdk/` commit): the ten root exports still
  read at sdk/src/index.ts:84-94 + 97, and measured consumers outside
  `sdk/test/` are zero — `.temper/harness.ts` imports `emit`/`harness`,
  `.temper/`'s other modules import the `claude-code` subpath, both READMEs
  and every `docs/proposals/` module spell only root nouns, and no Rust
  fixture names a retracted symbol. **The re-verify moved the scope twice.**
  (1) The note called the retarget one line; it is three — the value import
  `compileDeclarations` off the root at contract.test.ts:12 and
  emit.test.ts:22 sits beside the type import the note named at
  contract.test.ts:25, and a source-only grep never sees them. (2)
  root_exports.test.ts's generated exclusion splits in two: its `continue`
  at 88 goes dead (no root export is declared under `generated/` after the
  retraction) while `isAuthoringNoun`'s filter at 77 stays load-bearing,
  because `EmitResult` still names `Declarations`/`PayloadMember`
  (emit.ts:653/655) — the accepted residue the ruling names, and the reason
  the closure walk stays green through the retraction rather than breaking.
  Entry scoped to the verified gap, with emit.test.ts's existing
  removed-exports walk (623-628) named as the one home for the value-half
  assertion — no second test beside it. The eight type-only names erase at
  runtime, so strict `tsc` plus the `.d.ts` walk hold them, not that test.
  Closing checklist: queue disjoint — SEAM-EXPORTS-RETRACT is the only
  `open` entry and shares no file with either park (`sdk/**` vs `src/` +
  `tests/graph.rs` vs `.github/`). Both park gates re-tested on disk at this
  HEAD and hold verbatim: `git tag -l` carries the four era tags and no
  version tag, crate 0.1.0 vs npm 0.0.7, release.yml:7-9 still states the
  darwin + channel-3 deferral; `MAX_IMPORT_HOPS` still reads 5 at
  src/graph.rs:62 and nothing ruled the hop semantics. One fork resolved and
  deleted; the remaining four re-read, none contradicted by this window. All
  three cursors copied forward verbatim — this tick derived no spec and
  reconciled no code window; 8978596..HEAD touched no `src/`, `tests/`, or
  `sdk/` file.
- Queue: 1 pickable (SEAM-EXPORTS-RETRACT); 2 parked on human acts
  (IMPORT-HOP-CAP-CITE: a hop-depth probe. PACKAGING-CHANNELS-REMAINDER:
  Apple notarizing + the v0.1 tag). No gate is stale — both were tested this
  tick.

Plan continues: yes — the spec delta is live below this job: three `specs/`
commits past abe5d5d (14a803b, f6fe385, a571973 — decisions 0028
mention-reachable and 0029 an-edge-declares-its-target-set, plus their
contract.md edits), un-routed. Next tick derives it; each Decision's own
Consequences section is that tick's checklist.
