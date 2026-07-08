# Plan state

- Spec derived through: bdc938d
- Audited through: 1e5d7ec
- Residue swept through: 886dc64
- This tick: Residue sweep (job 4), 31cd09a..HEAD. Corpus moved (0012/0014
  shipped, 0013 fork-routed) while code held still. Three residue classes: genre
  noun → fully filed (EMBEDDED-FENCE-MEMBER-NOUN covers every authored genre
  file + retires genres.ts); Format label → fork-routed by
  `(format-template-spelling)`; registration scalar → filed
  (REGISTRATION-CHANNELS) but one file orphaned — `sdk/src/declarations.ts`
  `registrationLabel` (the SDK emit-side lock-column producer, coupled to
  `registration.via`) was absent from its fence. Added it; serialized under the
  shared-declarations.ts blockedBy chain. Cursor 31cd09a → 886dc64 (HEAD).
- Queue: 5 — EMBEDDED-FENCE-MEMBER-NOUN open/pickable; REGISTRATION-CHANNELS→
  COMMAND-KIND→AGENT-KIND serialized behind it; PACKAGING-CHANNELS parked. One
  open entry, fence disjoint.

Plan continues: no — inbox empty, spec cursor covers all specs commits (none
past bdc938d), no code shipped since the audit cursor, residue swept to HEAD.
Build takes over: EMBEDDED-FENCE-MEMBER-NOUN is pickable.
