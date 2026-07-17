# Plan state

- Spec derived through: 832f015
- Audited through: db85b0f
- Residue swept through: db85b0f
- This tick: RECONCILE `80685db..db85b0f` — both motions over eccea26's ship,
  the window's only commit touching `src/`/`tests/`/`sdk/`.
  **Audit: the set widening shipped as scoped, and it opens the wave's head.**
  Verified on disk, not off the log: `Predicate::Type` carries
  `kinds: BTreeSet<ValueType>` (`src/contract.rs`:142-148), so a field the docs
  spell `string|array` is gated as the union it is rather than false-positived
  against half of it; the empty set joins the vacuous-clause refusals at
  `src/engine.rs`:222, beside the empty-`forbidden_keys` one at 216; the lock
  column widened to `Option<Vec<String>>` (`src/drift.rs`:2767) with a
  read-side bare-string tolerance (`opt_str_or_str_array`, 3355) — a skew the
  next emit rewrites whole, never a second spelling temper emits. `cargo test`
  green on disk (47 result lines, 0 failed).
  **The head entry's gate re-tested and opened.**
  FIELD-ADDRESSING-RFC-9535-SUBSET rested on TYPE-ACCEPTS-A-SET alone; the ship
  commit (db85b0f) removed that entry, leaving the gate pointing at a tag no
  longer in the queue. It is now `open`, and the queue's one pickable entry —
  the same shape last tick found, and the second wave head in a row to open
  this way.
  **Cites re-stamped the same tick the window moved them** — the six-ship lag
  last tick reported did not recur. eccea26 moved every file the chain cites
  but five: `src/contract.rs` (decoder 358→368, enum head unmoved),
  `src/engine.rs` (~+8: `Type` 618, `Optional` 611, `ForbiddenKeys` 687,
  `AllowedChars` 700), `src/schema.rs` (match 64, no-op list 110-120→111-130 as
  its commentary grew), `src/extract.rs` (~+5: `FeatureValue` 107, `List` 118,
  `Map` 121, `kind` 113), `sdk/src/builtins.ts` (~+36: marketplace header
  838-855, its three bullets 842-851, `required("owner")` 897,
  `required("plugins")` 903; plugin-manifest hold 609-614; skill header
  923-929), `sdk/src/contract.ts` (~+2), `src/drift.rs` (`value_type` 2767,
  written 3628-3629, read 3673), and the oracle (`covered_rule` 142, no-op list
  161-180, the strict-bar test 244, `EXPECTED_LAG` 40 unmoved but six rows
  lighter). Verified *unmoved* rather than assumed, by empty `git diff`:
  `src/compose.rs`, `src/main.rs`, `src/reporter.rs`, `src/kind.rs`,
  `src/builtin_kind.rs`, `src/graph.rs`, `src/roster.rs`, `Cargo.toml`,
  `sdk/src/index.ts`, `tests/graph.rs`, `.github/`.
  **Two entries gained a rider from the window, neither rescoped.**
  CLOSED-KEYS-CLAUSE's plugin-manifest hold is now that contract's **only**
  hold, not its first — eccea26 discharged the six component-path fields, so
  the header's own count sentence (606-607) retires with the bullet, and its
  cross-reference to `skillDefaultContract`'s two holds is what SHAPE-PREDICATE
  retires from the other end; whichever lands second carries the sentence.
  SHAPE-PREDICATE's `value_type` precedent sharpened rather than moved: the
  `"type"` decode arm (381-389) now spells the entry's own rule in its comment
  (376-380) — an unknown name is no predicate at all, a vacuous-but-decodable
  argument fails admissibility — while the column's new arity is *not* the half
  to copy, since a shape is one name.
  **Sweep: clean.** No second implementation — `declared_kinds`
  (`engine.rs`:855, `|`-joined lattice names for a finding's prose) and
  `json_types` (`schema.rs`:180, JSON Schema's own type vocabulary) are two
  codomains of one spelling home, `ValueType::name`/`from_name` in
  `extract.rs`; neither a second lattice spelling nor a second decoder was
  minted. Nothing filed. Both parks re-tested on disk and hold: no version tag
  (`git tag -l` carries the four era tags alone), crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 states the darwin + channel-3 deferral verbatim; the window
  touches neither `src/graph.rs` nor `tests/graph.rs`, so `MAX_IMPORT_HOPS` is
  still 5 and still uncited by its own source. The `src/roster.rs`:470 orphan
  cite still waits for a carrier — re-read, still 470. No fork record moved:
  the audit resolved none; `(source-union-predicate)`'s cite re-stamped
  813-816→849-851, and the widening it names is still unratified.
- Queue: 7 entries, **1 pickable** — FIELD-ADDRESSING-RFC-9535-SUBSET. Four
  chain behind it, serialized on shared files; no entry rests on a fork. Two
  parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at db85b0f with the window's audit and sweep complete. Build
takes over: FIELD-ADDRESSING-RFC-9535-SUBSET is pickable, carrying 0033's
second widening, with three wave entries and the dial chain behind it, none
carrying unbuilt upstream.
