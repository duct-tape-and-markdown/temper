# Plan state

- Spec derived through: 3c1a58c
- Audited through: c370924
- Residue swept through: c370924
- This tick: RECONCILE 8913b59..c370924 — one code commit in the window
  (eb9674c, the fixture fold), audited and swept in one pass; both cursors
  advance. The spec cursor is copied forward verbatim: 3c1a58c was fully
  routed last tick and `<spec-delta>` is empty.
  **Audit.** eb9674c verified on disk, not off the log: `write_sibling`
  (`tests/common/mod.rs`:141) is the generalized create-parents-then-write
  primitive, and `write_skill` (150), `write_plugin_json` (157),
  `write_marketplace_json` (164), `write_settings` (170) each compose it —
  the fold shipped as scoped, and TEST-FIXTURE-WRITERS-ONE-HOME is already
  off the queue (96bff16). Its two stated non-folds hold: `coverage_note`'s
  `write_skill` (36) composes `common::write_skill` and only builds a body;
  `requirement_roster::write_clauses` (43) no longer names `Payload` or
  `drift::emit`. **All four gates re-tested and true** — `git log
  c370924..HEAD -- src/ sdk/ tests/ specs/ .github/` is empty, so no gate
  could have moved: `MAX_IMPORT_HOPS = 5` still reads 5 at `src/graph.rs`:65
  under a cite claiming five (IMPORT-HOP-CAP-CITE parked, unruled); `git tag
  -l` carries only the four era tags, crate 0.1.0 vs npm 0.0.7, and
  release.yml:7-9 states the darwin + channel-3 deferral verbatim
  (PACKAGING parked); both pickable entries' cites are unmoved.
  **Sweep.** One find, filed as TEST-MCP-FIXTURE-WRITER-ONE-HOME: the fold
  homed three manifest-locus writers and left `.mcp.json` — the fourth
  shipped manifest locus — with no home and two private copies,
  `mcp_server_kind::write_mcp` (49, body-passthrough, 4 callers) and
  `coverage_note::write_mcp_json` (52, `{}`-hardwired, 3 callers). That is
  `settings.json`'s pair-shape exactly, one locus over, and the
  `{}`-hardwired half is the direct sibling of the `write_settings_json`
  the same pass folded.
  **The CLAUDE.md axis was NOT re-filed.** 8913b59's entry ruled it out by
  name — `memory_gate::write_claude_md` (45, pads to N lines) vs
  `requirement_roster::write_claude_md` (749, verbatim body) are different
  jobs behind a colliding name — and the ruling holds on re-read: they share
  one `fs::write` line, no create-parents, no nesting. A prior tick's
  deliberate exclusion is not re-opened without evidence, and none moved.
- Queue: 5 entries — 3 pickable (LOCK-ROW-PATHS-HARNESS-RELATIVE,
  SDK-BLOCKS-FILE-REFUSAL, TEST-MCP-FIXTURE-WRITER-ONE-HOME), 2 parked on
  human acts. All three pickable are disjoint: `src/drift.rs`+`tests/emit.rs`
  vs `sdk/src/prose.ts`+`sdk/test/refusals.test.ts` vs `tests/common/mod.rs`+
  `tests/{mcp_server_kind,coverage_note,manifest_adapter}.rs`.

Plan continues: no — every input is drained. Inbox empty, no refactor
captures, `<spec-delta>` empty at 3c1a58c, and both reconciliation cursors
now sit at c370924 with no code commit past them. Build takes over on the
three pickable entries.

**Waiting on a ruling:** two forks gate real capability.
`(layer-delivery-format)` holds all four of 0030's derivations — nothing says
what artifact `--layer` names on disk. `(clause-vocabulary-holds)` is unmoved
— four shipped contracts hold decidable, documented rules the algebra cannot
spell, and the corpus sanctions only "undecidable" as a reason a clause is
absent. Nothing is broken by either; what they cost is the capability 0030
ruled important, and a gate whose reach is thinner than `specs/builtins.md`'s
"Strictest documented profile" stance reads.
