# Plan state

- Spec derived through: 862f0b60 — copied forward: no `specs/` commit past it.
- Audited through: e253fd44 — copied forward: no `src/`/`tests/`/`sdk/` commit past it.
- Residue swept through: e253fd44 — copied forward, same window.
- Posture swept through: 7d695577 — the rotation armed at that sha closed this
  tick; its last two frontier modules (src/drift.rs, src/glob.rs) are covered.
- This tick: posture sweep closed at src/drift.rs + src/glob.rs — two findings filed,
  the lock read twice per gate run and a second EOL normalizer.
- Queue: 10 pending — 2 open, 1 blockedBy, 3 deferred, 4 parked. Pickable: 1 —
  GATE-READS-THE-LOCK-ONCE-PER-RUN; INSTALL-LIFTS stays fork-held on
  `(hook-member-identity)`. Open forks: 19. Friction: 2 (human channel).
  Amendments: 0. Refactor: 0. Inbox: 0.

Plan continues: after-build — the sweep re-arms over 7d695577..HEAD; the pickable entry ships first
