# Plan state

- Spec derived through: b8396d4
- Audited through: 14c5de4
- Residue swept through: 14c5de4
- This tick: INBOX — the one live capture drained, `.flume/refactor/` back to
  its README alone. The prompt's `<refactor-captures>` block failed to render
  (an `exec-failed` shell probe), so the capture was invisible in context and
  the tick looked like post-ship reconciliation; `ls` found it, and the inbox
  job outranks the audit. **`build-type-predicate-cannot-cross-the-lock`
  (observed 024ba9b) verified true at HEAD on every home**, diffing forward
  over its three commits: `sdk/src/contract.ts:68` exports `type =
  (field: string)` with no declared kind; `sdk/src/generated/ClauseRow.ts`
  carries a column per predicate argument and none for a `ValueType`;
  `predicate_from_row` (`src/contract.rs:321`) has an arm for every predicate
  in the closed vocabulary **except `type`**, so the row lifts to `None` and
  `src/compose.rs:181` rejects it loud — a refusal fired on a predicate the SDK
  told the author to use. The engine half is fully built and reachable only
  from Rust: `Predicate::Type` (97), `engine.rs:626` decides it,
  `schema.rs:64` projects it. `rg` confirms no `predicate = "type"` row exists
  anywhere (`builtin_lock.toml:274`'s `field = "type"` is mcp-server's `enum`
  over a field *named* `type` — not a counter-example). Filed as
  **TYPE-PREDICATE-ROUND-TRIPS**, `open` and disjoint from every other open
  entry. **Two scoping facts the capture did not carry**, found on disk: the
  lock's clause *writer* is `ClauseRow::to_table` (`drift.rs:3304`), a second
  face to thread since `ClauseRow` is `Deserialize`-only; and the lattice has a
  live **wire-spelling collision** — `ValueType::name()`/`from_name()`
  (`extract.rs:59`/`76`) spell it lowercase (`"list"`) while the ts-rs binding
  `sdk/src/generated/ValueType.ts` spells the variants capitalized (`"List"`),
  so the entry names it as the thing to settle first rather than letting build
  discover it at the decoder. **The `per` departs from the capture channel's
  default cite on purpose**: `.flume/refactor/README.md` says cite
  `engineering.md`, but there is no second implementation here — nothing is
  duplicated, one home is *missing* — so the entry cites
  `specs/model/contract.md` ("clause"), the section that actually owns "the
  enum in code is the authority". A false cite is worse than a departed
  default. **The capture's sibling gap is a fork, not an entry** — it says so
  itself: `--strict`'s allow-list over a closed key set needs the vocabulary to
  grow or `optional`'s dead rows to be read, and that is a
  `specs/model/contract.md` decision. Registered `(closed-surface-predicate)`.
  Two narration riders named onto the entry's `extract.rs` (it opens that
  file): `ValueType`'s doc parenthesizes "the slice-1 shortcut this entry
  corrects" — a build tick's voice leaked into shipped prose — and
  `FeatureValue`'s calls the primitive "(forthcoming)" (95), false once it
  round-trips. **Gate re-tested opportunistically** (cheap, and the queue must
  be honest to commit): PLUGIN-MANIFEST-KIND shipped at c68f625, verified on
  disk (`tests/plugin_manifest_kind.rs`, `pluginManifestDefaultContract`), so
  MARKETPLACE-KIND's `blockedBy` is discharged → `open`. Its stale arithmetic
  re-derived with it: the `kinds.len()` assert is **10 at 2113**, not 9 at 2093
  — the entry said "read it on disk", and this is why. All three cursors copied
  forward verbatim: this tick derived nothing and audited no window.
- Queue: 5 entries — 2 pickable and file-disjoint (MARKETPLACE-KIND in
  `src/builtin_kind.rs`+`sdk/`+`tests/`; TYPE-PREDICATE-ROUND-TRIPS in
  `sdk/src/contract.ts`+`declarations.ts`+`generated/ClauseRow.ts`+`src/drift.rs`
  +`src/contract.rs`+`src/extract.rs`), 1 serialized behind MARKETPLACE-KIND on
  `src/bundle.rs`, 2 parked on human acts (both re-tested at 4084dcd and both
  hold: nothing ruled the hop semantics and the window touched no import
  mechanics; four era tags and no version tag, crate 0.1.0 vs npm 0.0.7). No
  file appears in two `open` entries — `sdk/src/builtins.ts` and
  `tests/lock_declaration_rows.rs` are MARKETPLACE-KIND's alone, which is why
  the `type` predicate's consumer clause (`plugin-manifest`'s `keywords`) and
  that file's now-answerable hold paragraph are deliberately out of the wiring
  entry's scope.

Plan continues: yes — post-ship reconciliation. The window `14c5de4..4084dcd`
(two `build:` commits: f8d2431 `json-document` write-face dispatch, c68f625
`plugin-manifest`) is unaudited and unswept; both cursors still read 14c5de4.
This tick spent its one job on the inbox, which outranks it. The audit motion
is partly pre-paid — PLUGIN-MANIFEST-KIND and EMIT-JSON-DOCUMENT-WRITE-FACE
were both verified shipped on disk here, and 4084dcd already dropped them —
but the sweep motion is untouched: nothing has read that window's code against
the corpus. Build is not blocked meanwhile: two entries are pickable and their
cites are fresh as of this tick.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
