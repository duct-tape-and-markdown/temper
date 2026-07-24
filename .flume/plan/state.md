# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: a4ed6d4 — unchanged this tick.
- Residue swept through: a4ed6d4 — unchanged this tick.
- Posture swept through: mid-rotation, at src/install.rs — one entry filed;
  src/json_manifest.rs next in the c9d11d5 rotation's frontier, untouched
  this tick.
- This tick: POSTURE SWEEP of src/install.rs (c9d11d5 rotation), read whole
  (1901 lines, no internal-crate neighbor small enough to fold in — imports
  span builtin_kind/check/compose/contract/drift/engine/frontmatter/import/
  json_manifest/json_splice/kind/placement). Embedded-provider-knowledge
  finding: `settings_path` (593) and `is_claude_path` (778) each hardcode
  the `.claude` locus as a literal instead of importing
  `builtin_kind::CLAUDE_ROOT`, the existing provider-face constant
  `coverage_note.rs` already reuses — filed
  INSTALL-CLAUDE-ROOT-PROVIDER-FACE-REUSE (open, mechanical). The tracked
  `placement_lines` orphan (1696-1702) re-confirmed unshifted and still
  unreached by any open or shipped entry; stamp advanced.
- Queue: 3 pending — 1 open, 1 parked, 1 deferred. Open forks: 2, unchanged.
  Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: after-build — INSTALL-CLAUDE-ROOT-PROVIDER-FACE-REUSE is
open and pickable; the posture sweep resumes at src/json_manifest.rs once
the wave hands back.
