# Plan state

- Spec derived through: f4189c3
- Audited through: 1e91b44
- Residue swept through: ce6c183
- This tick: Ship audit 44b4f27..HEAD. Two entries confirmed shipped on disk:
  EMIT-LF-NORMALIZE (76535bd — `normalize_lf` src/drift.rs:647, applied over
  both emit passes :709/:719; the "six families" comment rider landed :289/:296/
  :1424, no stale "four") and SETTINGS-FORMAT-PRESERVING (e617b4e — new
  `json_splice` module wired via lib.rs:37, install.rs splices the hooks merge
  :769-803 instead of re-serializing). Stale gate re-tested: MODULE-RELATIVE-
  PATHS was blockedBy SETTINGS-FORMAT-PRESERVING → blocker shipped → gate flipped
  to `open`; install.rs line-cites (warned to drift on that ship, +331/-96)
  re-verified at HEAD and rewritten (957→1128, 980→1151, 991→1162, doc
  950-953→1121-1126, tests 1349/1367→1494/1506/1525). emit.ts/prose.ts cites
  unmoved, re-confirmed. Cursor → 1e91b44.
- Queue: 2 — MODULE-RELATIVE-PATHS now `open` (sdk emit/prose + install.rs +
  tests; sole pickable, disjoint from PACKAGING). PACKAGING-CHANNELS parked
  (human release creds + engine-binary workflow).

Plan continues: yes — residue sweep trails HEAD (Residue swept through ce6c183;
76535bd/e617b4e landed past it). Next tick sweeps ce6c183..HEAD.
