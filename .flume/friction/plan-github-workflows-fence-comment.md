## Symptom
`.flume/chain.ts`'s `BUILD_WRITABLE_PATHS` lists `.github/**` under a bare
"// CI" comment with no carve-out. Nothing on disk states that
`.github/workflows/release.yml` is conventionally human-only —
`release.md`'s own frontmatter scopes to that exact path and every commit
touching it is `chore(release):`/`fix(release):`, never `build:` — while
`.github/workflows/temper.yml` IS build-fence-eligible and precedented
(6df1b76, a `build:` commit that rewrote it). The human chore commit that
filed inbox item 11 (CI Node 20 deprecation, afc495e) read the bare
`.github/**` glob and concluded the opposite for both files: "outside
build's writablePaths."

## Cost this tick
One inbox item's stated disposition was factually wrong and had to be
independently re-verified (chain.ts read + `git log` precedent on both
workflow files) before routing. No revert, no build tick lost this time —
but the next note citing ".github/workflows is outside the fence" repeats
the same mistake until the comment says otherwise.

## Suggested fix
Split the `.github/**` line's comment in chain.ts: name
`.github/workflows/release.yml`'s human-only convention explicitly
(mirroring `release.md`'s own path scope) so the glob's positive
fence-membership doesn't read as a blanket "CI is build's" minus the one
carved-out file.
