# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: 103f471 — unchanged this tick (no commits past it
  touched src/, tests/, or sdk/).
- Residue swept through: 103f471 — unchanged this tick, same reason.
- Posture swept through: mid-rotation, at src/roster.rs (neighborhood:
  its crate::compose/crate::engine/crate::extract/crate::tap imports
  read for context, no violation there) — 421cf79 rotation frontier now
  src/schema.rs onward.
- This tick: POSTURE SWEEP src/roster.rs — filed
  ROSTER-READ-OPT-IN-JOIN-DEDUP: the opt-in `satisfies` join
  (`Features::satisfies` naming a requirement) is reimplemented three
  times — roster::is_satisfier (roster.rs:26) and, independently,
  read::count_satisfiers (read.rs:1197) and read::satisfiers_of
  (read.rs:1478) — the latter two's own doc comments already name it
  "the same opt-in join" as roster's without consolidating onto one
  predicate ("One job, one home", engineering.md). coverage.rs's
  unfilled-check builds a different aggregate and is not part of this
  finding.
- Queue: 3 pending — 1 parked, 1 deferred, 1 open. Open forks: 2,
  unchanged. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — the only remaining live job is the posture
sweep and a pickable entry now exists (ROSTER-READ-OPT-IN-JOIN-DEDUP);
build ships it, the sweep resumes at src/schema.rs onward when the wave
hands back.
