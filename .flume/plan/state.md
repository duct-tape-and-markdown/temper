# Plan state

- Spec derived through: 832f015
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: RECONCILE `8fc5e21..b85df4a` — one build commit
  (CHECK-ANNOUNCES-THE-LOCK-FAMILY, dab85aa). Both motions ran; both cursors
  advance. **0032's five are now all shipped.**
  **Audit.** Verified on disk, not from the log. The entry shipped whole and
  build had already dropped it: `check::Announcement` (`check.rs`:146) is
  assembled once at `main.rs`:1070, past the last dial site, and all four
  reporters carry it in their own format's shape — terminal
  (`check::render`:214), github (one `::notice` per input, `reporter.rs`:173),
  SARIF (run property bag, 215), session-start (74). `cargo test`: 51 suites,
  zero failures. No pending entry's work is done.
  **The entry's one honest exception is honestly closed.** It was scoped
  naming a gap — `assemble_lock_family` resolved each local kind's units and
  kept only the derived rows, dropping the unit ids the local-member third
  needs — and required retention over a re-walk. Disk agrees: `LockFamily`
  now carries `local_members` (1506) and `joined_locks` (1501) beside the rows
  they produced, each doc'd with the reason (a second glob walk "could
  disagree with this one about which members exist"). Nothing re-derived, as
  scoped.
  Both parks re-tested against disk, both hold. IMPORT-HOP-CAP-CITE: nothing
  ruled the hop semantics, `graph.rs`:59 still reads 5 under a doc still
  asserting five hops, and `src/graph.rs` + `tests/graph.rs` are untouched
  across `0c3cbcb..b85df4a` — so every address carries and the entry is
  restamped, not rescoped. PACKAGING holds on every clause: `git tag -l`
  carries the four era tags and no version tag, crate 0.1.0 vs npm 0.0.7,
  `release.yml`:7-9 states the deferral verbatim, `git diff
  8fc5e21..b85df4a -- .github/` is empty.
  **Sweep — nothing filed, and that is the finding, for the second tick
  running.** The window's own diff offered one candidate and it did not
  survive its own strongest objection: `src/reporter.rs` encodes GitHub's
  workflow-command grammar from memory with no source or retrieved date
  anywhere in the file — `::notice` semantics (158-173), and the escape split
  `escape_data` (265: `%`, CR, LF) vs `escape_property` (274: additionally
  `:` and `,`), each asserting "per GitHub's rules". It reads like the cite
  class IMPORT-HOP-CAP-CITE is parked over. It is **not** the same disease,
  on two counts. First, the corpus scopes the obligation: spec-system.md's
  "Form rules" binds an external fact's cite to **where it is enforced** —
  "a clause's `cite`" — and the reporter enforces nothing; it renders
  temper's own output channel and never judges an artifact against an
  external format. Second, the blast radii do not compare: a wrong hop cap
  forges or suppresses a finding (intent.md invariant 2), while a wrong
  escape produces an ugly log line. It is pre-existing besides — cite-free
  at 8fc5e21, and dfba26f stripped spec cites repo-wide by hand. Filing it
  would read the rule past its own text, so it goes unfiled rather than
  filed weakly.
  **Fork board.** `(settings-local-kind)` sharpened again, still not
  promoted: the local-locus pattern's last leg closed this window — a local
  member is now read, gated, *and* named in the verdict — so a
  `settings.local.json` kind would inherit the announcement free, with no
  surface of its own. Ship-or-not is still a human's.
  `(source-union-predicate)` holds as the provider face's last hold; `sdk/`
  is untouched across the window, so its addresses carry unre-read.
  The ride-only class **paid out a fourth time, and it landed**:
  `main.rs`:1047's "The second and last dial site" (loose against four
  `apply` sites) was handed to CHECK-ANNOUNCES in scope, and dab85aa carried
  it — the comment now names the axis, not a count. Four payouts running, and
  the rule's condition has never failed: every discharge came from an entry
  that NAMED the cite. `src/roster.rs`:473 remains the class's last orphan,
  re-read at b85df4a; no queued entry opens that file, so it waits. The
  retired-format fixtures took their **fourth** non-touch (dab85aa edited
  only lines 17 and 394 of `tests/session_start.rs`), unshifted at 113/122/141.
- Queue: 2 entries, **0 pickable** — both parked on human acts (a hop-semantics
  probe; Apple notarizing + the v0.1 tag). No entry rests on a fork.

Plan continues: no — every input is serviced. Inbox empty, no refactor
captures, spec delta empty (cursor at 832f015 with no `specs/` commit past
it), and `8fc5e21..b85df4a` is reconciled on both motions with both cursors
advanced. Build cannot take over: the queue is parked end to end, so the loop
hibernates until a human rules a hop cap or fires a release. **This is the
queue's floor, not a stall** — 0032's wave is complete and nothing derivable
is left standing.
