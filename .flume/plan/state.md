# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: d2ed4e8 — advanced from a4ed6d4.
- Residue swept through: d2ed4e8 — advanced from a4ed6d4.
- Posture swept through: mid-rotation, at src/install.rs — unchanged this
  tick; src/json_manifest.rs next in the c9d11d5 rotation's frontier.
- This tick: POST-SHIP RECONCILIATION, a4ed6d4..d2ed4e8. One src-touching
  commit in window: d760857 (build: import builtin_kind::CLAUDE_ROOT in
  install.rs), shipped as INSTALL-CLAUDE-ROOT-PROVIDER-FACE-REUSE
  (d2ed4e8). Audit: read the diff on disk — `settings_path` (593) now
  joins `builtin_kind::CLAUDE_ROOT` and `is_claude_path` (778) formats it
  with a slash, exactly the entry's scope, no behavioral change, entry
  already correctly absent from pending.json. Sweep: `rg` for remaining
  `.claude` literals in src/ turned up only doc comments, module headers,
  and test fixtures (a separate, excluded class per this file's fixture
  note) — no further production-code duplication of the locus literal.
  Re-tested PACKAGING-CHANNELS-REMAINDER's parked condition: release.yml
  still states the darwin/channel-3 deferral verbatim at lines 7-9, park
  holds. Confirmed on disk the shipped fix's line-for-line diff (no net
  line change) leaves install.rs's tracked `placement_lines` orphan
  (1696-1702) unshifted; open-questions.md's install.rs orphan paragraph
  updated to fold the ship in.
- Queue: 2 pending — 0 open, 1 parked, 1 deferred. Open forks: 2,
  unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — no pickable entries exist to hand build, so the
posture sweep's open rotation drives itself; next tick resumes at
src/json_manifest.rs.
