# Plan state

- Spec derived through: d5d1b37
- Audited through: c685a93
- Residue swept through: c349cb2
- This tick: ship audit of the MODE-ROOT-MEMBER-FIELD ship (cb0814f build +
  c685a93 chore) past the audit cursor. Verified on disk: compose::Authority
  renamed to EnforcementMode (two values {Shared, Surface} kept, compose.rs:23-31),
  the root member's `mode` field carries enforcement mode, the fabricated
  `authority` emit fact is gone — decision 0005's type-rename slice. MODE already
  dropped from pending by its chore. Re-tested the stale gates: GENRE-FOLD's
  blockers MODE + SATISFIER both shipped and its cites survived MODE's main.rs
  churn (main.rs:198/479 intact, extract.rs symbols at cited lines) → unblocked
  to `open`, stale "serialized" parentheticals cleaned. PACKAGING-CHANNELS park
  holds (install.rs still pins ^0.0.2, root package.json still the private flume
  manifest, no release.yml). enforcement-mode-vocabulary fork: MODE's type rename
  is now on disk — updated the record's stale `Authority`→`EnforcementMode`
  reference, fork stays OPEN (values recut needs John). Audit cursor → HEAD.
- Queue: 2 entries — GENRE-FOLD (open, pickable, sole open, disjoint from
  parked), PACKAGING-CHANNELS (parked on release creds).

Plan continues: yes — residue sweep c349cb2→HEAD (the MODE ship, cb0814f) is
code past the residue cursor and not yet swept against the corpus; that is the
next live input below the audit job.
