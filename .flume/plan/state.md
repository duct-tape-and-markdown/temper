# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: a4ed6d4 — advanced from 4d9be4e.
- Residue swept through: a4ed6d4 — advanced from 4d9be4e.
- Posture swept through: mid-rotation, at src/graph.rs — src/hash.rs next
  in the c9d11d5 rotation's frontier, untouched this tick.
- This tick: POST-SHIP RECONCILIATION over 4d9be4e..a4ed6d4. Of the four
  commits, only bf4b5cd and 8272023 touch src/tests/sdk (both sdk/
  version-pin files only; afc495e/a4ed6d4 are .flume/-only). Audit: on
  disk, sdk/package.json + package-lock.json both read 0.0.12, v0.0.12
  tagged — neither pending entry references release version bumps, so
  nothing to drop. Re-tested PACKAGING-CHANNELS-REMAINDER's park
  condition: release.yml lines 7-9 still state the darwin +
  channel-3 deferral verbatim, unmoved by this window. Sweep: no new
  symbols/vocabulary entered src/tests/sdk (pure version-field edits) —
  nothing to file. Both cursors reconciled clean, advanced to HEAD.
- Queue: 2 pending — 0 open, 1 parked, 1 deferred. Open forks: 2, unchanged.
  Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — posture sweep at src/hash.rs (c9d11d5 rotation
frontier) is next tick's job; queue holds no pickable (open-gate) entries.
