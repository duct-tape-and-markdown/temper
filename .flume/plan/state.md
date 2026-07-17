# Plan state

- Spec derived through: 832f015
- Audited through: 385c429
- Residue swept through: 385c429
- This tick: RECONCILE `db85b0f..385c429` — both motions over aaf70f1's ship,
  the window's only commit touching `src/`/`tests/`/`sdk/`.
  **Audit: the addressing widening shipped as scoped, and it opens the wave's
  head.** Verified on disk, not off the log: a clause's `field` is an
  addressing path (`src/address.rs`, new — `FieldPath::parse` is both the
  parser and the admissibility gate, and `serde_json_path` is the hidden
  engine, never a hand walk); the declared subset is *enforced* rather than
  merely stated — `unaddressable` (`src/engine.rs`:168) refuses a filter,
  slice, index, descent, and quoted name, and the presence rule beside it
  (`required("plugins[*]")` names elements, so no key of it can be absent)
  fires from the same fence; `marketplaceDefaultContract` now spells
  `owner.name` and `plugins[*].source`, so two of its three holds discharged.
  `cargo test` green on disk (48 result lines, 749 passed, 0 failed).
  **The head entry's gate re-tested and opened.**
  CLOSED-KEYS-CLAUSE rested on FIELD-ADDRESSING-RFC-9535-SUBSET alone; the
  ship commit (385c429) removed that entry, leaving the gate pointing at a tag
  no longer in the queue. It is now `open`, and the queue's one pickable
  entry — the third wave head in a row to open this way.
  **Cites re-stamped the same tick the window moved them**, and the movement
  was wide: `src/contract.rs` (~+14 to +18 — enum head 118→132, `Required`
  138-141, `Optional` 144-147, decoder 386, `key()` 523, and the two further
  exhaustive matches a new variant must answer, `target()` 583 and
  `documented_field()` 625), `src/engine.rs` (~+52 — `admissibility` 110
  unmoved, `Optional` 676, empty-`forbidden_keys` 265, empty-type-set 271,
  `ForbiddenKeys` 739, `AllowedChars` 752, `declared_kinds` 920),
  `src/schema.rs` (~+16 — match 80, no-op list 127-146), `sdk/src/contract.ts`
  (~+11 to +14 — node-scope block 83, `type` ctor 93-97), `sdk/src/builtins.ts`
  (skill header 923-929→936-941, `allowedChars` name clause 962, while the
  plugin-manifest hold 609-614 and its count sentence 606-607 held still),
  `tests/graph.rs` (~-8: the hop-cap test 1357→1349). Verified *unmoved*
  rather than assumed: every `src/graph.rs` park address (55-59, 519, 618-620,
  643-646, 648) despite a 40-line edit to that file; `all_kinds`
  (`src/builtin_kind.rs`:374-388, still eleven) despite a 15-line edit; and by
  empty `git diff`: `src/drift.rs`, `src/compose.rs`, `src/reporter.rs`,
  `src/kind.rs`, `tests/manifest_schema_oracle.rs`, `sdk/src/index.ts`,
  `.github/`.
  **Two entries gained a rider from the window, neither rescoped.** The window
  split the value predicates into two categories a new one must pick between,
  and each queued widening lands on a different side: SHAPE-PREDICATE carries a
  `field`, so it must join `addressed_field` (`engine.rs`:189) or its path goes
  unparsed and the subset bound goes unenforced on one predicate — the one
  address aaf70f1 added to its scope. CLOSED-KEYS-CLAUSE names no field, so it
  lands on the key side the enum's own head doc (`contract.rs`:126-128) now
  draws beside `forbidden_keys`, joins no path machinery, and its commit body
  says so rather than letting the omission read as a miss.
  **Sweep: clean, and one consolidation confirmed rather than assumed.**
  `extract::resolve_key_path` — the hand-rolled dotted key-path walk aaf70f1's
  body names as retired — is gone from `src/`, `tests/`, and `sdk/` (`rg`
  returns nothing), so path walking has one home and it is the RFC engine's.
  The one surviving `split('.')` in `src/` is `drift::collection_key_of`
  (1366), and it is not the same job: it takes the head segment of a manifest
  *collection address* label (`hooks.<Event>` ⇒ `hooks`), a registration's
  serialization address, never a clause's field path — one line, one domain,
  no second walker. Nothing filed. Both parks re-tested on disk and hold: no
  version tag (`git tag -l` carries the four era tags alone), crate 0.1.0 vs
  npm 0.0.7, release.yml:7-9 states the darwin + channel-3 deferral verbatim;
  `MAX_IMPORT_HOPS` is still 5 (`graph.rs`:59) and its doc still asserts a
  five-hop cap its own source does not say. Two records moved, neither
  resolved: `(source-union-predicate)`'s prediction played out exactly — the
  widening shipped, the bullet stayed, and it is now the marketplace contract's
  only hold (cite 849-851→838-847); the stale-cite record drops from two lines
  to one — aaf70f1 carried the `Cargo.toml` riders it was given and landed
  both, leaving `src/roster.rs`:469 (470→469) the last orphan of its class,
  still waiting for a carrier.
- Queue: 6 entries, **1 pickable** — CLOSED-KEYS-CLAUSE. Three chain behind it,
  serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at 385c429 with the window's audit and sweep complete. Build
takes over: CLOSED-KEYS-CLAUSE is pickable, carrying 0033's third widening,
with two wave entries and the dial chain behind it, none carrying unbuilt
upstream.
