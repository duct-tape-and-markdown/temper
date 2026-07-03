# The config surface — author, import, project, drift

The surface is `temper`'s **composition write surface**: the medium the harness
*lives in*. A member is authored here in the **surface language** (below) — a
markdown dialect whose structure is the very structure its package's clauses
range over — and `temper` **projects** it into the project. `.claude/` and
`specs/` are compiled output — generated, deterministic, never the authored home
(`00-intent.md` law 7; Decision below). `import` is the one-time on-ramp: it
**migrates** an existing harness into the language (Decision below); `re-add`
re-parses direct on-disk edits back in; `apply` projects the surface out. Built
and proven for skills in slice 1; the shape generalizes to every artifact kind
and every landscape (`30-landscapes.md`).

## The surface: the assembly over its contents

The surface has two homes (`05-model.md`), and only two:

- **`temper.toml`** (project root) is the **assembly** — it *binds* a package to each
  kind, declares the **requirements** (the roster) and the **relationships-that-must-
  exist**, and layers over the built-in floor. It is the extensional layer: what the
  environment contains and how it connects. It does **not** inline contract clauses or
  kind definitions — those live as artifacts below.
- **`.temper/`** holds the **authored-and-checked artifacts**, organized as kind-
  directories (pluralized kind names): the **members** (`skills/`, `rules/`, `specs/` —
  one document each, header over body), the **packages** (`packages/` — the contract
  clauses + guidance each kind is checked against), and any **custom kind** definitions. `package` is a peer kind, not
  a privileged path (`15-kinds.md`); there is no `member/` bucket — "member" is the
  role a non-governing artifact plays, and its kind name already says so.

`check` is two relations, one scope apart. **Conformance:** each member satisfies the
contract its bound package carries. **Admissibility:** the assembly *and* each package
are themselves well-formed against the definition (never checked against themselves).
Two greens (`00-intent.md`). A package is thus **both** governed (admissibility checks
it) **and** governing (its clauses check members) — the reflexive nature the corpus
leans into (`05-model.md`), and the reason the governing relation lives at the assembly
rather than at any one file: `temper.toml` binds, packages check, the definition
grounds. One engine, every layer an instance (`00-intent.md`).

The `.temper/` **lock** (`lock.toml`, below) is **neither** the assembly nor a
conforming artifact: it is the contents' generated **state-of-record** —
provenance + drift/apply fingerprints — the baseline `diff`/`apply` stand on, written
by the tool, never hand-composed.

## Topology: one document per member

The harness is ~half prose-dominant (skills, agents, `CLAUDE.md` — small typed
header + large body) and ~half structured JSON (manifests, hooks, settings). The
surface is a **workspace tree**, and its unit is the **member document**:

- **A member is one authored document** in the surface language: a TOML-fenced
  structured header (the clause modules, below) over a markdown body, `git
  mv`-able, beside its companions (copied byte-for-byte). Prose kinds author in
  markdown; structured-JSON kinds (settings, hooks, manifests) author a
  TOML-native document that projects to JSON — same model, per-kind-family
  concrete syntax.
- **The surface's own structured text** is written format-preserving
  (`toml_edit`) whenever the tool touches it; the human authors it freely.
- **The lock** (`lock.toml`) — the contents' generated **state-of-record** (above):
  every artifact with its provenance + drift/apply fingerprints, the baseline
  `diff`/`apply` compare against. The tool writes it; you never compose it.

```
temper.toml                   # the ASSEMBLY: package↔kind bindings, requirements, relationships
.temper/
  lock.toml                   # generated state-of-record (provenance + drift fingerprints)
  skills/<name>/              # a MEMBER (kind-dir is the pluralized kind name)
    SKILL.md                  # ONE document: TOML-fenced header (clause modules) + body
    <companions…>             # copied byte-for-byte
  rules/<name>/               # members: RULE.md
  specs/<name>/               # members: SPEC.md
  packages/<name>/            # a PACKAGE: PACKAGE.md — same medium (10-contracts.md)
  kinds/<name>/               # a custom KIND definition: KIND.md — same medium (Decision below)
```

## The IR

One typed value per artifact kind, behind an `Artifact` sum type. Each carries
its typed fields, a content-faithful body where it has one, an `extra` catch-all
that **preserves unknown frontmatter keys verbatim** (never dropped), companion
paths, and provenance. Skills are modelled and shipped (`src/skill.rs`); the
`disable-model-invocation` field is load-bearing (Pocock's invocation axis) and
must be in the IR.

The IR generalizes to a per-kind **extractor** (`30-landscapes.md`): parse a unit
into the structured features the contract engine validates. For a skill that is
frontmatter + body; for a spec it is headings, bindings, and declared model
elements. Extraction is the soundness boundary — it surfaces only
deterministically-decidable features, never inferred meaning. It ranges over the
**member document**: the surface is canonical, and `check` never ranges over
generated output (`15-kinds.md`, the adapter).

## The member document — the surface language

`.temper/<kind-dir>/<name>/` is not a mirror of a source file — it is where the
member **lives**. Its document is the member's **representation in the harness
model**: **every clause that governs this member, gathered in one place**, with
the body authored below the header in the same file.

A clause has two sides under one name (`10-contracts.md`): its **predicate** lives in
the kind's **package** (clauses live only in packages), and what the member carries here
is that clause's **value** for this member, filed under the same name (`[clause.name]`,
`[clause.description]`). The package defines the check; the member shows its value for
it — so the member is legible *through* its contract without duplicating the predicate.

The header is **clause-structured**, not a flat blob. Each clause is its own
**module** (a `[table]` in the TOML-fenced header), so the member is legible
*through what its contract checks* — a labeled home for each authored part:

- **field clauses** — `[clause.<field>]`, one per structured field the contract reads
  (`value = …`), format-preserving; the member's own typed fields.
- **`satisfies` clauses** — `[satisfies.<requirement>]`, the requirements this member
  fills (`10-contracts.md`), each carrying its **rationale** (the authored *why*, law 7,
  first-class here rather than delegated and forgotten). The opt-in bindings coverage reads.
- **`requirement` clauses** — `[requirement.<name>]`, the demands this member
  *publishes* (`10-contracts.md`, "a requirement's publisher"): the same table the
  assembly's roster carries, at the concept's own home — how an intent spec declares
  the entities an architecture spec must satisfy (`90-spec-system.md`). Together
  with `satisfies`, these are the graph's whole source: member-to-member coupling
  is a **join** — a published demand and an opt-in claim — never a free-standing
  one-way edge clause (`45-governance.md`, "coupling is a join").
- **`[provenance]`** — generated: `source_path` + `import_hash` (the drift anchor).
- **the body** — the member's prose, below the header. Not cargo: it is authored
  *in the medium*, and a part no clause governs today is one declaration away from
  contract — a required section, a length cap, an edge from a heading. Importing a
  member is **recognizing** it: the author says what it is for and gains the power
  to dictate the requirements and standards that hold it there.

```markdown
+++
# .temper/skills/dev-standards/SKILL.md — every clause governing this member
[clause.name]
value = "dev-standards"
[clause.description]
value = "Maintains development standards."

[satisfies.engineering-standards]
rationale = "the home for engineering-standards enforcement"

[requirement.lint-runner]         # a join half: the runner opts in with
means = "the runner this skill dispatches to"   # [satisfies.lint-runner]
kind = "skill"

[provenance]                       # generated, not authored
source_path = "./.claude/skills/dev-standards/SKILL.md"
import_hash = "…"
+++

# Dev standards

<the body — the member's prose, authored here>
```

Field / `satisfies` / `requirement` clauses are **authored** (the intent-encoding); `provenance`
is **generated**; conformance status is **derived** (a `check` output, never persisted
into the document — computed, not authored). This is what makes the surface an
*authoring space* rather than a lint target: the member document holds **all the
clauses that define its meaning and role**, not just its contents
(`40-composition.md`).

### Decision: the member is one document in the surface language

**Chosen:** a member is a single markdown document — a TOML-fenced header (the
clause modules above) over the body — patched format-preserving (`toml_edit`)
when the tool writes it, authored freely by the human always. **Rejected:** (a)
a `meta.toml` + body-file split — the pipe model's residue: two files carrying
one member invite incoherence, and framing the body as a byte-carried sidecar
makes the surface a wrapper around cargo rather than the medium the member lives
in. (b) YAML frontmatter on the surface — no format-preserving YAML editor
exists in Rust; YAML belongs to the *generated* side only, where deterministic
re-emission is the discipline (Decision below).

### Decision: the header dialect is TOML

**Chosen:** the surface language's structured text — member headers,
`temper.toml`, `PACKAGE.md` headers — is TOML. The deciding constraint is
**co-authorship**: the human, the agent, and the tool write the same file, so
the medium needs a format-preserving editor (comments, order, whitespace survive
a field patch) — `toml_edit` is the only mature one in the Rust ecosystem for
any config dialect. Secondary: TOML parses unambiguously (no implicit-typing
traps — a type checker whose own medium has ambiguous scalars would be
self-satire); flat named tables diff line-by-line, where the surface is actually
reviewed; and Taplo delivers the emitted schema (`50-distribution.md`) as
keystroke validation in the authored medium. The familiarity objection (the
Claude Code audience lives in YAML frontmatter) is softened twice: the
projection they read stays native YAML, and the surface's **primary author is
the agent** (`00-intent.md`, positioning), for whom dialect familiarity is no
obstacle. **Not a one-way door:** the language's identity is the clause-module
structure, not its spelling — import parses and projection re-emits, so swapping
dialects later is a deterministic rewrite `temper` can run on its own surface.
**Rejected:** YAML (no format-preserving Rust editor; ambiguous scalars);
JSON/JSONC (comment-hostile; not an authoring medium); KDL (designed for
exactly this and node-shaped — re-examine when its tooling matures; today the
ecosystem is too thin to carry the medium); the programmable configs
(CUE/Dhall/Nickel/Pkl — expressiveness in the medium is the same unsound-proxy
door the algebra bolted); a bespoke dialect (a parser and an editor ecosystem
owned forever, against "adopt libraries for solved mechanics").

### Decision: a kind definition is `KIND.md` — one document, same medium

**Chosen:** a custom kind is authored as `.temper/kinds/<name>/KIND.md`, a
document in the surface language like every other artifact: the TOML-fenced
header carries the definition — `governs`, the composed extraction,
entities/relationships (`40-composition.md`) — and the body is the kind's own
prose: what this class of artifact *is*, for the humans and agents who will
author its members. A kind definition is not "pure structure" once the surface
is an authoring medium — the class description is exactly the prose worth
keeping beside the declaration, and the guidance channel can deliver it where a
member of the kind is being authored. **Rejected:** (a) a bare `kind.toml` — a
second file convention forking the surface into two dialects, and it strands
the kind's prose; (b) overloading another document name for the role —
`KIND.md` keeps the uppercase-document-per-directory convention (`SKILL.md`,
`PACKAGE.md`) uniform: the file names the role its directory plays. (Resolves
`(kind-artifact-format)`.)

## Artifact kinds & package binding

The kind *system* — the extraction algebra and the built-in/custom split — is
`15-kinds.md`; these are the **built-in harness kinds** the surface ships and how
`check` dispatches them. Each kind has an extractor and a **package** bound to it (its
built-in package by default). Slice 1 shipped **skill**; the next kind is **rule**
(`.claude/rules/*.md`): frontmatter `paths` (optional — the real Claude Code scoping
key) plus a content-faithful body. Its package's clauses forbid the Cursor keys Claude
Code ignores (`description`, `globs`, `alwaysApply`) — the exact mistake that motivated
the project (a rule authored with `.mdc` frontmatter loads nothing). `import` scans
every built-in kind at its harness locus — **one `harness_path`, the project
root, captures every kind**: `.claude/skills/*/SKILL.md`, `.claude/rules/*.md`
(a repo-root `skills/` tree is a *plugin* layout, the plugin kind's business,
never the project convention) — plus every custom kind
the assembly declares (`40-composition.md`); `check` dispatches each member to the
package its kind is bound to. This is the path to self-hosting: `temper`'s own
`.claude/` is rules, so once the rule kind exists, `temper check` can run on its own
house.

### Decision: package binding is by artifact kind

**Chosen:** `check` binds each kind to a package — the built-in package by default
(skill → `skill.anthropic`, rule → `rule.anthropic`; built-ins are named for
their source, `10-contracts.md`), overridable in the assembly. **Rejected
(for now):** a single active contract, or a CLI flag to pick one — neither generalizes
to a mixed harness (skills *and* rules in one import). **Superseded:** the earlier
deferral of project-authored packages to "a later extension" — packages are now
first-class project artifacts under `.temper/packages/` (`10-contracts.md`), bound in
the assembly exactly as built-ins are; there is no privileged embedded-only tier.
(Resolves `(contract-selection)`.)

## Content-faithful, deterministically projected (law 5)

- **Content-faithful:** `temper` never rewords, synthesizes, or drops authored
  prose — the words are the human's, whether composed on the surface or carried
  in by `import`. The invariant is *authored, never synthesized*, not
  *structure only*.
- **Import normalizes once.** `import` is a *parse into the surface language*,
  not a copy: framing, header layout, and file topology normalize to the
  language; content carries verbatim. A migration may reformat — the source's
  byte layout is not a contract, its content is (Decision below). The fixpoint
  lives on the surface: re-importing the surface's own projection yields the
  surface back, identically (asserted by snapshot).
- **Projection re-emits, deterministically.** `apply` compiles the member
  document to the harness format. Same surface in, same bytes out, idempotent —
  generated output never churns, and the body lands in it byte-identical to the
  surface's (content-faithful by construction). Companions are copied
  byte-for-byte.
- `provenance = { source_path, import_hash }`; `import_hash` is the SHA-256 of
  the source bytes at import — the drift anchor, computed at import so the lock
  is complete before write-back exists.
- The surface's own structured text is patched format-preserving (`toml_edit`);
  a lossy serialize-from-scratch on anything a human authors is forbidden.

## Drift / apply — three states, never two (the hard core)

Write-back (`apply`) is the differentiating engine and the thing "fearless
refactoring" (law 6) rests on. It tracks three states: **desired** (the edited
surface), the **last-applied fingerprint**, and **real on-disk** — so it can
distinguish "the human edited the surface" from "the world drifted" and merge
rather than clobber. Drift surfaces a choice (diff · overwrite · skip · re-add);
`apply` is idempotent and dry-runnable. `re-add` (on-disk → surface) is a first-
class direction, because humans and agents also edit the projection directly: it
**re-parses** the edited output and lifts the change into the member document —
a parse, not a byte-copy.

### Decision: the surface is the source of truth

**Chosen:** the composition surface is canonical; `.claude/` + `specs/` are a
**projection** of it (`apply`), and direct on-disk edits are reconciled back with
`re-add` (drift, above). **Rejected:** the surface as a read-only *lens* over
canonical on-disk files. The lens framing contradicts law 7 — you cannot *compose*
a harness you only mirror — and strands fearless refactoring (law 6), which needs a
surface the author authors. `re-add` keeps direct-on-disk editing first-class
without demoting the surface. (Resolves `(surface-authority)`.)

### Decision: surface authority is a declared posture, never a baked stance

**Chosen:** how firmly the surface owns its projections is the author's
declaration — an assembly `authority` posture: **`shared`** (the default) keeps
direct on-disk edits first-class (`re-add` reconciles; guards inform and
route); **`surface`** opts into enforcement. The enforcement artifacts are
**install-wired, enumerated, self-audited, reversible** (the `gate-installed`
pattern): a managed-by **note** on projections whose format tolerates
cost-free metadata (never stamped by `apply` — law 5 keeps the projection
content-faithful; memory projections skip the note, a comment there costs
context every session), and a **guard hook** at the provider's write boundary
that warns-and-routes under `shared` and blocks under `surface`. Degree maps
onto the existing severity vocabulary — note = information, warn = advisory,
block = required — so temper never escalates on its own determination. The
limit is stated, not solved: the hook binds one provider's writes and shared
files have other consumers, so surface authority is only as strong as the
weakest uninstrumented consumer — the note is the only universal layer and
CI the backstop wall. **Rejected:** (a) baked-in blocking — the tool
determining invasiveness on an installed surface it was merely invited onto;
(b) `apply`-stamped notes — the projection is the surface body, and a
stamping projector breaks law 5 for every downstream byte-comparison; (c)
framing the hook as a wall — multi-consumer loci (`docs/market-formats.md`)
make that a false promise. (Ratified 2026-07-03; graduates
`(surface-authority-lock)` from `docs/horizons.md` — the drift re-cut noted
there still rides behind the shipped lock.)

### Decision: the workspace is per-project

**Chosen:** the surface targets a **per-project** harness — the `.claude/` and
co-located artifacts of one project, located by the explicit path `import` / `check`
already take. **Rejected (for now):** managing a mirror of the global `~/.claude`,
or both at once. The per-project harness is the unit a contract gates and a session
loads; the global config is a later extension the same engine handles as another
landscape root (`30-landscapes.md`), not a redesign. (Resolves `(workspace-scope)`.)

### Decision: the projection is re-emitted; the surface is patched

**Chosen:** `apply` **re-emits the projection deterministically** from the
member document — full-file, byte-stable, idempotent; the *surface's own*
structured headers are patched format-preserving (`toml_edit`) when the tool
writes them. **Supersedes** the earlier "patch only changed fields, never
re-emit" rule (`(yaml-writeback)`): that rule was load-bearing when `.claude/`
was a peer surface humans hand-curated — no comment-preserving YAML editor
exists in Rust, so re-emission there was lossy. With the projection generated,
there is nothing of the human's in it to lose: the content lives in the surface,
and determinism replaces preservation as the guarantee (YAML now exists only on
the generated side). **Rejected:** surgically patching the projection to
preserve hand edits — that blurs authored-vs-derived; a direct edit to the
projection is *drift*, and drift is `re-add`'s to lift into the surface, never
`apply`'s to tiptoe around.

### Decision: import is a migration, and recognition is incremental

**Chosen:** `import` lifts an existing harness into the surface language
**once** — mechanically: clause values populated by extraction, bodies carried
content-faithful, provenance stamped. Members arrive **unrecognized** — governed
by their kind's floor, fully functional — and **recognition** (the
intent-encoding: `satisfies` + rationale, edges) accrues member-by-member
afterward. The pressure to recognize comes from the author's own declared
requirements failing coverage — the right instrument — never from import
ceremony. **Rejected:** (a) byte-preserving import — a wrapper around an opaque
blob condemns the surface to be a pipe, not a medium (law 5); the source's byte
layout is not a contract, its content is. (b) import that demands recognition up
front — a toll booth at the on-ramp; a 40-artifact harness must land governed by
the floor on day one and earn its graph over time.

## CLI surface

- `temper import <harness-path> [--into <workspace>]` — scan → surface + lock.
- `temper check [<workspace>]` — the gate: validate **conformance** (each member
  against the package its kind is bound to, `10-contracts.md`) and **admissibility**
  (the assembly and each package against the definition); exit non-zero on a
  `required`-clause violation (`--deny-advisories` to also block on advisory).
  `check --harness <path>` is the **one-shot mode**: import-internally over a raw
  harness, no workspace touched — the session-start placement's verb
  (`50-distribution.md`).
- `temper diff` / `apply` / `re-add` — the drift engine (future).
- `temper bundle` — compose into a publishable plugin + `marketplace.json`
  (future; the publish verb — `50-distribution.md`).
- `temper install` — project the gate's wiring (`SessionStart` hook, CI job, schema
  modeline) into the harness, drift-synced (future; `50-distribution.md`).
- `temper schema [--kind <kind>]` — emit the assembly and its bound packages as an
  editor JSON Schema for keystroke validation (future; `50-distribution.md`).
- `temper why <member>` — **read**: everything that holds this member in place — the
  requirements it `satisfies` (each with its authored rationale), the package its
  kind binds, its joins in and out (future; Decision below).
- `temper impact <member>` — **read**: blast radius as a verb — the
  deterministic tier-1 traversal answering "what strands if this member is
  removed or renamed": the requirements left unfilled, the `satisfies` left
  dangling, the directive edges left unbacked, the members whose reachability
  dies with it — the graph payoff `00-intent.md` promises, over the join,
  activation, and directive edges the graph already carries. (Ratified
  2026-07-03; graduates `(impact-verb)` from `docs/horizons.md`.)
- `temper requirements [<name>]` — **read**: the roster, each requirement with its
  satisfier set and coverage state; with `<name>`, one requirement's satisfiers —
  the reverse walk, and the blast radius a removal would open (future; Decision
  below).

### Decision: the CLI gains a read family — `why` and `requirements`

**Chosen:** two **read-only traversal verbs** over data `check` already computes —
`why <member>` walks the requirement↔`satisfies` edge forward (this member → the
requirements it fills, with rationale → the package governing it → its edges);
`requirements` walks the same edge in reverse (requirement → satisfier set → what a
removal would strand). They are projections, never gates: no new engine semantics,
no non-zero exit on findings — the traversal payoff the graph promises ("removing a
load-bearing entity surfaces its blast radius," `30-landscapes.md`, law 6) finally
given a verb. Their output is a **teaching surface**, not a table dump: it narrates
the model in full sentences over the author's own artifacts, in the corpus's exact
vocabulary (`50-distribution.md`, "the gate teaches"). Built after the
surface-language migration, once coverage + graph data exist to read. **Rejected:** (a) growing `check` flags into a query surface —
the gate stays a gate, and a reporting flag that answers "why" muddies a verb whose
exit code CI trusts; (b) a general `query` verb — a query language is surface
`temper` does not need for the two questions that matter, which are exactly the two
directions of the one requirement↔`satisfies` edge (`10-contracts.md`). (Resolves
`(read-verbs)`.)

Logic lives in the library; `main` is a thin `clap` dispatch that maps results to
an exit code (`.claude/rules/rust.md`).
