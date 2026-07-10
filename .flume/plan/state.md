# Plan state

- Spec derived through: a0fccaf
- Audited through: 5717a13
- Residue swept through: 5717a13
- This tick: Inbox — drained the `(manifest-authoring-surface)` resolved note
  (observed a9f7b9e). Deleted the open-questions fork record and derived
  Decision 0021 **phase 1** as an open blockedBy chain, read-side, hook-first:
  MANIFEST-KIND-MODEL (open) → MANIFEST-ADAPTER-READ → HOOK-KIND →
  MCP-SERVER-KIND. Only the head is `open`; the chain is linear so no two
  open entries share a file (the four heavily overlap builtin_kind.rs /
  builtins.ts / the frozen lock — serialized by design). Consequences
  checklist routed in the commit body; bullet 1 (phase 1) → these four
  entries; bullet 2 (phase 2 write side) is NOT yet derivable (its surfaces —
  SDK constructors, the emit projection, bundle.rs conversion, guard coverage
  — don't exist until phase 1 lands), so it holds; bullets 3 (coverage
  retirement) and 4 (builtins.md list) routed as absorbed/session — see body.
- Spec cursor HELD at a0fccaf: a9f7b9e is only *partially* routed (phase 2
  underived), so it re-surfaces via the delta each tick as the safety net —
  phase 2 derives once phase 1 ships. The delta, not the drained inbox, keeps
  0021 alive.
- Queue: MANIFEST-KIND-MODEL (open, next) → 3-deep phase-1 chain (blockedBy) →
  PACKAGING-CHANNELS (parked). Disjoint: the phase-1 chain (src/sdk) vs
  PACKAGING (package.json, release.yml) share no path.

Plan continues: yes — post-ship reconciliation (window 5717a13..HEAD carries
55d7ee4, the sdk 0.1.0→0.0.6 re-cut; both cursors trail it). Spec-delta
phase 2 is present-but-not-derivable (holds behind phase 1), so the next
*actionable* input is that reconciliation.
