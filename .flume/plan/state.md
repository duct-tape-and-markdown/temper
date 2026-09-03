# Plan state

- Spec derived through: 13455d4a — unchanged, copied forward (not this tick's job).
- Audited through: 8feb4617 — advanced from 09a79fef; window (4d7e8127, 5953c754) touched src/graph.rs, sdk/src/emit.ts, sdk/src/kind.ts.
- Residue swept through: 8feb4617 — advanced, same window: a clean function replacement (graph.rs's `uncontained`) and one additive SDK fact, no dangling symbols or duplicate surface left behind.
- Posture swept through: 22f8064c — unchanged, mid-rotation continues (forward window non-empty via this tick's src/sdk touches, but the audit was the live job this tick).
- This tick: post-ship reconciliation 09a79fef..8feb4617. Verified on disk (full `cargo test` + `pnpm -C sdk test`, all green) that MENTION-REACHABLE-SUBSET-CONTAINMENT and EDGE-TARGET-FACTS-REPO-ROOTED-PATH shipped to acceptance; both already dropped from pending by the ship commit, nothing further to do. Reconciled two recorded merge-failure footprints (`observedFiles`, pending-entry rule): SKILL-CONTRACT-MENTION-REACHABLE-PATHS's under-stated the declared ripple — folded src/builtin_lock.toml and tests/contract_template.rs into files.edit, full rewrite, and confirmed its prose precondition (#33's fix) shipped at 4d7e8127; HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL's footprint matched its declared files exactly — full rewrite clearing the marker, no content change. Both re-stamped `scoped at 8feb4617`.
- Queue: 10 pending — 2 open, 5 blockedBy, 1 parked, 2 deferred (unchanged count-shape aside from the two ships already reflected here). Open forks: 4. Friction: 0. Amendments: 0. Inbox: 0. Refactor: 0.

Plan continues: after-build — SKILL-CONTRACT-MENTION-REACHABLE-PATHS and HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL are pickable now; the posture sweep's open rotation resumes once the wave hands back.
