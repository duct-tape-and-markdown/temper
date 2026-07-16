# Plan state

- Spec derived through: a571973
- Audited through: eb2776d
- Residue swept through: eb2776d
- This tick: POST-SHIP RECONCILIATION of 8978596..eb2776d — a one-commit
  window (e5de411, SEAM-EXPORTS-RETRACT), so both motions ran in one tick;
  no split earned. **Audit:** the retraction is verified on disk, not from
  the log — `sdk/src/index.ts` no longer names any of the ten (the
  `declarations.js` re-export block at 80+ is gone whole, `PayloadMember`
  dropped from the `emit.js` type line), the row family still exports
  module-side off `declarations.ts`, and all three suite sites reach it at
  `../src/declarations.js` (contract.test.ts:25-26, emit.test.ts:37). The
  commit's own two claims re-tested: the `(seam-rows-public-face)` fork
  record is absent from open-questions.md, and root_exports.test.ts:11-13
  now states the exclusion at the ruled boundary (the rows reach the face
  only by inference through `EmitResult`). `rg` over `sdk/` proves nothing
  reaches the retracted names through the root face. **One stale gate, now
  flipped:** MENTION-REACHABLE-PREDICATE was `blockedBy`
  SEAM-EXPORTS-RETRACT — the blocker shipped at e5de411 and the chore
  dropped it from the queue, so the entry's gate pointed at a tag no longer
  in pending. It is `open`, and its `sdk/src/index.ts` description no
  longer claims to serialize behind a shipped entry: the retraction cut
  lines 80+ and left the contract export block untouched, so `Predicate`
  (25) and `globValid` (35) still resolve and the closure walk still needs
  no edit. **One cite drifted, now corrected:** EDGE-TARGET-SET's two
  `sdk/test/emit.test.ts` fixture sites moved +2 (1085/1173 → 1087/1175)
  when e5de411 retargeted that file's import; `refusals.test.ts:220` is
  unmoved. Its `blockedBy` MENTION-REACHABLE-PREDICATE holds on the real
  reason — the shared `src/graph.rs`, `src/drift.rs`,
  `sdk/src/declarations.ts` — never the emit.test.ts overlap that is now
  spent. **One rider routing went stale and is now re-routed** — the audit's
  own instruction to re-test every "rides X" condition NOW, and the find of
  this tick: the `compose::effective` straggler (compose.rs:233) recorded "no
  queued entry opens `compose.rs`, so it has no carrier and waits", but
  EDGE-TARGET-SET widens `Edge.to` at 52 in that very file. It is the
  carrier, and the record's own twice-proven precedent says a carrier that
  merely opens the file discharges nothing — so the rider is named in
  EDGE-TARGET-SET's `files[].description`, where build reads it. The
  retirement re-checked clean first: no `fn effective` survives in
  compose.rs, and the `effective contract` hits in main.rs/drift.rs/install.rs
  are the ordinary English word, never the retired symbol. The other four
  riders re-tested the same way and each still has no carrier (read.rs,
  prose.ts, Cargo.toml, session_start.rs — no queued entry edits any).
  **Sweep:** the window is one retraction and leaves no residue — no
  retired symbol survives, no second implementation appears. Spec cursor NOT
  advanced and the reason is now settled rather than restated: 0c25b2c wears
  a `specs:` subject claiming the contract.md cite-drop, but that work landed
  in a571973 itself (its diff drops both `(decision 0029)` and `(decision
  0028)`), and 0c25b2c's actual one-line diff is a `.temper/lock.toml`
  re-emit (`templates = [{ kind = "supporting-doc", path = "*.md" }]`,
  SKILL-NESTED-REFERENCE-DOCS catching up) — a mis-messaged commit, not
  un-derived intent. `git log a571973..HEAD -- specs/` is empty: the delta is
  genuinely drained. One corpus-form violation surfaced, NOT filed — plan
  never writes `specs/` and a `specs:` commit is the session's by ceremony:
  `specs/model/pipeline.md:102` still carries `(decision 0024)` inline, the
  exact class a571973's review correction cut from contract.md, against
  spec-system.md/"Form rules" (body text never references decisions). Both
  parks re-tested at this HEAD and hold: `MAX_IMPORT_HOPS` reads 5 at
  graph.rs:62 under a cite claiming five, and the packaging park holds on
  every clause (four era tags and no version tag, crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 states the deferral). Every accepted-debt rider
  re-verified on disk and unmoved — the window touched none of their files.
- Queue: 1 pickable (MENTION-REACHABLE-PREDICATE, now unblocked); 2 blocked
  behind it (MENTION-REACHABLE-RULE-CLAUSE, EDGE-TARGET-SET — disjoint from
  each other, so both run beside each other once it lands); 2 parked on
  human acts (IMPORT-HOP-CAP-CITE: a hop-depth probe.
  PACKAGING-CHANNELS-REMAINDER: Apple notarizing + the v0.1 tag). No gate is
  stale — all tested this tick.

Plan continues: yes — **the inbox went live mid-tick and is NOT routed**.
Three notes landed in the working tree stamped `observed at eb2776d`, after
the snapshot this tick oriented off (which showed the inbox empty), from a
simulated consumer war game: (1) the plugin/marketplace manifest kinds are
identity-stated in CLAUDE.md/intent yet unshipped — needs the live plugin
docs fetched and cited before anything encodes, so it routes as an
investigation, never straight to build; (2) evidence-only datums for
`(multi-harness-projection)` and `(eval-capability)` — neither unparks;
(3) a vocabulary/UX long tail, all simulated demand, none queue-jumping,
carrying one genuinely fork-shaped tension: a lastReviewed-staleness clause
would make the gate read the clock, colliding with deterministic,
byte-reproducible `check`. They are committed here undrained rather than
left loose in the working tree, where a checkout would lose them. One tick
is one job and this tick's job was the reconciliation; the inbox is job 1
and is the next tick's, and its demand is simulated — every claim re-verifies
against the tree before it scopes anything. Build can take
MENTION-REACHABLE-PREDICATE in parallel; nothing in the inbox touches it.
