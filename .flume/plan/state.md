# Plan state

- Spec derived through: b8396d4
- Audited through: 4084dcd
- Residue swept through: 4084dcd
- This tick: POST-SHIP RECONCILIATION of `14c5de4..4084dcd` (f8d2431
  `json-document` write-face dispatch, c68f625 `plugin-manifest`) — both
  motions, both cursors advanced. Inbox empty and `.flume/refactor/` at its
  README alone, so job 3 was the first live input.
  **Audit.** Both ships verified on disk, not from the log: `project_bytes`
  routes `Some(Format::JsonDocument)` to `json_manifest::write_document`
  (`src/drift.rs:1740`) with `format_from_row` lifted beside `content_from_row`
  (`src/kind.rs:882`); `claude_code_plugin_manifest()` is in `all_kinds()` and
  `pluginManifestDefaultContract` ships four cited clauses. 4084dcd had already
  dropped both entries, so the drop motion was pre-paid — but **its cite
  re-derivation was not, and one claim was false**: MARKETPLACE-KIND asserted
  `all_kinds()` "315, re-verified unmoved at 4084dcd" when c68f625 had pushed it
  to **344** by inserting the sibling above it; the `builtins.ts` header cite
  (605-620) is really **599-627**. Both rewritten, and the entry now says the
  number moves whenever a sibling lands. The `kinds.len()` assert **is** 10 at
  2113 as claimed (a second assert at 1572 is a one-kind fixture, not it). All
  four gates re-tested: BUNDLE-EMIT-THROUGH-KINDS still blocked (no
  `claude_code_marketplace`); both parks hold on every clause — `MAX_IMPORT_HOPS`
  still 5 at `graph.rs:65` under a cite claiming five, nothing ruled the hop
  semantics, that window touched no import mechanics; four era tags and no
  version tag, crate 0.1.0 vs npm 0.0.7, release.yml:7-9 states the deferral
  verbatim.
  **Sweep — the motion nothing had run, and it found a live defect.**
  **JSON-DOCUMENT-BODY-REFUSAL** filed: `project_bytes`'s JSON arm renders
  `fields` and never reads `body`, so a `json-document` member's authored prose
  is **silently dropped** — invariant 3 (never drops authored words) and
  invariant 6 (loud or nothing) break on the same arm. Reachable today, not
  theoretical: `pluginManifest` declares no `shape`, so it is body-bearing
  (`CustomKind::new` defaults `Content::File`), and `MemberInit` carries
  `prose?` unconditionally (`sdk/src/kind.ts:197`) — the SDK type refuses
  nothing. The include path is the vivid one: `splice_includes` resolves the
  include and pushes its row into the lock, then the spliced bytes are dropped —
  a fingerprinted dependency whose content reaches no artifact. This is f8d2431's
  own blind spot: that tick argued `placements` cannot exist over JSON bytes *by
  construction* (sound — `placement_lines` matches only `---\n` or a banner) and
  never made the same argument for `body`, which has none. `per` cites
  `specs/intent.md` "Invariants"; `blockedBy` TYPE-PREDICATE-ROUND-TRIPS on
  `src/drift.rs` alone. Two candidate fixes were weighed and one is not real:
  declaring the kind `shape: "fields"` buys **no** type-level refusal
  (`MemberInit.prose?` does not vary on `shape`), so the engine-side loud refusal
  is the fix, sited where a `Result` can fire before a byte is written.
  Swept clean otherwise: the window introduced no retired vocabulary, and
  `bundle`'s `write_json` already delegates to `json_manifest::write_manifest`
  — the JSON **encoder** has one home, so BUNDLE-EMIT-THROUGH-KINDS's target is
  narrowed on disk to the hand-built manifest **values**, not the writer. The
  corpus/code gap on `plugin-manifest`'s `--strict` contract is routed by
  `(closed-surface-predicate)`, not re-filed. All three standing debt riders
  re-verified unmoved and restamped to 4084dcd (`session_start.rs` 121/140,
  `prose.ts`'s ten lines, `Cargo.toml`'s cite at 42); none has a carrier.
  Spec cursor copied forward verbatim — this tick derived nothing.
- Queue: 6 entries — 2 pickable and file-disjoint (MARKETPLACE-KIND in
  `src/builtin_kind.rs`+`src/builtin_lock*`+`sdk/`+`tests/`;
  TYPE-PREDICATE-ROUND-TRIPS in `sdk/src/contract.ts`+`declarations.ts`
  +`generated/ClauseRow.ts`+`src/drift.rs`+`src/contract.rs`+`src/extract.rs`),
  2 serialized (JSON-DOCUMENT-BODY-REFUSAL behind TYPE-PREDICATE-ROUND-TRIPS on
  `src/drift.rs`; BUNDLE-EMIT-THROUGH-KINDS behind MARKETPLACE-KIND on
  `src/bundle.rs`), 2 parked on human acts. No file appears in two `open`
  entries — checked mechanically.

Plan continues: no — every input below the serviced job is dead. Inbox empty,
`.flume/refactor/` at its README, spec delta empty (`Spec derived through:`
b8396d4 is the last `specs:` commit), and both reconciliation cursors now read
4084dcd, the tip of the last build window. Build takes over: two entries are
pickable, file-disjoint, and their cites were re-derived on disk this tick.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
