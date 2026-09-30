---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: ["Cargo.toml","sdk/package.json","sdk/package-lock.json",".github/workflows/release.yml","CHANGELOG.md","docs/MIGRATING-*.md"]
---
# Release — cutting a temper version

A release is interactive, never a build tick. The versioning policy is
`specs/distribution.md`, "Versioning"; this rule is the recipe. The tag is
the trigger and the published pair is the gate, not the local tree.

## The cut, in order

1. **Mine the draft.** `pnpm changelog` prints an `[Unreleased]` draft from
   the `build:` commits since the last recorded version, breaks first. It is
   raw material, never pasted as is.
2. **Curate.** Fold it into `CHANGELOG.md` as `## [X.Y.Z] — <date>` in the
   `public-prose` register: `### Upgrading` and `### Breaking` lead, then
   Added / Changed / Fixed. Drop internal refactors and test work. A break
   the draft missed (a body with no `BREAKING:` line) is still a break —
   read the bodies of anything touching the CLI, the SDK surface, the lock
   or addresses.
3. **Migration note.** If the entry has `### Breaking`, write
   `docs/MIGRATING-X.Y.Z.md`: each break before and after, opening with a
   link to the note before it.
4. **Bump the version in every home together** — a partial bump fails the
   gate:
   - `Cargo.toml`, the version `temper --version` reports;
   - `sdk/package.json`, its own `version` **and** the
     `optionalDependencies` engine pins (`@dtmd/temper-<platform>`).
5. **Re-emit every committed lock.** Each lock records the engine version
   that wrote it, so the bump moves them: `cargo run -- emit` for this
   repo's `.temper/`, and the shipped example with the recipe its
   byte-compare test prints on failure. CI's `emit --frozen` plus
   `git diff --exit-code` fails any lock left behind.
6. **Gates green**: `cargo test`, `cargo clippy --all-targets -- -D
   warnings`, `cargo fmt --all --check`, `pnpm --dir sdk test`,
   `cargo run -- check`.
7. **Commit `chore(release): cut X.Y.Z`**, tag `vX.Y.Z`, push `main` and the
   tag. The tag push publishes (`release.yml`).
8. **Watch the release run.** The cut is shipped only when its smoke job —
   installing the published pair from the registry and round-tripping
   `install` → `emit` → `check` — is green. Green build jobs and provenance
   do not prove the pair works together.
9. **Sync the SDK lock** once the engines are live: `npm --prefix sdk
   install`, commit `chore(release): sync the SDK lock to X.Y.Z`.

## Why the SDK lock trails the tag

The new platform packages do not exist on the registry until the release
publishes them, so `npm install` cannot resolve them before step 9.
`release.yml` publishes with `npm install`, regenerating the lock on the
runner, so a lagging committed lock never blocks a release; the main gate
uses `npm --prefix sdk ci`, which fails `EUSAGE` on the mismatch. `main` is
red between steps 7 and 9, which is expected — a red `main` never blocks the
release, and a green release never proves `main`; reconcile both. **Never
hand-edit `sdk/package-lock.json`'s version fields** — a hand-bumped field
leaves the resolved pins stale.

## Standing constraints

- **`0.1.0` is the launch tag's to stake.** Interim cuts stay on `0.0.x`;
  the launch waits on `specs/distribution.md`'s launch gate.
- **`NPM_TOKEN`** is the repo secret the publish authenticates with. Never
  paste it into a transcript; rotate at the registry, then `gh secret set`.
