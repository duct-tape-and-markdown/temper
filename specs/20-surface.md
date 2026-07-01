# The config surface — compose, import, project, drift

The surface is `temper`'s **composition write surface**: the typed thing the
author *composes* the harness in, the contract engine validates, and `temper`
**projects** into the project. It is the source of truth — `.claude/` and `specs/`
are projected from it, not the reverse (`00-intent.md` law 7; Decision below).
`import` is the on-ramp (an existing harness → surface), `re-add` reconciles direct
edits, and the write direction (`apply`) projects the surface back out. Built and
proven for skills in slice 1; the shape generalizes to every artifact kind and
every landscape (`30-landscapes.md`).

## The surface: the assembly over its contents

The surface has two homes (`05-model.md`), and only two:

- **`temper.toml`** (project root) is the **assembly** — it *binds* a package to each
  kind, declares the **requirements** (the roster) and the **relationships-that-must-
  exist**, and layers over the built-in floor. It is the extensional layer: what the
  environment contains and how it connects. It does **not** inline contract clauses or
  kind definitions — those live as artifacts below.
- **`.temper/`** holds the **authored-and-checked artifacts**, organized as kind-
  directories: the **members** (`skill/`, `rule/`, `spec/` — header + byte-faithful
  body), the **packages** (`package/` — the contract clauses + guidance each kind is
  checked against), and any **custom kind** definitions. `package` is a peer kind, not
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

## Topology: structured-index + markdown sidecars

The harness is ~half prose-dominant (skills, agents, `CLAUDE.md` — small typed
header + large body) and ~half structured JSON (manifests, hooks, settings). No
single inlined format serves both, so the surface is a **workspace tree**, not
one file:

- **Prose bodies stay as real `.md`**, byte-faithful and `git mv`-able.
- **Structured headers** are written format-preserving (`toml_edit`).
- **The lock** (`lock.toml`) — the contents' generated **state-of-record** (above):
  every artifact with its provenance + drift/apply fingerprints, the baseline
  `diff`/`apply` compare against. The tool writes it; you never compose it.

```
temper.toml                   # the ASSEMBLY: package↔kind bindings, requirements, relationships
.temper/
  lock.toml                   # generated state-of-record (provenance + drift fingerprints)
  skills/<name>/              # a MEMBER (kind-dir is the pluralized kind name)
    meta.toml                 # clause modules (fields · satisfies · edges) + [provenance]
    SKILL.md                  # body, byte-faithful
    <companions…>             # copied byte-for-byte
  rules/<name>/               # members
  specs/<name>/               # members
  packages/<name>/            # a PACKAGE (clauses + guidance) — a peer kind, checked by the definition
  kinds/<name>/               # a custom KIND definition (extraction + entities/relationships)
```

## The IR

One typed value per artifact kind, behind an `Artifact` sum type. Each carries
its typed fields, a byte-faithful body where it has one, an `extra` catch-all
that **preserves unknown frontmatter keys verbatim** (never dropped), companion
paths, and provenance. Skills are modelled and shipped (`src/skill.rs`); the
`disable-model-invocation` field is load-bearing (Pocock's invocation axis) and
must be in the IR.

The IR generalizes to a per-kind **extractor** (`30-landscapes.md`): parse a unit
into the structured features the contract engine validates. For a skill that is
frontmatter + body; for a spec it is headings, bindings, and declared model
elements. Extraction is the soundness boundary — it surfaces only
deterministically-decidable features, never inferred meaning.

## Each artifact directory is a representation, not a copy

`.temper/<kind>/<name>/` is not a mirror of the source file — it is the artifact's
**representation in the harness model**: **every clause that governs this artifact,
gathered per-artifact in one place**, with the byte-faithful body carried *alongside*.

A clause has two sides under one name (`10-contracts.md`): its **predicate** lives in
the kind's **package** (clauses live only in packages), and what the member carries here
is that clause's **value** for this artifact, filed under the same name (`[clause.name]`,
`[clause.description]`). The package defines the check; the member shows its value for
it — so the artifact is legible *through* its contract without duplicating the predicate.

The representation is **clause-structured**, not a flat header. Each clause is its own
**module** (a `[table]` in `meta.toml`), so the artifact is legible *through what its
contract checks* — a clear per-clause breakdown, and a labeled home for each authored
part rather than one undifferentiated blob:

- **field clauses** — `[clause.<field>]`, one per frontmatter field the contract reads
  (`value = …`), format-preserving; the artifact's own typed fields.
- **`satisfies` clauses** — `[satisfies.<requirement>]`, the requirements this artifact
  fills (`10-contracts.md`), each carrying its **rationale** (the authored *why*, law 7,
  first-class here rather than delegated and forgotten). The opt-in bindings coverage reads.
- **edge clauses** — `[edge.<target>]`, the declared references/relationships to other
  artifacts (`45-governance.md`), the graph's source — authored, never grepped from prose.
- **`[provenance]`** — generated: `source_path` + `import_hash` (the drift anchor).
- **body** — `SKILL.md`/`RULE.md`/… copied byte-for-byte (law 5), never re-rendered.

```toml
# .temper/skills/dev-standards/meta.toml — every clause governing this artifact
[clause.name]
value = "dev-standards"
[clause.description]
value = "Maintains development standards."

[satisfies.engineering-standards]
rationale = "the home for engineering-standards enforcement"

[edge.lint-runner]
relation = "depends-on"

[provenance]                       # generated, not authored
source_path = "./.claude/skills/dev-standards/SKILL.md"
import_hash = "…"
```

Field / `satisfies` / edge clauses are **authored** (the intent-encoding); `provenance`
is **generated**; the body is **carried**; conformance status is **derived** (a `check`
output, never persisted into the representation — computed, not authored). This is what
makes the surface an *authoring space* rather than a lint target: each artifact directory
holds **all the clauses that define its meaning and role**, not just its contents
(`40-composition.md`).

## Artifact kinds & package binding

The kind *system* — the extraction algebra and the built-in/custom split — is
`15-kinds.md`; these are the **built-in harness kinds** the surface ships and how
`check` dispatches them. Each kind has an extractor and a **package** bound to it (its
built-in package by default). Slice 1 shipped **skill**; the next kind is **rule**
(`.claude/rules/*.md`): frontmatter `paths` (optional — the real Claude Code scoping
key) plus a byte-faithful body. Its package's clauses forbid the Cursor keys Claude
Code ignores (`description`, `globs`, `alwaysApply`) — the exact mistake that motivated
the project (a rule authored with `.mdc` frontmatter loads nothing). `import` scans
every built-in kind (`skills/*/SKILL.md`, `.claude/rules/*.md`) plus every custom kind
the assembly declares (`40-composition.md`); `check` dispatches each member to the
package its kind is bound to. This is the path to self-hosting: `temper`'s own
`.claude/` is rules, so once the rule kind exists, `temper check` can run on its own
house.

### Decision: package binding is by artifact kind

**Chosen:** `check` binds each kind to a package — the built-in package by default
(skill → `skill.anthropic`, rule → `rule`), overridable in the assembly. **Rejected
(for now):** a single active contract, or a CLI flag to pick one — neither generalizes
to a mixed harness (skills *and* rules in one import). **Superseded:** the earlier
deferral of project-authored packages to "a later extension" — packages are now
first-class project artifacts under `.temper/packages/` (`10-contracts.md`), bound in
the assembly exactly as built-ins are; there is no privileged embedded-only tier.
(Resolves `(contract-selection)`.)

## Provenance and round-trip discipline (law 5)

- `provenance = { source_path, import_hash }`; `import_hash` is the SHA-256 of
  the original source bytes — the drift anchor, computed at import so the lock is
  complete before write-back exists.
- Bodies and companions are **copied, never re-rendered.** Only the structured
  header is written, via `toml_edit` (preserves comments/order/whitespace). A
  lossy serialize-from-scratch on anything a human edits is forbidden.
- `import` is **idempotent**: re-importing an unchanged harness yields an
  identical workspace (asserted by snapshot).
- The author may **compose** prose in the surface — it is the write surface (law
  7) — but `temper` never **synthesizes** it. Prose is stored and projected
  byte-faithfully whether it arrived by `import` or by authoring; the invariant is
  *authored, never synthesized*, not *structure only*. A composed prose body is as
  byte-faithful on projection as an imported one.

## Drift / apply — three states, never two (the hard core)

Write-back (`apply`) is the differentiating engine and the thing "fearless
refactoring" (law 6) rests on. It tracks three states: **desired** (the edited
surface), the **last-applied fingerprint**, and **real on-disk** — so it can
distinguish "the human edited the surface" from "the world drifted" and merge
rather than clobber. Drift surfaces a choice (diff · overwrite · skip · re-add);
`apply` is idempotent and dry-runnable. `re-add` (on-disk → surface) is a first-
class direction, because humans also edit the harness directly.

### Decision: the surface is the source of truth

**Chosen:** the composition surface is canonical; `.claude/` + `specs/` are a
**projection** of it (`apply`), and direct on-disk edits are reconciled back with
`re-add` (drift, above). **Rejected:** the surface as a read-only *lens* over
canonical on-disk files. The lens framing contradicts law 7 — you cannot *compose*
a harness you only mirror — and strands fearless refactoring (law 6), which needs a
surface the author authors. `re-add` keeps direct-on-disk editing first-class
without demoting the surface. (Resolves `(surface-authority)`.)

### Decision: the workspace is per-project

**Chosen:** the surface targets a **per-project** harness — the `.claude/` and
co-located artifacts of one project, located by the explicit path `import` / `check`
already take. **Rejected (for now):** managing a mirror of the global `~/.claude`,
or both at once. The per-project harness is the unit a contract gates and a session
loads; the global config is a later extension the same engine handles as another
landscape root (`30-landscapes.md`), not a redesign. (Resolves `(workspace-scope)`.)

### Decision: write-back patches changed fields, never re-emits

**Chosen:** `apply` **patches only the fields that changed**, in place — TOML
headers via `toml_edit`, YAML frontmatter by surgical field patch — leaving every
untouched byte (comments, key order, whitespace) exactly as the human wrote it.
**Rejected:** re-emitting a header by serializing it from scratch. No comment-
preserving YAML editor exists in Rust, so a full re-emit would normalize the
frontmatter and clobber human edits — the lossy round-trip law 5 forbids on
anything a human authors. (Resolves `(yaml-writeback)`.)

## CLI surface

- `temper import <harness-path> [--into <workspace>]` — scan → surface + lock.
- `temper check [<workspace>]` — the gate: validate **conformance** (each member
  against the package its kind is bound to, `10-contracts.md`) and **admissibility**
  (the assembly and each package against the definition); exit non-zero on a
  `required`-clause violation (`--deny-advisories` to also block on advisory).
- `temper diff` / `apply` / `re-add` — the drift engine (future).
- `temper bundle` — compose into a publishable plugin + `marketplace.json`
  (future; the publish verb — `50-distribution.md`).
- `temper install` — project the gate's wiring (`SessionStart` hook, CI job, schema
  modeline) into the harness, drift-synced (future; `50-distribution.md`).
- `temper schema [--kind <kind>]` — emit the assembly and its bound packages as an
  editor JSON Schema for keystroke validation (future; `50-distribution.md`).

Logic lives in the library; `main` is a thin `clap` dispatch that maps results to
an exit code (`.claude/rules/rust.md`).
