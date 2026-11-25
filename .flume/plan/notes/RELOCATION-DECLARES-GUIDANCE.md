# RELOCATION-DECLARES-GUIDANCE

Shipped. `KindRelocation` gains `guidance`, `fieldGuidance` (keyed by the
widened surface `T`, so `KindRelocation<T extends object>` now) and `cite`;
any of guidance/fieldGuidance counts as a diverging face, so
`relocate(skill, { guidance })` alone is a valid relocation.

Shape chosen: `relocate` *replaces* the base's counsel rather than layering a
second provenance fact. The relocated facts drop the built-in's
guidance/fieldGuidance/cite and carry only the delta's, so `guidanceRules`
lost its `relocates` skip and just projects whatever counsel the facts hold
(shipped values are still skipped by identity). `relocationInvariants` now
erases the three counsel fields so relocation-vs-base still resolves.

Side effect worth knowing: a relocated kind's kind-fact row `guidance`/`cite`
columns now carry the relocation's counsel (absent when it declares none),
no longer the built-in's. The engine's `overlay_builtin_kind` clones the
compiled-in built-in and never reads those columns, so `explain`/`schema`
still serve the built-in's own counsel for a relocated kind - the relocation's
counsel reaches the agent only through the rule. If explain should show the
relocation's counsel, the overlay must read the row's guidance: Rust work,
not taken here. No dogfood (`.temper/`) relocation exists, so no lock moves.
