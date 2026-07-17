# Plan state

- Spec derived through: 16ce347
- Audited through: b3a1636
- Residue swept through: b3a1636
- This tick: SPEC DELTA — derive 16ce347 (`0031 — a plugin registers by
  enablement`), the first live input (inbox empty, `.flume/refactor/` its
  README alone, and no `src/`/`tests/`/`sdk/` commit past the audit cursor:
  the only commit in that window is this tick's own predecessor, `plan:`
  e81c758). **The roster grows seven → ten, filed as a five-entry chain**,
  every surface `rg`'d or read at e81c758 rather than trusted. Ripple modelled
  on a7a8cc1, the last kind addition (9 files, +303): `builtin_kind.rs` +
  `builtin_lock.toml` (generated, never hand-edited) + `builtin_lock.rs` +
  `builtins.ts` + `claude-code.ts` + the two lock tests, plus a per-kind
  acceptance file. **The chain is serialized because everything shares
  `builtin_kind.rs`/`builtins.ts`/`kind.rs`, not because the work is
  sequenced**: INSTALLED-PLUGIN-KIND (open) → PLUGIN-JSON-DOCUMENT-FORMAT →
  PLUGIN-MANIFEST-KIND → MARKETPLACE-KIND → BUNDLE-EMIT-THROUGH-KINDS.
  **Three findings the derivation turned up, each carried into the entry that
  needs it rather than left for build to re-pay:** (1) `Format` is a declared
  fact **nothing branches on** — `rg 'Format::YamlFrontmatter'` finds only
  constructions, the label parse, and one test assert; the adapter dispatch
  (`main.rs:1160`) keys on `(content, collection_address, governs)` and falls
  every file kind through to `read_file_unit` regardless of format, so
  `json-document` is what makes the fact load-bearing and `main.rs` is that
  entry's heart. (2) `installed-plugin` sharing hook's `.claude/settings.json`
  governs glob is **not** a collision: `governs_collision_diagnostics`
  (`main.rs:1801`) skips every collection-address kind (1812/1824) and its
  header states the rule — the mining representation.md forbids is two
  *document* kinds contending for one file. (3) **The docs contradict
  themselves on the `enabledPlugins` key charset** — plugins-reference
  documents `<plugin>@<marketplace>` while schemastore's
  `claude-code-settings.json` patterns `^[a-zA-Z0-9_-]+(?:/[a-zA-Z0-9_-]+)?$`,
  admitting no `@` — so the entry forbids a key-shape clause: encoding one
  against either spelling forges findings on valid harnesses (invariant 2).
  The value type is settled and agreed (boolean), so the channel-gate field
  stands. **Consequences checklist, all four bullets routed** — evidence in
  the commit body. **`src/read.rs`'s five-line rider found its carrier**:
  INSTALLED-PLUGIN-KIND edits the file and names the lines. Both other cursors
  copied forward verbatim: this tick audited and swept nothing.
- Queue: 8 entries — 2 pickable and disjoint (EXAMPLE-EDGE-TARGET-SET-SPELLING
  in `examples/**`; INSTALLED-PLUGIN-KIND in `src/`+`sdk/`+`tests/`), 4
  serialized behind it on shared files, 2 parked on human acts (a hop-depth
  probe; Apple notarizing + the v0.1 tag). Both parks re-tested at e81c758 and
  hold. No file appears in two `open` entries.

Plan continues: no — the delta is drained (16ce347 was its only commit, now
fully routed), the inbox is empty, `.flume/refactor/` holds its README alone,
and no code commit sits past the audit cursor. Build takes over: two entries
are pickable. **One thing for a human, unchanged and not the loop's:**
decision 0030 is still a hole — `specs/decisions/` runs 0023…0029, 0031, and
0030 (`review is the price of softening`) is orphaned at d6381b4, reverted by
this phase's own `continuation marker is honest` gate firing on a human
`specs:` commit. Recoverable via `git show d6381b4`; John's alone to restore;
the misfire is filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
