# Plan state

- Spec derived through: b8396d4
- Audited through: 18f219d
- Residue swept through: 18f219d
- This tick: POST-SHIP RECONCILIATION of `4084dcd..18f219d` (c74aab9
  `marketplace`, c7bd4f3 the `type` predicate's lock crossing) — both motions,
  both cursors advanced. Inbox empty and `.flume/refactor/` at its README
  alone, so job 3 was the first live input.
  **Audit.** Both ships verified on disk, not from the log: `claude_code_marketplace()`
  (`src/builtin_kind.rs:353`) is in `all_kinds()` (374-382) and the lock's
  `kinds.len()` assert reads **11** at `tests/lock_declaration_rows.rs:2135`
  (ten roster kinds plus `supporting-doc`, exactly as c74aab9's body argues);
  `type` carries its declared kind end to end — `value_type` column
  (`drift.rs:2505`), `predicate_from_row` arm (`contract.rs:332-334`) decoding
  via `ValueType::from_name`, SDK `type(field, kind)` (`contract.ts:74`).
  18f219d had already dropped both entries, so the drop motion was pre-paid.
  **Both blockers cleared, and both dependents opened.** JSON-DOCUMENT-BODY-REFUSAL
  (was `blockedBy` TYPE-PREDICATE-ROUND-TRIPS on `src/drift.rs`) → `open`:
  c7bd4f3's drift.rs edits landed at 2445+/3337+, clear of every cite it holds.
  Its cites were **re-derived and two were wrong when filed** — `format_from_row`
  reads 989 not 988, the `splice_includes` call 982 not 983 (both unmoved across
  the window; the entry was off by one at 826f06d). BUNDLE-EMIT-THROUGH-KINDS
  (was `blockedBy` MARKETPLACE-KIND) → `open`; every `src/bundle.rs` cite
  (178/185/191/223/238/259/278) re-verified unmoved. Both parks hold on every
  clause: `git diff 4084dcd..HEAD -- src/graph.rs tests/graph.rs` is **empty**,
  `MAX_IMPORT_HOPS` still 5 at 65 under a cite claiming five, nothing ruled the
  hop semantics; four era tags and no version tag, crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 states the deferral verbatim.
  **Sweep — the window declared its own remainder and nothing carried it.**
  **TYPE-CLAUSE-CONSUMER** filed: `type` now round-trips, and **no shipped
  contract uses it** — `rg '\btype\(' sdk/src/builtins.ts` matches prose only.
  `pluginManifestDefaultContract`'s header (614-618) still asserts the predicate
  "takes no declared kind, and the clause row carries no column for one" —
  **false at HEAD**, and the same bullet names its own consumer and calls the
  rule decidable and documented (`keywords` a string is "a load error
  everywhere, not merely under `--strict`"). `src/builtin_lock.toml`'s header
  (42-48) narrates the same retired hold. This is c7bd4f3's declared out-of-scope
  corner ("that file is MARKETPLACE-KIND's") which c74aab9 correctly did not
  poach — an honest handoff with no catcher, so plan filed it. Not comment
  staleness: the missing **clause** is the entry; the header moves with it.
  `per` cites `specs/builtins.md` "Default contracts"; `tests/type_predicate.rs:35`
  already proves the exact mechanism (`type("keywords", "list")`) over a custom
  kind — the shipped kind whose real field motivated it stays ungated.
  **`(nested-field-addressing)` registered** — MARKETPLACE-KIND shipped a wide
  corner cut (its own `.flume/friction/` capture): a clause addresses a **flat**
  top-level key and `json_to_feature` (`src/extract.rs:916-926`) discards inner
  object keys and stringifies array elements, so `owner.name`, `plugins[]`'s
  `name`/`source`, and the `source` union have no clause — a catalog Claude Code
  refuses outright passes `check` clean. Forked, not filed: `specs/builtins.md`
  sanctions only "undecidable, deliberately absent", and **three** contracts now
  carry a *decidable-but-unexpressible* hold the corpus does not name. Naming
  that category is a corpus act (`model/contract.md`, "clause"), never a build
  tick's. Kin to `(closed-surface-predicate)`; may settle as a pair.
  `(plugin-author-dogfood)`'s blocked-in-fact clause **discharged** — all three
  0031 kinds ship, verified in `all_kinds()`; only the human ruling holds it.
  `(closed-surface-predicate)` corrected: `Predicate::Optional` is at
  `contract.rs:89`, not 88 (unmoved across the window — wrong when written).
  All three standing debt riders re-verified unmoved and restamped to 18f219d
  (`session_start.rs` 121/140, `prose.ts`'s ten lines, `Cargo.toml`'s cite at
  42); none has a carrier. Spec cursor copied forward verbatim — this tick
  derived nothing.
- Queue: 5 entries — 3 pickable and file-disjoint (TYPE-CLAUSE-CONSUMER in
  `sdk/src/builtins.ts`+`src/builtin_lock.toml`+`sdk/test/`+`tests/plugin_manifest_kind.rs`;
  JSON-DOCUMENT-BODY-REFUSAL in `src/drift.rs`+`tests/json_document_format.rs`;
  BUNDLE-EMIT-THROUGH-KINDS in `src/bundle.rs`+`tests/bundle.rs`), 2 parked on
  human acts. No file appears in two entries — checked mechanically.

Plan continues: no — every input below the serviced job is dead. Inbox empty,
`.flume/refactor/` at its README, spec delta empty (`Spec derived through:`
b8396d4 is the last `specs:` commit), and both reconciliation cursors now read
18f219d, the tip of the last build window. Build takes over: three entries are
pickable, file-disjoint, and every cite was re-derived on disk this tick.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
