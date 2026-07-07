# Plan state

- Spec derived through: f4189c3
- Audited through: 1e91b44
- Residue swept through: 859c2e6
- This tick: Residue sweep ce6c183..HEAD — clean, nothing fileable. The two
  code commits ship the corpus faithfully: EMIT-LF-NORMALIZE (76535bd)
  `normalize_lf` src/drift.rs:647 over both emit passes :709/:719 matches
  pipeline.md "projections written LF uniformly"; SETTINGS-FORMAT-PRESERVING
  (e617b4e) the `json_splice` merge install.rs:769-803 matches install
  "format-preserving". The two surviving `to_string_pretty` are legitimate
  (install.rs:722 fresh-doc path per its own comment; :1309 package.json,
  out of scope). drift.rs "six families" comment rider already landed with
  76535bd. Cursor → 859c2e6 (HEAD).
- Queue: 2 — MODULE-RELATIVE-PATHS `open` (sdk emit/prose + install.rs + tests;
  sole pickable, cites reconciled at 859c2e6). PACKAGING-CHANNELS parked (human
  release creds + engine-binary workflow). Disjoint — no shared file.

Plan continues: no — every input current (inbox empty, no spec delta, code
audited through 1e91b44, residue swept to HEAD); MODULE-RELATIVE-PATHS is
pickable, build takes over.
