# Plan state

- Spec derived through: a9f7b9e
- Audited through: c5df845
- Residue swept through: c5df845
- This tick: Post-ship reconciliation, window 77d590e..HEAD. Its one
  src/tests/sdk commit is 76aaa83 — HOOK-KIND: the hook built-in kind
  (fields-only, embedded, collection address `hooks.<Event>`) plus the
  wired manifest read path — verified on disk (builtin_kind.rs:200
  `claude_code_hook`, builtins.ts `hook`/`hookDefaultContract`, tests/
  hook_kind.rs, and the main.rs `Fields`+`collection_address` read
  dispatch). Audit: the entry shipped and was already dropped from pending
  by its chore(flume) c5df845; its downstream blocker cleared, so
  MCP-SERVER-KIND flips `blockedBy HOOK-KIND` → `open` (the second manifest
  kind inherits the main.rs dispatch + src/kind.rs helpers HOOK-KIND wired
  — no main.rs edit needed; notes refreshed). MANIFEST-WRITE-SIDE stays
  parked — still a placeholder needing re-scope, phase 1's remaining kind
  (MCP-SERVER-KIND) unshipped; reason refreshed. PACKAGING-CHANNELS parked,
  condition re-verified true (only temper.yml, no release.yml; root
  package.json still the private flume manifest) — untouched.
  Sweep: 76aaa83 is corpus-sanctioned (0021; builtins.md "The coverage
  bar": hooks are registration members) and subtractive over settings.json's
  coverage (retires the `hooks.<Event>` segment, MANIFEST-WRITE-SIDE
  completes it) — no new residue. It opened `sdk/src/builtins.ts` (+103
  lines) and `src/kind.rs` but reconciled neither rider: the three deleted-
  PACKAGE.md cites shifted 308/348/385→344/384/421, left as unchanged context
  → undischarged, carried (MCP-SERVER-KIND next opens builtins.ts). All other
  riders re-verified on their untouched files, stamps to c5df845. Both
  cursors advance.
- Queue: MCP-SERVER-KIND (open, next) → MANIFEST-WRITE-SIDE (parked, phase 2)
  → PACKAGING-CHANNELS (parked). Disjoint: only the head is `open`.

Plan continues: no — inbox empty, no specs delta past a9f7b9e,
reconciliation done and both audit/residue cursors at HEAD. MCP-SERVER-KIND
is `open` for build to pick up.
