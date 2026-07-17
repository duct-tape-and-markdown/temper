# Plan state

- Spec derived through: 832f015
- Audited through: da31f82
- Residue swept through: da31f82
- This tick: RECONCILE `832f015..da31f82` — both motions over bce89b7's ship,
  the window's only commit touching `src/`/`tests/`/`sdk/`.
  **Sweep: clean.** 0034's named demolition drained whole — `rg "layout-only"`
  over `src/`, `tests/`, `sdk/src/` returns nothing, and the fence's narration
  went with the arm (`local_locus_fault`'s doc, `local_locus_admissibility`'s
  doc, the `gate` call-site comment, `tests/local_locus.rs`'s module header).
  Nothing filed; no second implementation surfaced.
  **Audit: one finding, and it was a gate reason, not code.**
  LOCK-FAMILY-ASSEMBLED-ONCE re-verified on disk — all three claims hold
  (`embedded_features_by_kind` 1555 folding `declarations.nested_members`
  alone, call sites 523/913, `kind_features` 1364 landing the `nested` half
  only at 1384, the false-green severity at `tests/local_locus.rs`:297). It
  stays pickable.
  TOML-DOCUMENT-READ-FACE's gate reason was **false and is retired**: last tick
  wrote that the lock-family join "subsumes the `local_document_rows` format
  match this face answers". It does not — the match sits *inside*
  `local_document_rows` (1412-1418), which the join keeps; the join retires the
  two *commitment* branches at that function's call sites (1229, 1377), a
  different surface. The gate stands on serialization alone (both edit
  `src/main.rs`), so the reason is re-stated, not the gate dropped. The
  correction became scope the entry lacked: bce89b7 left that
  `(content, format)` match exhaustive with **no catch-all**, so
  `Format::TomlDocument` is a compile error until the entry picks its arm —
  now named in its `src/main.rs` description with the doc's own decision rule.
  **Cites re-stamped, re-read on disk, never carried:** bce89b7 moved four
  `src/kind.rs` addresses under two entries citing them —
  TOML-DOCUMENT-READ-FACE (`Format` 555→554, `label` 571→570,
  `format_from_label` 975→964, doc 551-554→548-552) and DIAL-KIND
  (`CustomKind::local` 729→728, which last tick's own re-stamp missed).
  Both parks re-tested and hold — the window touches neither `src/graph.rs` nor
  `.github/`; `MAX_IMPORT_HOPS` is still 5, `git tag -l` carries only the four
  era tags, release.yml:7-9 states the deferral verbatim. The
  `src/roster.rs`:470 orphan cite still waits for a carrier, unmoved.
- Queue: 12 entries, **1 pickable** — LOCK-FAMILY-ASSEMBLED-ONCE. Nine chain
  behind it, serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at da31f82 with the window's audit and sweep complete. Build
takes over: LOCK-FAMILY-ASSEMBLED-ONCE is pickable, and nine entries queue
behind a derivation that today fails open.
