# Plan state

- Spec derived through: 5fd21e72 — unchanged, copied forward (no `specs/` commits past cursor).
- Audited through: a67ac627 — advanced; window 17ca0bf2..a67ac627 reconciled this tick.
- Residue swept through: a67ac627 — same window, no residue found.
- Posture swept through: tests/tap.rs next — unchanged, copied forward (posture job not taken this tick).
- This tick: post-ship reconciliation, window 17ca0bf2..a67ac627. Five commits touch `src/`,`tests/`,`sdk/`: three are pure release/lock-sync (01485f10, 4997f221, ad10e8cb — no code), one is a version bump (32c37ea1), one is a real fix (c054dbce, `sdk/src/emit.ts` memberTable — restores last-writer-wins for registration-address duplicates, refusal kept for projected members). Verified on disk the fix completes no pending entry and invalidates none: HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL's parked reason and `(hook-member-identity)` dependsOnForks still hold true post-fix (the commit's own body defers the discriminator to that fork); SDK-MEMBER-TABLE-NESTED-EDGE-TARGET's line cites (255/307/700) still accurate, untouched by this diff (different mechanism — embedded-value indexing, not registration-address collision). Sweep: commit body names no retirement/demolition; no unsanctioned residue in the window. Clean, no gap.
- Queue: 28 pending — 18 open, 4 parked, 3 deferred, 3 blockedBy. Open forks: 8. Friction: 0. Amendments: 0. Inbox: 0. Refactor: 0.

Plan continues: after-build — inbox, spec-delta, and reconciliation are all clear; the only remaining live input is the open posture-sweep rotation (tests/tap.rs next), and 18 open entries are pickable, so ready work ships first and the sweep resumes when the wave hands back.
