# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: f130ebc — window befa77f..f130ebc (55b8539, 9bf9ebb) audited: both build tags (REGISTRATION-ENABLEMENT-LABEL-FIELD-CARRYING, INSTALL-PACKAGE-JSON-ANCESTOR-SHORT-CIRCUIT) shipped and drained from pending.json already; verified live on disk (field-carrying `enablement(enabled)` wire label, `ensure_package_json`/`spawn_npm_install` split), not just via log. The prior tick's cursor claim ("befa77f..HEAD touches only version bumps") was stale — 55b8539/9bf9ebb both carry real src/sdk-src/test changes; corrected here.
- Residue swept through: f130ebc — same window swept: no dangling `ensure_dependency` references (rg clean across src/tests/sdk/specs), `cargo test` green. Both parked entries' blockers untouched (graph.rs, release.yml darwin/channel-3 deferral unchanged on disk). Updated the ride-only orphan record in open-questions.md (install.rs `placement_lines` doc comment, now 1690-1696) to carry INSTALL-PACKAGE-JSON-ANCESTOR-SHORT-CIRCUIT as another entry that touched install.rs without reaching it.
- Posture swept through: 00b880d — unchanged; re-arms next tick (this window's 55b8539/9bf9ebb touch sdk/src/declarations.ts and src/install.rs, both posture-eligible), but posture sweep is job 4 and wasn't this tick's job.
- This tick: POST-SHIP RECONCILIATION, window befa77f..f130ebc — audit clean (both entries shipped, tested, dropped), sweep clean (no residue), one ride-only record refreshed; see commit body for the stale-cursor correction.
- Queue: 2 pending, 0 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — posture sweep re-arms next tick (window now touches sdk/src/declarations.ts and src/install.rs), and no pickable entries exist to hand build meanwhile (both queue entries are parked), so the next tick runs the sweep directly rather than idling.
