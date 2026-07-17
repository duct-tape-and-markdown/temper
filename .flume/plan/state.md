# Plan state

- Spec derived through: 832f015
- Audited through: 0c3cbcb
- Residue swept through: 0c3cbcb
- This tick: RECONCILE `385c429..0c3cbcb` — both motions over 7fae62e's ship,
  the window's only commit touching `src/`/`tests/`/`sdk/`.
  **Audit: the closed-keys widening shipped as scoped, and it discharged a
  hold the manifest kind had carried since it landed.** Verified on disk, not
  off the log: `Predicate::ClosedKeys` decodes at `contract.rs`:395 and judges
  at `engine.rs`:793 over `declared_keys` (`contract.rs`:687) — the allow-list
  read off the contract's own `required`/`optional` siblings, so the key set is
  declared once and no second copy can drift beside it; `optional`, a row
  nothing had read since it shipped, is load-bearing for the first time
  (`engine.rs`:714 holds it, `declared_keys` reads it). The refusal's mirror is
  on disk too: a `closed-keys` clause over no declared key is inadmissible
  (`engine.rs`:274), the empty-`forbidden_keys` rule's twin (288). The JSON
  Schema face is the object's — `additionalProperties: false` (`schema.rs`:122,
  emitted 196) with every declared key named as a property, an empty subschema
  where its only clause has no schema face (178-196), so the editor never
  squiggles a key the contract declares. `EXPECTED_LAG` did not move
  (`manifest_schema_oracle.rs`:40), exactly as the shipped oracle test
  predicted. `cargo test` green on disk (49 result lines, 762 passed, 0 failed).
  **The head entry's gate re-tested and opened.**
  SHAPE-PREDICATE rested on CLOSED-KEYS-CLAUSE alone; the ship commit (0c3cbcb)
  removed that entry, leaving the gate pointing at a tag no longer in the
  queue. It is now `open`, and the queue's one pickable entry — the fourth wave
  head in a row to open this way.
  **Cites re-stamped the same tick the window moved them.** `src/contract.rs`
  (~+9 to +13 — decoder 386→395 and its `"type"` arm 408-415, `key()` 534,
  `target()` 595, `documented_field()` 638; the enum head held at 132),
  `src/engine.rs` (`unaddressable` 168→176, `addressed_field` 189→197,
  `AllowedChars` 752→813, `declared_kinds` 920→981; `admissibility` held at
  110), `src/schema.rs` (match 80→83, no-op list 135-154,
  `addresses_a_property` 199→222), `sdk/src/contract.ts` (`type` ctor ~+2, the
  node-scope header rewritten), `sdk/src/builtins.ts` (skill header ~+122, to
  1058-1061; `allowedChars` name clause 1082), `tests/manifest_schema_oracle.rs`
  (no-op list 161-180→165-185), `src/roster.rs` (the orphan cite 469→473).
  Verified *unmoved* by empty `git diff` rather than assumed: `src/drift.rs`,
  `src/compose.rs`, `src/reporter.rs`, `src/kind.rs`, `src/builtin_kind.rs`,
  `src/main.rs`, `src/graph.rs`, `tests/graph.rs`, `sdk/src/generated/`,
  `.github/` — so DIAL-KIND's, CHECK-ANNOUNCES', and the hop-cap park's every
  address carries from last tick's re-read rather than being re-derived.
  **One entry gained a rider, and it changed hands inside the window.**
  7fae62e discharged `pluginManifestDefaultContract`'s hold outright — its
  header now reads "The whole profile ships … nothing decidable is held" — and
  in doing so falsified `marketplaceDefaultContract`'s header (958-960), which
  still points at "the same shape of hold `pluginManifestDefaultContract` names
  for its own two". The sentence the queue had been tracking on the plugin side
  is gone; its stale twin is the marketplace's, and it rides SHAPE-PREDICATE,
  the one queued entry that opens `sdk/src/builtins.ts`. SHAPE-PREDICATE's
  `engine.rs` rider sharpened rather than moved: `addressed_field` ends in a
  catch-all `_ => None` (211), so omitting the shape arm compiles clean and
  leaves the path un-fenced with no build error — the omission the compiler
  cannot catch. 7fae62e's own body is the precedent for stating it: `closed-keys`
  names no field, and it says so rather than letting the omission read as a miss.
  **Sweep: clean, and one near-duplicate confirmed rather than assumed.**
  The window minted `FieldPath::head_name` (`src/address.rs`) — the top-level
  key a path is rooted at. `rg` for a second head-segment walker across `src/`
  + `sdk/src/` finds one line, `drift::collection_key_of` (1366), and it is the
  same line the last sweep cleared: it takes the head of a manifest *collection
  address* label (`hooks.<Event>` ⇒ `hooks`), a registration's serialization
  address, never a clause's field path. One line, one domain, no second walker.
  Nothing filed. Both parks re-tested on disk and hold: no version tag
  (`git tag -l` carries the four era tags alone), crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 states the darwin + channel-3 deferral verbatim;
  `MAX_IMPORT_HOPS` is still 5 (`graph.rs`:59) and its doc still asserts a
  five-hop cap its own source does not say. One record moved, none resolved:
  `(source-union-predicate)`'s prediction played out a second time — a sibling
  contract's last hold retired and this bullet stayed, now the marketplace's
  only hold and, once SHAPE-PREDICATE lands, the last hold in the provider
  face. The stale-cite record holds at one orphan and gained its sharpest
  datum: 7fae62e opened `src/roster.rs` — the first entry ever to — and left
  the cite standing, because the entry was never given it.
- Queue: 5 entries, **1 pickable** — SHAPE-PREDICATE. Two chain behind it,
  serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at 0c3cbcb with the window's audit and sweep complete. Build
takes over: SHAPE-PREDICATE is pickable, carrying 0033's fourth widening, with
the dial chain behind it, none carrying unbuilt upstream.
