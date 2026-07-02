# Distribution — delivering the gate

Distribution is not a second product; it is **placing the one gate** at every
moment a harness is authored, changes, or is used. Every placement emits from the
same source — the **assembly** (built-in **packages** ⊕ the author's `temper.toml`,
`40-composition.md`) — so a placement can never drift from the gate it delivers:

| Moment | Placement | Form of the gate |
| ------ | --------- | ---------------- |
| Keystroke | editor schema | the decidable contract as JIT validation (`temper schema`) |
| Session start | `SessionStart` hook | **advisory** — checks, surfaces the verdict, instructs notify-and-approve (cannot block; Decision below) |
| Human change | CI job on PRs | "gate, don't lint," where humans collaborate |
| On demand | the CLI | the author runs it |

Same assembly, four placements, shifted as far left as the work allows.
Consumption is self-hosting aimed outward: the placements `temper` carries against
its own `.claude/` are the ones a stranger installs against theirs — no separate
external finish line (`00-intent.md`).

## The plugin — the Claude-Code-native delivery

`temper` ships as a Claude Code plugin bundling three things: the **skill** (how to
operate the gate), the **`SessionStart` hook** (law 1), and the **shipped built-in
packages** (the std-lib, embedded — so an installed `temper` has something to
check against). It is distributed through a marketplace, and it is `temper`'s first
plugin dogfood — the artifact kind `temper` exists to project.

The plugin is a **vendored surface** — generated and checked in, **built by flume
per spec** today and by `temper bundle` (self-packaging, below) in time; it is *not*
hand-curated like `.claude/` or the `packages/` std-lib sources (product territory,
`10-contracts.md`). Its files are
an instance of what `temper` projects, so `temper check` will in time gate the plugin
too. (Build's writable paths must include the plugin tree for the loop to author it.)

### Decision: the skill is mechanics, never taste

**Chosen:** the bundled skill teaches the agent to *operate the gate* — when to
`import` / `check`, how to read a diagnostic, and when to **challenge the contract**
versus fix the artifact (the `collaboration` reflex) — and the **model's vocabulary
itself** (the cleave: kind reads, package judges, member fills, assembly binds),
because an agent cannot operate a verdict spoken in words it does not hold.
Vocabulary is mechanics, not taste. **Rejected:** a skill that
advises *what a good harness is* ("write triggers like this," "keep descriptions
tight"). That advice is the tool's taste, and law 2 forbids hardcoding it; it lives
in **packages** (data, adopted by choice), never in skill prose. The split is
the dogfood of law 2 — the skill runs the checker, the **packages** carry the opinions.

### Decision: the session-start gate is advisory, not blocking

**Chosen:** the `SessionStart` hook **is the `temper` binary itself** (exec-form
command — Claude Code spawns it and reads JSON from its stdout), which checks the
project in one shot and, on a failing contract, emits the verdict as
`additionalContext` with an instruction to notify the user and get approval before
continuing. No shell wrapper, no external deps, no escaping — the binary owns the
output contract (a `claude-session-start` reporter; see Reporters below), capped to
Claude Code's 10k `additionalContext` limit. (This is `check --harness <path>` — the one-shot
import-internally mode the CLI surface names, `20-surface.md` — not the two-step
import-then-check of the author workflow.) **On a project that carries an
authored surface** (an assembly + `.temper/` exist), the hook checks the
*surface* — the two-step path — never a fresh import: a fresh import discards
recognition (the authored `satisfies` links), so every filled requirement would
read unfilled — a false positive on clean input, the exact failure law 3
forbids. The one-shot import is the fallback for a surfaceless harness. It does
not block.
**Rejected:** a hard block at session start (a `PreToolUse` hook denying the
agent's first action). Two reasons: Claude Code's `SessionStart` *cannot* block (it
only injects context and shows stderr), and — the deeper one — a hard block on a
live session is hostile, and **a hostile gate gets disabled** (law 3's failure
mode, arriving through a UX door instead of a false-positive one). The hard block
lives where it is cheap — CI, the author's terminal, the keystroke schema; at
session start the gate routes through the human. The **posture is author-declared**
(`10-contracts.md`, severity is declared): the author tunes how firmly a failing
contract is surfaced, default advisory. (Fail-loud still holds: a hook that cannot
*run* `temper` errors loudly — that is the gate being unable to check, not a
contract failing.)

## Decision: `install` projects the gate's wiring; drift keeps it synced

**Chosen:** `temper install` **projects** the gate's integration points — the
`SessionStart` hook into `.claude/settings.json`, the CI job into `.github/`, the
schema modeline into artifact headers — as ordinary artifacts under the three-state
drift engine (`20-surface.md`). It complements the plugin rather than duplicating
it: the plugin carries the hook in its own `hooks.json`, while `install` delivers
the placements a plugin *cannot* — CI workflows and schema modelines live in *your*
repo — and wires the gate for users who run the binary without the plugin.
`temper check` then verifies *its own gate is installed and undrifted*: the harness checking that its self-check is wired (law 1,
turned on itself). **Rejected:** a bespoke per-integration checksum or lockfile to
detect staleness (Lefthook's approach). `temper` already owns principled drift; a
second staleness mechanism would be redundant and could disagree with the first.
Installing the gate is just projection.

## The gate at keystroke — the emitted schema

`temper schema [--kind <k>]` emits a JSON Schema **from the assembly and its bound
packages** and wires it into frontmatter via the `# yaml-language-server: $schema=…`
modeline (the path that makes JSON Schema validate `.md` frontmatter today, no daemon;
served over LSP later). It carries two channels — which map exactly onto a package's
two: a bound package's **contract** clauses become the validation channel, its
**guidance** becomes the docs channel — and the split is the on-law guarantee:

- **validation** (the squiggle) — the **decidable clauses only**; a true positive
  by construction, so it never cries wolf at keystroke (law 3).
- **docs** (hover) — per-field guidance prose, the best-practice text `10` keeps
  *out of checks*; advisory, never gates.

Taste cannot become a squiggle — the closed algebra has no syntax for it, and
neither does the schema — so it can only ride the docs channel. The **medium
enforces law 2**: the editor delivers the decidable contract as validation and the
guidance as documentation, and cannot confuse the two.

## The gate teaches — one vocabulary, four placements

The surface's **primary author is an agent** (`00-intent.md`, positioning), and an
author who does not hold the model cannot author under it — so teaching the model
is not documentation off to the side; it is a delivery concern of the gate itself.
Four channels, four moments of contact, all placements above:

- **the skill** — the primer, *before authoring*: the vocabulary and the cleave,
  the authoring loop, how to read a verdict (mechanics per the Decision above).
- **the diagnostic** — JIT, *at the moment of failure*: a failing clause carries
  its package's colocated guidance (`10-contracts.md`) — the violation is the
  teaching moment. **Model-level failures teach the model itself**: a dangling
  `satisfies`, an unbound kind, an inadmissible package confuse the *cleave*, not
  a clause, and their diagnostics carry the engine's own guidance ("a `satisfies`
  names a requirement; requirements are declared in the assembly") — guidance with
  no package to live in, because it is the engine's.
- **the read verbs** — teaching by rendering, *during exploration*: `why` /
  `requirements` (`20-surface.md`) narrate the model over the author's own
  artifacts — the fastest teacher there is.
- **the schema docs channel** — hover, *at keystroke* (above).

### Decision: every placement speaks the corpus's vocabulary — no synonyms

**Chosen:** the corpus's nouns — kind, package, member, assembly, requirement,
clause, `satisfies`, edge — are API. Every placement (skill prose, diagnostic
text, read-verb output, schema docs) uses exactly these, one name per concept.
**Rejected:** per-surface "friendlier" wording. A synonym forks the reader's
mental model — the silent-drift failure `temper` hunts, reintroduced in its own
mouth — and the audience that most needs the teaching (the agent) is precisely
the one that pattern-matches on exact terms.

## Fail-loud delivery — the invariant

A placement that cannot run `temper` must **error, never silently skip**. A
`SessionStart` hook that no-ops because the binary is absent is precisely "a rule
that silently doesn't load" — the 2am failure `temper` exists to kill
(`00-intent.md`). The gate's transport inherits the gate's soundness bar: if it
cannot check, it fails loud; it does not wave the session through.

## Decision: acquisition rides the ecosystem's package managers

**Chosen:** ship the prebuilt binary through the channels the ecosystem already
uses — **npm** with platform-specific `optionalDependencies` (the route most
`.claude/` projects can take, since they carry a `package.json`), plus standalone
release binaries, Homebrew, and `cargo install`; the channel is auto-detected
(Biome's `BIOME_DISTRIBUTION` pattern). Fail-loud is *intrinsic* — a missing
platform binary is an install error, not a silent skip — which is why this
satisfies the invariant above. **Rejected:** a single bespoke installer, or
assuming a globally-`PATH`'d binary as the only route; the first strands the common
JS-project case, the second fails silently when absent. (Resolves
`(binary-bootstrap)`.)

## Outward seams

- **Reporters.** `temper check` emits machine formats from one diagnostic source —
  GitHub annotations inline on the PR and **SARIF** for code-scanning (findings land
  where the team reviews), plus a **`claude-session-start`** reporter that emits the
  hook payload directly (the gate above). One reporter family, every placement
  (Biome's `ci` / reporter model; `miette` already structures the diagnostics).
- **Migrate, with a fix.** `import` from a foreign tool (a Cursor `.mdc`, a
  rulesync export) is migration that *corrects on the way in*: the `rule.toml`
  `forbidden_keys` clause catches the inert `globs` / `alwaysApply` keys at the
  on-ramp — the motivating bug, fixed at import. Positioning made concrete.
- **`bundle`.** Composes an imported harness into a publishable plugin +
  `marketplace.json` (`20-surface.md`). Project-authored **packages** publish through
  this same channel — a package is a first-class publishable artifact, not vendor-only;
  the built-in std-lib packages are merely the first-party instances. The dogfood
  target: `temper`'s own plugin is itself producible by `bundle` — `temper` distributes
  itself with its own verb.
