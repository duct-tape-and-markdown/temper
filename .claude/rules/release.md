---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: ["Cargo.toml","sdk/package.json","sdk/package-lock.json",".github/workflows/release.yml"]
---
# Release — cutting a temper version

A release is interactive, never a build tick. The tag is the trigger and the
published pair is the gate, not the local tree.

## The version moves in lockstep across every home

A cut bumps three homes together, and a partial bump fails the gate:

- `Cargo.toml` — the crate version `temper --version` reports.
- `sdk/package.json` — the npm driver: its own `version` **and** the
  `optionalDependencies` engine pins (`@dtmd/temper-<platform>`).
- The lockfiles — `Cargo.lock` (cargo re-syncs it on `build`) and
  `sdk/package-lock.json`.

**Never hand-edit `sdk/package-lock.json`'s version fields.** Regenerate it
with `npm --prefix sdk install`. The main gate runs `npm --prefix sdk ci`,
which fails `EUSAGE` the moment the lock's engine pins disagree with
`package.json`; a hand-bumped version field leaves the resolved pins stale and
reddens `main`.

## The lock regenerates only after the engines publish

The new-version platform packages do not exist on the registry until the
release publishes them, so `npm install` cannot resolve them before the tag.
The order is fixed:

1. Bump `Cargo.toml` and `sdk/package.json` (version + pins), write the
   CHANGELOG entry, commit.
2. Tag `vX.Y.Z` and push it — `release.yml` builds the engines and publishes.
3. Once the engines are live, `npm --prefix sdk install` regenerates the lock
   at the new version; commit the sync.

`main` is red on the lock mismatch between steps 1 and 3. That is expected and
does not block the release.

## Publish tolerates lock drift; the gate does not

`release.yml` publishes with `npm install` — it regenerates the lock on the
runner and stamps the engine pins to the tag — so a lagging committed lock
never blocks a release. The main gate uses `npm ci`, which is strict. A red
`main` never blocks the release, and a green release never proves `main`;
reconcile both.

## A cut is shipped only when smoke passes

The release's smoke job installs the published pair from the registry and
round-trips `install` → `emit` → `check`. Green build jobs and provenance do
not prove the pair works together; only smoke does.

## Standing constraints

- **`0.1.0` is the launch tag's to stake.** Interim cuts stay on `0.0.x`. A
  bump to `0.1.0` is the launch, gated on `specs/distribution.md`'s launch
  gate, not a routine cut — do not stake it until that gate is met.
- **`NPM_TOKEN`** is the repo secret the publish authenticates with. Never
  paste it into a transcript; rotate at the registry, then `gh secret set`.
- Every cut carries a `CHANGELOG.md` entry, in the `public-prose` register.
