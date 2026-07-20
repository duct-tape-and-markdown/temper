# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: a54f3c3 — unchanged; window a54f3c3..HEAD (749f06f touching sdk/src/declarations.ts) unreconciled, not this tick's job.
- Residue swept through: a54f3c3 — unchanged, same unreconciled window.
- Posture swept through: full src/ list covered; sdk/src/assembly.ts, sdk/src/builtins.ts, and sdk/src/claude-code.ts covered too — mid-rotation, unchanged; sdk/src/contract.ts is the tree-order candidate next.
- This tick: INBOX. Both notes verified on disk (matched the report exactly): `src/compose.rs:896` reads the dial via raw `builtin_kind::definition(dial::KIND)` instead of `overlaid_builtin_kinds` (every sibling `kind_features` caller migrated at d108fbf, this one missed); `src/engine.rs:883`'s `guard_membership_fails` `_` arm returns `false` where the pre-extraction inline `When` arm (d865e30) was `_ => continue`, inverting the fallthrough at its sole call site (1237). Filed READ-DIAL-OVERLAY-MIGRATE and GUARD-MEMBERSHIP-FALLTHROUGH-RESTORE, both open/pickable. Inbox drained to empty.
- Queue: 4 pending, 2 open (READ-DIAL-OVERLAY-MIGRATE, GUARD-MEMBERSHIP-FALLTHROUGH-RESTORE), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unaffected, unchecked this tick). BUILTINS-SETTINGS-MANIFEST-EXPORT-PRUNE already shipped (c4e0998), no longer in the queue. Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — post-ship reconciliation, window a54f3c3..HEAD (749f06f touching sdk/src/declarations.ts, plus this tick's own commit landing on top) is unreconciled and sits below the inbox job this tick serviced; the posture rotation and the two open pending entries wait behind it.
