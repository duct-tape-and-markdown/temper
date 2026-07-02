# Plan state

- **Phase:** reconcile. HEAD a27b750.
- **Last shipped:** REACHABILITY (build 50c5a00, chore a27b750). Verified on disk:
  `world()` + `reachable` + `dead_activation` live in `src/graph.rs`, library-proven
  in `tests/graph.rs:460` — but `graph::reachable` has **no caller in `main.rs`**;
  the test file itself notes gate-side wiring is next.
- **This tick:** filed **REACHABILITY-WIRE** (parked) for that gate-wiring residual,
  plus the fork `(reachability-gate-mechanism)` naming the undecided seam — how a
  graph-scope reachability fact carries author-declared severity (acyclic is
  always-on, degree is assembly-declared, the spec says "the package's clause
  choice"). Re-verified the five carried entries' anchors on disk (BUILTIN_KINDS
  kind.rs:30 = ["skill","rule"]; Primitive kind.rs:456; Field-get kind.rs:499;
  parse_primitive kind.rs:1048; builtin.rs SKILL/RULE_PACKAGE 32/37) — all accurate,
  no rewrite. Inbox empty.
- **In flight / pickable:** none. All six entries are parked (REACHABILITY-WIRE,
  MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS) or deferred (EXTRACTION-VOCAB-GAPS,
  AGENT-KIND) — every one human-gated: curated `kinds/`+`packages/` data outside
  build's fence, a fence-widen (chain.ts), release creds, or an unresolved fork.
- **Next:** human action unblocks the queue — author the curated activation/package
  data, resolve `(reachability-gate-mechanism)`, widen the fence, or set release
  creds. No autonomous build work remains until then.

Plan continues: no — queue reconciled, inbox empty, no pickable entry; the remaining
work is human-gated, so re-planning would only re-emit the same queue.
