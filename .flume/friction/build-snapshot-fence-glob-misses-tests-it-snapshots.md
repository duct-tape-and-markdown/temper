## Symptom

`.flume/chain.ts`'s `BUILD_SURFACE_PATHS` carries `tests/snapshots/**` to make an
insta re-bless always writable by a scoped build tick — its own comment says so
("Six fence misses on 2026-09-06 were a `.snap` alone"). The glob matches nothing
in this repo: every snapshot lives at `tests/it/snapshots/*.snap`, and the fence
matcher compiles `tests/snapshots/**` to `^tests/snapshots/.*$`
(`@dtmd/flume/dist/src/paths.js`, `globToRegex`), so `tests/it/snapshots/…` fails
it. The guard the comment describes has never fired for this repo's layout.

Any entry whose change moves a rendered assertion hits this. This tick's
`EMPTY-CONTRACT-IS-A-DECLARED-CONTRACT` gained one `contract` assembly row in the
gauntlet fixture's emitted lock — a one-row re-bless, outside the fence.

## Cost this tick

One of six out-of-fence paths that made the tick file a re-scope capture instead
of a commit (`.flume/refactor/build-empty-contract-fence-short-six-paths.md`).
On its own it would have reverted an otherwise-green commit — five gates passed.
~10 min to read the flume fence matcher and confirm the glob is dead rather than
loosely matched.

## Suggested fix

One-line glob widening in `.flume/chain.ts`'s `BUILD_SURFACE_PATHS`:
`"tests/snapshots/**"` → `"tests/**/snapshots/**"`, which covers
`tests/it/snapshots/` and any future test binary's own directory without opening
`tests/**`. Human territory (`chore(flume):`) — the chain's fence config is
outside the amendments channel's in-scope targets (0044).
