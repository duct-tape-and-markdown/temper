# Plan state

- Spec derived through: f4189c3
- Audited through: 1e5d7ec
- Residue swept through: 859c2e6
- This tick: Ship audit 1e91b44..HEAD — MODULE-RELATIVE-PATHS shipped (78a48db,
  chore 1e5d7ec dropped the entry). Verified on disk: `baseDir` fully retired
  (no ref in sdk/src/tests), emit resolves via `new URL(prose.path,
  prose.moduleUrl)` sdk/src/emit.ts:151 carrying the declaring module's
  import.meta.url, install.rs `relative_to_member_module` (:1128) climbs two
  levels — matches pipeline.md "The SDK": a path reference resolves relative to
  the module that states it, never the workspace. PACKAGING-CHANNELS parked
  reason re-tested, still true: no .github/workflows/release.yml (only
  temper.yml check), root package.json still the private `temper-flume-harness`
  manifest, sdk `@dtmd/temper` 0.0.4. Cursor → 1e5d7ec (HEAD).
- Queue: 1 — PACKAGING-CHANNELS parked (human release creds + engine-binary
  workflow). No pickable entry.

Plan continues: yes — residue sweep trails (859c2e6..HEAD covers the
MODULE-RELATIVE-PATHS code 78a48db; not yet swept against the corpus).
