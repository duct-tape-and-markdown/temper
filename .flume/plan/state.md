# Plan state

- Spec derived through: cd7135b
- Audited through: 99337b8
- Residue swept through: 77b2eb9
- This tick: Residue sweep (job 4), continued. Inbox/spec-delta/ship-audit
  stayed quiet (no src/tests/sdk commits past 99337b8 — only prior plan
  commits touch the tree). Swept vocabulary: decision 0001 retired
  `floor`→`default contract` and `blast radius`→`impact`; a separate,
  later-settled term is `enforcement mode` (representation.md/distribution.md),
  displacing an older `posture` naming. Verified against current corpus body
  (builtins.md never says `floor`; contract.md's `impact` never says `blast
  radius`; representation.md/distribution.md say `enforcement mode`, never
  `posture`). `floor` is live in real identifiers/user-facing text/test names
  (main.rs, compose.rs, read.rs's pinned `why` narration, 6 SDK-exported
  clause arrays, their sdk test, tests/read_verbs.rs, tests/builtin_lock_frozen.rs's
  embedded fixture, builtin_lock.toml's header) — filed
  RETIRE-FLOOR-VOCABULARY-FOR-DEFAULT-CONTRACT. `posture` (enforcement-mode
  sense) is live in one test fn name plus doc comments in install.rs/drift.rs —
  filed RETIRE-POSTURE-VOCABULARY-FOR-ENFORCEMENT-MODE, blockedBy
  INSTALL-WHOLE-CONVERSION (both touch install.rs + tests/install.rs).
  `blast radius` stayed unfiled — it's read.rs's own internal description of
  what `impact` computes (depth rule: exact narration wording is code's HOW,
  not a modeled-noun collision like `floor`/`posture` are). Comment-only stray
  mentions of `floor`/`posture` (builtin.rs, kind.rs, roster.rs, extract.rs,
  contract.rs, schema.rs, engine.rs, bundle.rs, sdk/kind.ts's nesting-depth
  "posture 3", sdk/contract.ts's severity-flavored "posture") ride whichever
  entry next opens those files for a real reason, per the comment-staleness
  exception.
- Queue: 6 — INSTALL-WHOLE-CONVERSION, ACCEPTANCE-CUSTOM-KIND-VIA-LOCK,
  RETIRE-FLOOR-VOCABULARY-FOR-DEFAULT-CONTRACT are three pickable,
  file-disjoint open entries; RETIRE-POSTURE-VOCABULARY-FOR-ENFORCEMENT-MODE
  and RETIRE-OWN-PATH-MACHINERY are both blockedBy INSTALL-WHOLE-CONVERSION;
  PACKAGING-CHANNELS is parked.

Plan continues: yes — quiet closing pass (job 5) is next; inbox, spec delta,
ship audit, and residue sweep are all current as of this tick.
