# INTENT — long-range design rationale

`SPEC.md` (repo root) is the canonical design document; this file captures the
*why* behind it and the things that are intentionally true regardless of any one
release line.

## The one-sentence thesis

The Claude Code harness has become a real codebase but is maintained like a pile
of loose files. `author` treats it as a **typed codebase you compile**: import →
typed surface → lint/compose → apply.

## Invariants (true across all releases)

- **The surface is structured-index + markdown-sidecars, never one inlined file.**
  The harness is ~50/50 prose-dominant vs structured-JSON; bodies stay as real
  `.md` (byte-faithful, diffable, `git mv`-able), structured headers round-trip
  via `toml_edit`. (SPEC §4.)
- **Provenance is load-bearing.** Every imported artifact records its
  `source_path` + `import_hash`. This is what makes drift detection and
  write-back possible and is the real engineering cost — independent of format.
- **`apply` is three-state, never two.** desired (surface) / last-applied
  fingerprint / real on-disk, so the tool can tell "you edited the surface" from
  "the world drifted" and merge instead of clobbering. (SPEC §5.)
- **The lint `Rule` trait always sees the whole workspace**, so cross-artifact
  rules (the differentiator vs single-artifact linters) never force a signature
  change.

## Positioning

`rulesync` makes a harness *portable* across assistants (a cross-tool
fan-out transformer). `author` makes a harness *good* — quality, composition,
maintenance, in a Claude-Code-native object model. Different axis; `author` could
consume rulesync's surface rather than compete with it. (SPEC §1–2.)

## Why this is built with flume, in Rust

- **flume** drives the build (plan → build, gated commits) — dogfooding the
  user's own AI-derivation harness to construct a harness-management tool.
- **Rust** because the typed IR (algebraic artifact taxonomy + a `Rule` trait)
  and `toml_edit` (format-preserving round-trip) are genuine fits, and the
  project exercises Rust's enjoyable core without async/unsafe/perf distraction.
  The known soft spot is YAML write-back (no comment-preserving editor in Rust) —
  see the `(yaml-writeback)` open question.

## The recursive dogfood

`author`'s own `.claude/` harness is hand-curated to an exemplary standard
because it is simultaneously (1) the dev environment, (2) the environment the
flume build agents run inside, and (3) `author`'s first real fixture — the thing
it imports, lints, and projects. The product is tested against itself.
