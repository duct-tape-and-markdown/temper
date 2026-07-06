# Plan state

- **Phase:** residue sweep — **nothing shipped since the last plan tick**
  (544b041). The only commit since is one flume-prompt chore (61d56a2). Spec-delta
  empty (no `specs/` commit since 544b041); inbox empty. Intent unmoved.
- **Last shipped:** TEMPER-TOML-ZERO (build 4d6e813 / chore ed95bcc) — terminal
  of the S1→S7 `(inplace-lock-producer)` demolition chain. Re-verified on disk this
  tick: `rg temper.toml src/` = 0; `import::run` copy-tree gone (only `write_rollup`,
  the emit lock-writer, survives); the `[[member]]` codec gone from compose.rs;
  `compose::effective` composes each kind's contract from the lock's `ClauseRow`.
- **This tick:** ran the residue sweep (not "delta empty → idle"). Found one named
  residue with its gate re-tested false: bug 2's stale `kind::BUILTIN_KINDS =
  ["skill","rule"]` const (kind.rs:27). Its "retires with the entangled kind.rs
  demolition" routing is stale — the entanglement was one compose.rs codec test,
  deleted by CODEC-RETIRE, so the const now has **zero** usages across src/ and
  tests/. **Filed KIND-BUILTIN-CONST-RETIRE** (`open`, this tick's one pickable
  entry). PACKAGING-CHANNELS unchanged (still parked).
- **In flight:** two entries — **KIND-BUILTIN-CONST-RETIRE** (`open`, pickable:
  delete the dead const, cite 15-kinds "identity is an import") and
  **PACKAGING-CHANNELS** (parked on human release creds + the per-platform
  engine-binary workflow + John's decide-at-release calls; cite 50-distribution
  "Three channels"). The two are disjoint (src/kind.rs vs .github/ + package.json).
- **What's next (all human-gated after this cut):** PACKAGING-CHANNELS release
  setup + USPTO name screen; the genre-fence-format workshop (cascade is the
  pilot); the OPEN forks that need John before they yield pickable work —
  `(default-assembly-as-data)`, `(edge-representation-unify)` join→graph,
  `(json-projection-format)`/`(hook-kind-locus)`/`(builtin-workspace-qualified-key)`
  (SDK-primary foundation); and the `custom_kinds`-empty foundation gate
  (main.rs:432,661), which stays until the SDK-primary front door delivers custom
  kinds.

Plan continues: no — the residue sweep produced one pickable `open` entry
(KIND-BUILTIN-CONST-RETIRE); spec-delta and inbox are empty and the rest of the
queue is human-gated. Hand to build; the queue drains by building, not re-planning.
