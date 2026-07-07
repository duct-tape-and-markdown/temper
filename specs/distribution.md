# Distribution — what ships, where it gates

Distribution places the one gate at every moment a harness is authored,
changed, or used; the offering places it in front of a stranger. One subject:
what ships, to whom, under what terms. Every placement runs the same compiled
program, riding the lock or embedded in the engine — none can drift.

## What ships — three channels

1. **The SDK** — one npm package, **`@dtmd/temper`** (published 2026-07-05;
   `@dtmd` is the project's own scope). The kernel nouns export from the
   root; the first-party provider face — built-in kinds and default
   contracts as exported values — from the `claude-code` subpath: adoption
   is an import, overriding is a spread. Identity travels by import, so the
   channel *is* the registry; there is no second one. Importing a kind or
   default contract is code execution at authoring time — devDependency-tier
   trust, bounded by CI's `emit --frozen` byte-compare. One package, one
   publish: core and provider cannot drift; the engine pin is declared once.
2. **The engine binary** — pinned by the SDK at an exact version
   (per-platform `optionalDependencies`), also acquirable standalone. It
   embeds a **built-in lock**: the first-party module's own emit — the same
   declaration rows an adopted harness commits — never hand-transcribed; CI
   re-derives it from the module and byte-compares. The normative property
   is **no-runtime checking**: every placement consumes committed artifacts
   plus the lock, offline, with no language runtime — the implementation
   language (Rust today) is non-normative.
3. **The plugin** — a bundle produced by `temper bundle`, publishable with a
   `marketplace.json`: the **skill** plus the **`SessionStart` hook**. The
   skill teaches mechanics — when to `install` / `emit` / `check`, how to
   read a finding, when to challenge the contract versus fix the member, and
   the kernel nouns, because an agent cannot operate a verdict spoken in
   words it does not hold. It never teaches taste: opinions live in default
   contracts, per the spine rule.

**The stranger gate:** a bare `temper check` — the binary alone, judging
against its built-in lock — gates any harness with no Node, no SDK, no
toolchain; the plugin a stranger installs is the one that gates this repo
(`intent.md`, self-hosting).

## The placements and their enforcement modes

- **Keystroke** — on a fully composed harness the toolchain is the wall:
  the SDK's types deliver the decidable contract as compile-time validation,
  TSDoc guidance as hover. For prose-authored members, `temper schema`
  generates a JSON Schema from the compiled clauses, wired via the
  `yaml-language-server` modeline `install` places. Validation is the
  decidable predicates only; guidance is advisory — taste cannot squiggle.
- **Session start** — **advisory**. The hook is the engine running `check`
  with the session-start reporter: on failure it emits the verdict as
  `additionalContext` (capped to Claude Code's 10k limit) with an instruction
  to notify the user and get approval before continuing. It surfaces and
  asks; it never blocks — a hard block on a live session is hostile, and a
  hostile gate gets disabled — and never silently passes.
- **CI** — **hard gate**, a documented two-line user-authored job (never an
  install-managed workflow file): run `temper check`; where the harness is
  SDK-emitted, re-run `emit --frozen` and byte-compare — the check that makes
  byte-reproducibility mechanical. SARIF is CI's reporter.
- **The author's terminal** — **hard**; the author runs `temper check`.
- **Per tool call** — the `PreToolUse` guard is `temper guard`; it follows
  the author's declared enforcement mode, three values split by where the
  finding goes: **block** denies the call; **warn** allows it and surfaces
  the finding in-band, into the live context; **note** allows it and records
  the finding out-of-band only — the next report, never the session.
  Default: warn.

`temper install` is the one on-ramp: discovery report, one question, every
answer flag-spelled (`--yes`), no invisible state — re-running converges.
Placements are lock-grounded, never assumed; every managed line is
content-keyed, and `check` verifies its own gate is installed and undrifted.
Every reporter (terminal, SARIF, session-start) renders one diagnostic
source, and every placement speaks the kernel nouns exactly — a synonym
forks the reader's mental model. **Fail-loud invariant:** a placement that
cannot run the engine errors, never silently skips. So the SDK pins its
engine; a missing platform binary is an install error, not a runtime shrug —
if it cannot check, it fails loud. The emit payload — the compiled program
the SDK pipes to its pinned engine — is internal, versioned in lockstep,
never a public format; the committed interface is artifacts plus the lock.

## The offering — terms and the road to a stranger

The README is the landing page: ~800 words, a tagline naming category and
differentiator (the type checker for the harness, not another linter), one
hero visual — a real `check` finding rendered by `miette`, guidance attached,
regenerable from the shipped binary, never a screenshot that drifts — and a
run-before-install quickstart: the stranger gate itself. Honest status is a
trust feature: pre-1.0 policy stated plainly, a root CHANGELOG, frequent
tagged releases; `AGENTS.md` at root, `CLAUDE.md` sourcing it.

> **New encoding — ruled 2026-07-06, rides this ceremony for ratification:**
> pre-1.0 releases carry no backward-compatibility burden.

**License:** MIT OR Apache-2.0, dual (`LICENSE-MIT` + `LICENSE-APACHE`);
never copyleft — a voluntarily adopted gate cannot carry adoption friction.
**AI authorship** is two-sided disclosure: the codebase is stated plainly as
largely agent-built under human-authored specs and gated commits, and
AI-assisted contributions are welcome with disclosure from a contributor who
can defend the change unassisted. The community surface stays small —
`CONTRIBUTING.md`; `SECURITY.md` with private reporting and an evidence bar
(reports demonstrate, never speculate); one bug form, one feature form —
while Discussions, a code of conduct, and `good-first-issue` bait wait for
a community. Launch sequencing: soft-launch to the Claude Code ecosystem
first (curated awesome-lists — submission requires the repo public ≥1 week —
plugin marketplaces, community channels); general stages second. The wedge
demo: a findings table over famous public harnesses, reproducible with one
command; every shipped capability is a relaunch moment.

**The v0.1 launch gate is mechanical:** prebuilt binaries install on all
three OSes without a Rust toolchain; the zero-config `temper check` produces
real findings on a clean machine — the quickstart survives a stranger; the
demo asset regenerates from the shipped binary. The name stays `temper` on
uncontested registry forms (crate `temper-cli`, scoped npm `@dtmd`, own
Homebrew tap) — a provisional keep reaffirmed at launch; the USPTO name
screen is the due-diligence item before it. Fail loud, never wave through.
