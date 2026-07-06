# Distribution — delivering the gate

Distribution is not a second product; it is **placing the one gate** at every
moment a harness is authored, changes, or is used. Every placement runs the same
compiled program — clauses compiled from SDK values, riding the lock
(`20-surface.md`) or embedded in the engine as the default program — so a
placement can never drift from the gate it delivers:

| Moment | Placement | Form of the gate |
| ------ | --------- | ---------------- |
| Keystroke | tsc over the SDK (posture 3); editor JSON Schema generated from compiled clauses (postures 1–2) | the decidable contract as JIT validation, guidance as hover |
| Session start | `SessionStart` hook → the session-start reporter | **advisory** — surfaces the verdict, instructs notify-and-approve (law 1) |
| Human change | a two-line user-authored CI job | hard gate + integrity: re-emit `--frozen`, byte-compare |
| On demand | the CLI | the author runs it |

Same program, four placements, shifted as far left as the work allows.
Consumption is self-hosting aimed outward: the placements `temper` carries
against its own `.claude/` are the ones a stranger installs against theirs — no
separate external finish line (`00-intent.md`).

## Three channels

1. **The SDK** — one ordinary npm package, **`@dtmd/temper`**: the core nouns
   export from the package root, and the first-party provider face exports
   from the `claude-code` subpath (`@dtmd/temper/claude-code`, carrying the
   built-in kinds and the built-in floors as exported values — Decision
   below). A **floor distributes as an exported clause
   array**: adoption is an import, overriding is a spread (`10-contracts.md`).
   Because identity travels by import (`15-kinds.md`), the channel *is* the
   registry — npm already carries versioning, lockfiles, and provenance
   discipline; there is no second registry and no on-disk package format to
   publish. The trust boundary is stated, not hidden: importing a floor or
   kind is code execution at authoring time — devDependency-tier trust,
   bounded by CI's `emit --frozen` byte-compare — and the gate itself never
   touches the registry.
2. **The engine binary** — pinned by the SDK at an exact version (per-platform
   `optionalDependencies`, the shape npm-distributed native tools already use),
   and also acquirable standalone for harnesses with no `package.json`.
3. **The plugin / bundle** — a harness-installable bundle whose hook runs the
   gate, produced by `temper bundle` (`20-surface.md`), publishable with a
   `marketplace.json`. Project-authored kinds and floors publish through
   channel 1 like any module; `bundle` delivers the *gate* into a harness, not
   clauses.

### Decision: one SDK package — the provider face is a subpath export

**Chosen:** the SDK ships as a single package, **`@dtmd/temper`** (published
2026-07-05; `@dtmd` is the project's own scope, satisfying `(project-name)`'s
scoped-registry ruling). The core nouns export from the root; the first-party
provider face exports from the `claude-code` subpath — `import { skill } from
"@dtmd/temper/claude-code"`. Identity still travels by import
(`15-kinds.md`): a subpath is a full module specifier, so collision stays
impossible and adoption stays an import. One repo, one version, one publish —
the core and the first-party provider cannot drift apart, and the engine pin
(channel 2) is declared exactly once. Third-party providers publish their own
packages, as before; the first party earns no privilege beyond shipping in
the same box. **Rejected:** separate core and provider packages (`temper` +
`@temper/claude-code`, the earlier sketch) — a version matrix and a second
publish ceremony for a product with exactly one first-party provider, paid
before any second provider exists to justify it. (Resolves
`(sdk-package-layout)`; ruled 2026-07-05.)

## The stranger gate

A bare `temper check` — the binary alone, with the compiled default program
embedded at build time — gates any harness: **no Node, no SDK, no toolchain**.
This is the downstream-checker positioning made concrete: `rulesync` makes a
harness portable, marketplaces distribute artifacts, and `temper` sits
downstream of both, checking what you installed — the same two greens
(`00-intent.md`, self-hosting) aimed outward. The normative property is
**no-runtime checking**: the engine, and every placement that invokes it,
consumes committed artifacts plus the lock, offline, with no language runtime.
The engine's implementation language is deliberately non-normative
(`(engine-language)` resolved 2026-07-04: the engine stays in Rust);
specs state the property, never the language.

## The plugin — the Claude-Code-native delivery

The plugin bundles the **skill** (how to operate the gate) and the
**`SessionStart` hook**; the engine it invokes carries the embedded default
program, so an installed `temper` has something to check against before its
host ever touches the SDK. The plugin is a generated surface — an instance of
what `temper` projects, itself gated by `temper check` — and the dogfood target
is that `temper`'s own plugin is produced by `temper bundle`: the tool
distributes itself with its own verb.

### Decision: the skill is mechanics, never taste

**Chosen:** the bundled skill teaches the agent to *operate the gate* — when to
`init` / `emit` / `check`, how to read a finding, when to **challenge the
contract** versus fix the artifact (the `collaboration` reflex) — and the
model's own vocabulary (the six nouns: harness, member, kind, clause,
requirement, prose), because an agent cannot operate a verdict spoken in words
it does not hold. Vocabulary is mechanics, not taste. **Rejected:** a skill
that advises *what a good harness is* ("write triggers like this"). That advice
is the tool's taste, and law 2 forbids hardcoding it; it lives in **floors**
(exported clause data, adopted by choice), never in skill prose. The skill runs
the checker; the floors carry the opinions.

### Decision: session start is advisory — a check reporter, not a verb

**Chosen:** the `SessionStart` hook is the engine binary itself (exec-form
command — Claude Code spawns it and reads JSON from its stdout) running `check`
with the **session-start reporter**: on a failing contract it emits the verdict
as `additionalContext` — capped to Claude Code's 10k `additionalContext`
limit — with an instruction to notify the user and get approval before
continuing. No shell wrapper, no second code path: the placement is a reporter
over the one diagnostic source, so it can never disagree with the terminal. On
a temper-adopted harness it checks the committed artifacts against the lock's
program; on a stranger harness, the embedded default program (the stranger
gate above). It does not block. **Rejected:** (a) a hard block at session
start — Claude Code's `SessionStart` *cannot* block (it only injects context
and shows stderr), and, the deeper reason, a hard block on a live session is
hostile, and a hostile gate gets disabled (law 3's failure mode through a UX
door). The hard block lives where it is cheap — CI, the author's terminal, the
keystroke wall; at session start the gate routes through the human, and the
enforcement posture at each placement is author-declared (law 1), default
advisory here. (b) A dedicated `session-start` verb — a reporter that grew a
CLI surface of its own is a second placement to keep in sync with the first.

## Decision: `install` is two placements, one mechanism

**Chosen:** `temper install` places exactly two things — the `SessionStart`
hook entry in `.claude/settings.json`, and the **managed header lines** in
authored artifacts (the schema modeline and the managed-by note) — and both are
**content-keyed**: each managed line carries a fingerprint of its own content,
so staleness is detected by the same one mechanism everywhere, and `temper
check` verifies its own gate is installed and undrifted (law 1, turned on
itself). **Rejected:** (a) a bespoke per-integration checksum or lockfile
(Lefthook's approach) — a second staleness mechanism could disagree with the
first; (b) an install-managed CI workflow file — CI is *your* repo's territory,
and a managed workflow is a generated file nobody reads that still needs its
own drift story. The CI placement is instead a **documented two-line
user-authored job** (below).

The installed `PreToolUse` guard is the **`temper guard` subcommand** — the
engine binary reading the hook's stdin payload; whether it blocks follows the
author's declared surface-authority posture (note / warn / block,
`20-surface.md`), **default advisory** (`(guard-posture)` resolved 2026-07-04;
the keep-Rust latency evidence is what makes the per-tool-call placement
viable). The generated-shell grep — a shell script pattern-matching
tool input — stays rejected: pattern-matching prose is mining (law 8), and a
generated script is a second implementation of the gate's judgment. The guard
installed today is advisory-only (always exit 0) — the migration-era state the
subcommand replaces.

## The gate at keystroke — one wall, two spellings by posture

The keystroke placement follows the authoring posture (`20-surface.md`):

- **Posture 3 (fully composed):** the toolchain is the wall. The SDK's plain
  interfaces deliver the decidable contract as compile-time validation — an
  inexpressible clause is unwritable — and TSDoc delivers guidance as hover.
  tsc serves both channels; no temper process is in the loop.
- **Postures 1–2 (prose and embedded genres):** `temper schema [--kind <k>]`
  generates a JSON Schema **from the compiled clauses** — the same rows the
  gate judges — covering frontmatter and typed fenced blocks, wired via the
  `# yaml-language-server: $schema=…` modeline (the managed header line
  `install` places; the path that makes JSON Schema validate `.md` frontmatter
  with no daemon).

Both spellings carry the clause's two channels, and the split is the on-law
guarantee: **validation** (the squiggle) is the decidable predicates only — a
true positive by construction, never crying wolf at keystroke (law 3) — and
**docs** (hover) is the clause's guidance, advisory, never gating. Taste cannot
become a squiggle: neither the type system nor the schema has syntax for it, so
it can only ride the docs channel. The medium enforces law 2.

## Reporters — one diagnostic source, three targets

`temper check` emits every placement's output from one diagnostic source, and
findings carry the compiled debug labels, so every reporter speaks the author's
vocabulary:

- **terminal** — the author's placement.
- **SARIF** — CI's machine format; uploaded, it lands findings as code-scanning
  annotations where the team reviews.
- **session-start** — the hook payload (the Decision above).

**Cut:** a GitHub workflow-command reporter (`::error file=…`) — SARIF upload
already covers inline annotation, and a second CI dialect is a second surface
to drift. The CI placement is a documented two-line job the user authors: run
`temper check`, and — where the harness is SDK-emitted — re-run `emit --frozen`
and byte-compare, the integrity check that makes law 5's byte-reproducibility
mechanical. Two lanes, both offline on the check side: the toolchain lane
proves the committed artifacts are what the source emits; the engine lane
gates them.

## The gate teaches — one vocabulary, every placement

The surface's primary author is an agent (`00-intent.md`, positioning), and an
author who does not hold the model cannot author under it — so teaching is a
delivery concern of the gate itself. Four moments of contact:

- **the skill** — the primer, *before authoring* (the Decision above).
- **the finding** — JIT, *at the moment of failure*: a failing clause carries
  its colocated guidance and cite (`10-contracts.md`) — the violation is the
  teaching moment. Model-level failures (a dangling `satisfies`, an unfillable
  requirement) carry the engine's own guidance, because they confuse the model,
  not a clause.
- **`explain`** — teaching by rendering, *during exploration*: the read verb
  (`20-surface.md`) narrates requirements, reachability, and blast radius over
  the author's own artifacts — the fastest teacher there is.
- **the docs channel** — hover, *at keystroke* (above).

### Decision: every placement speaks the corpus's vocabulary — no synonyms

**Chosen:** the model's nouns — harness, member, kind, clause, requirement,
prose, `expect`/`require`, `satisfies`, registration — are API. Every placement
(skill prose, finding text, `explain` output, schema docs) uses exactly these,
one name per concept. **Rejected:** per-surface "friendlier" wording. A synonym
forks the reader's mental model — the silent-drift failure `temper` hunts,
reintroduced in its own mouth — and the audience that most needs the teaching
(the agent) is precisely the one that pattern-matches on exact terms.

## Fail-loud delivery — the invariant

A placement that cannot run the engine must **error, never silently skip**. A
`SessionStart` hook that no-ops because the binary is absent is precisely "a
rule that silently doesn't load" — the 2am failure `temper` exists to kill
(`00-intent.md`). This is why the SDK **pins** its engine rather than assuming
a `PATH`'d binary, and why a missing platform binary is an install error, not a
runtime shrug: the gate's transport inherits the gate's soundness bar. If it
cannot check, it fails loud; it does not wave the session through.

## Version lockstep — the seam is not a format

The SDK pins its engine at an exact version, and the in-flight JSON between
them — the compiled program the SDK pipes to the engine — is **internal**,
versioned in lockstep, never a public format. The committed seam is artifacts
plus the lock; integrity is CI re-emitting `--frozen` and byte-comparing
(`20-surface.md`). The entry gate holds here as everywhere: a stable seam
format is earned by a consumer who needs it, not designed in advance. Until
that consumer lands, "the SDK and its pinned engine agree" is the only
compatibility promise, and it is enforced by the pin.

## Migration — `init` corrects on the way in

`init` over a foreign tool's artifacts (a Cursor `.mdc`, a rulesync export) is
migration that fixes at the on-ramp: the rule floor's clause against inert keys
catches stranded `globs` / `alwaysApply` at import — the motivating bug, caught
where it enters. `temper` sits downstream of the tools that move artifacts, and
the gate meets them at the door.
