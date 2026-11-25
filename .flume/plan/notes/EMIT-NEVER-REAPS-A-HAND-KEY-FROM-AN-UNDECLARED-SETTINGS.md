# EMIT-NEVER-REAPS-A-HAND-KEY-FROM-AN-UNDECLARED-SETTINGS

Shipped: `DriftError::OwnerlessManifestKeys` (manifest_segment_reaps) refuses by
name when a manifest with no container member carries residue keys the build
does not declare; `--teardown` does not override. Install's represented path
catches that refusal for `.claude/settings.json`, writes a `settings` member
module holding only the refused keys, splices it into `harness.ts` (import
prepended, ident opening the single `members: [` list; refuses
`HoistUnwired` otherwise), and re-emits.

Observed:
- No existing tests/it/emit.rs expectation moved: every residue-reap test
  there carries a container member, where the reap still stands.
- The ownerless manifest has no fingerprint, so a key the *harness-level*
  `settings:` option used to declare and has since dropped now refuses too
  (emit cannot tell it from a hand key). Remedy is a hand delete. Plan may
  want a lock row for ownerless manifests if that bites.
- Hand-added *registration* entries (a hook group) in an ownerless manifest
  are still ledger-reaped (`MemberReaped`), not refused. Same totality
  argument applies; out of this entry's scope.
- `install --dry-run` on such a harness surfaces the emit refusal rather
  than previewing the hoist.
- root.locus-declared guidance reworded in sdk/src/assembly.ts,
  src/builtin_lock.toml, the gauntlet snapshot and examples/base-harness lock.
  The dogfood `.temper/lock.toml` (not build-writable) still carries the old
  wording until a human re-emits.
