# Plan state

- Spec derived through: 832f015
- Audited through: 8fc5e21
- Residue swept through: 8fc5e21
- This tick: RECONCILE `a2e48aa..8fc5e21` — three build commits, a full wave
  (SCHEMA-DOCS-CHANNEL-ACCUMULATES 3451b98, ADDRESSED-FIELD-FENCE-EXHAUSTIVE
  7013a04, DIAL-KIND eaee2af). Both motions ran; both cursors advance.
  **Audit.** Verified on disk, not from the log. All three shipped and were
  already dropped from the queue: `push_description` joins a field's guided
  clauses into one `description` (`schema.rs`:337); `addressed_field`
  (`engine.rs`:201) now names every arm with no wildcard, its doc stating the
  bound; the dial ships whole — `src/dial.rs`, `tests/dial_kind.rs` (13 tests),
  `sdk/src/dial.ts`, `temper_dial()` (`builtin_kind.rs`:388), and the
  `builtin_lock.toml` rows. No pending entry's work is done.
  **The wave's payoff is the queue's, not just the tree's: DIAL-KIND was
  CHECK-ANNOUNCES's blocker, so the last of 0032's five is now pickable** —
  re-tested rather than assumed, and `rg announce src/ tests/` is empty, so its
  work is genuinely undone. Its premise moved in its favour and the entry is
  rewritten rather than patched: last tick two of its three inputs were
  assembled and the joined lock was the built-and-unannounced gap; now **all
  three are built and unannounced** — the dial ship put `dial` on `LockFamily`
  (1491) and accumulates `dialed` (830) across four `apply` sites, final at
  `refusals` (1061). So the entry is announcement with no re-derivation, with
  one honest exception the rewrite names: `assemble_lock_family` (1512)
  resolves each local kind's units and keeps only the derived rows, dropping
  the unit ids — the local-member third needs them retained, never a re-walk.
  Every address re-derived on disk; the dial's +103 lines moved all of them
  (`LockFamily` 1426→1474, `assemble_lock_family` 1457→1512,
  `qualify_layer_label` 1546→1629). `src/reporter.rs` is untouched, so its
  three reporter addresses carry.
  Both parks re-tested against disk, both hold: IMPORT-HOP-CAP-CITE's subject
  is untouched — nothing ruled the hop semantics and `graph.rs`:59 still reads
  5; PACKAGING holds on every clause — `git tag -l` carries the four era tags
  and no version tag, crate 0.1.0 vs npm 0.0.7, `release.yml`:7-9 states the
  deferral verbatim, `git diff a2e48aa..8fc5e21 -- .github/` is empty.
  **Sweep — nothing filed, and that is the finding.** The window's own diff
  offered a candidate and it did not survive its own strongest objection: the
  dial ship hand-added `dial.apply` to three parallel contract sites (886, 932,
  997), which pattern-matches last tick's ADDRESSED-FIELD-FENCE filing. It is
  **not** the same disease. Those three sites *call* two existing one-home
  surfaces — `compose::with_joined_clauses` and `dial::apply` — rather than
  reimplementing either, so "one job, one home" (`engineering.md`) is already
  honored; the real complaint would be forgettability, which is a different
  claim that section does not carry, and a six-param extractor to save one line
  per site is likely worse than the disease. Plan does not invent intent its
  source lacks, so it goes unfiled rather than filed weakly.
  The sweep's one real catch is **ride-only by rule**: `main.rs`:1047 calls the
  selection loop "The second and last dial site" while four `apply` sites
  exist. Handed to CHECK-ANNOUNCES in scope — it opens the file and reads the
  very `dialed` set that comment governs — which is the fourth payout of the
  ride-only rule and the first on a cite the sweep itself surfaced.
  **Fork board.** `(settings-local-kind)` sharpened, not promoted: its own
  framing said "beyond the dial itself", and the dial itself now ships, so a
  `settings.local.json` kind would be the second instance of a live pattern
  rather than the first of an untried one. Ship-or-not is still a human's.
  `(source-union-predicate)` holds as the provider face's last hold, re-read at
  8fc5e21 — with its verification handle corrected: the record said to grep
  "pending a vocabulary addition" at `builtins.ts`:960, but that sentence wraps
  959-960 and a one-line grep finds nothing. The record now says so, or a later
  tick reads a live hold as discharged. `src/roster.rs`:473 remains the
  ride-only class's last orphan, re-read at 8fc5e21; no queued entry opens that
  file, so it waits.
- Queue: 3 entries, **1 pickable** — CHECK-ANNOUNCES-THE-LOCK-FAMILY, newly
  unblocked, alone on `src/main.rs` + `src/reporter.rs`. Two parked. No entry
  rests on a fork.

Plan continues: no — every input is serviced. Inbox empty, no refactor
captures, spec delta empty (cursor at 832f015 with no `specs/` commit past
it), and `a2e48aa..8fc5e21` is reconciled on both motions with both cursors
advanced. Build takes over: one pickable entry, the last of 0032's five.
