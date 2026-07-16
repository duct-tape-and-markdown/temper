# Plan state

- Spec derived through: 82c816e
- Audited through: a2f4a1c
- Residue swept through: a2f4a1c
- This tick: DERIVE the spec delta — Decision 0025 (82c816e) routed whole,
  four entries filed as one serialized chain (they share `sdk/src/kind.ts`,
  so only the head is `open`). Every Consequences bullet is checked by name
  in the commit body. The four code routes the record names:
  COMPOSED-BODY-ADMISSION (retire declaration-side `withinHosts` —
  `sdk/src/kind.ts:56`; admission moves corpus-side over the host kind,
  which is what lets a built-in's composed body admit consumer types at
  all), SKILL-NESTED-REFERENCE-DOCS (the file-child nesting template —
  `KindFacts` carries no template fact and `src/kind.rs:565`'s `Template`
  is embedded-only, both narrower than representation.md; `skill` then
  declares its bundled reference documents), EMBEDDED-FORMAT-TARGET-FACTS
  (`ResolvedEmbeddedMemberValue`, `sdk/src/kind.ts:296`, carries no target
  facts — a render hook cannot place what it cannot see), and
  FORMAT-OMITS-EDGE-CLAUSE (the predicate is absent from the closed enum,
  `src/contract.rs:81`; 0025 is the decision that ratifies adding it).
  **One bullet is a fork, not an entry:** `(guidance-climb)` — the ratified
  third delivery layer has no corpus home, and both candidate homes refuse
  it (the plugin skill "never teaches taste"; a default contract carries
  opinion only as a clause's guidance, and the climb is not a check). Its
  own Consequences line sequences it after the machinery, so it blocks
  nothing. Moot, verified: the corpus edits landed in 82c816e itself; the
  mention respell retires nothing (no "four loci" narration anywhere in
  `src/`/`sdk/`); `docs/proposals/` is human territory. Fork board:
  `(posture-recursion)` DELETED — resolved by 0025 and now routed, so the
  lifecycle rule evicts it; the skill-package asymmetry shrank to the
  choice that still stands (its factory datum is SKILL-NESTED-REFERENCE-DOCS
  now, not a pending demand signal). Five residue riders' "no queued entry
  opens that file" routing went stale the moment these entries landed —
  re-pointed: `sdk/src/kind.ts`'s two riders and `emit.test.ts:907` ride
  COMPOSED-BODY-ADMISSION, builtins.ts's two surviving PACKAGE.md cites ride
  SKILL-NESTED-REFERENCE-DOCS, `contract.rs:475` rides
  FORMAT-OMITS-EDGE-CLAUSE.
- Queue: 1 pickable (COMPOSED-BODY-ADMISSION); SKILL-NESTED-REFERENCE-DOCS →
  EMBEDDED-FORMAT-TARGET-FACTS → FORMAT-OMITS-EDGE-CLAUSE serialize behind
  it; PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag) — re-tested on disk at 1baeedd: no version tag, crate 0.1.0
  vs npm 0.0.7, release.yml:7 still defers darwin. Gate still true.

Plan continues: no — the delta is fully routed (82c816e was the only commit
past the cursor), and the input below it is dry: both audit cursors sit at
a2f4a1c, still the window's last src-touching commit — nothing has touched
`src/`/`tests/`/`sdk/` since (5ea787e/21a8a3c/1ded3a8/bf98217/1baeedd touched
`.flume/`, `docs/`, `.claude/`, and `specs/` only). The inbox is empty.
**Build takes over: COMPOSED-BODY-ADMISSION is pickable.**
