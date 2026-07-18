# Architecture — the code tree's declared shape

The codemap: where code lives, what each home owns, and the invariants
that bound them. This page is the authority the posture sweep reads its
subsystem roster from and the `per` a tree-reorganization entry cites.
Per the practice it instantiates (matklad, "ARCHITECTURE.md", retrieved
2026-07-17): a map of the country, not an atlas — modules are named,
never line-linked; invariants are stated as absences; the page is
revised rarely, on ratified intent, never per commit.

## The tree stays flat

`src/` is a flat module list and stays one. The evidence-backed modern
practice for this size class is flat-over-nested — "even comparatively
large lists are easier to understand at a glance than even small
trees," and tree structure deteriorates where flat lists cannot
(matklad, "Large Rust Workspaces", retrieved 2026-07-17). The grouping
lives here, in the codemap; a split lands as a new flat module in its
declared subsystem, and directories are not adopted ahead of a genuine
multi-crate need, which no current subsystem has.

## Codemap — the engine (`src/`)

Seven subsystems over the flat list. Each names its one job; a module
appears in exactly one.

- **foundation** — leaf vocabulary with no internal dependencies:
  `check` (the diagnostic types every judge speaks), `extract`
  (markdown/heading mechanics), `hash`, `address`, `tap` (the telemetry
  record), `json_splice`. Nothing here knows what a harness is.
- **model** — what a harness IS: `kind` (the kind algebra),
  `contract` (clauses, predicates, selections), `compose` (member
  composition), `schema` (the interchange face), `roster` (membership
  resolution).
- **formats** — external format mechanics, implemented once
  (`representation.md`, "kind"): `frontmatter`, `document`,
  `json_manifest`, `toml_document`.
- **pipeline** — how the model becomes files and stays true:
  `drift` (emit, the lock, drift detection — the largest module and the
  pipeline's core), `import` (discovery), `read` (the read verbs'
  resolution), `builtin_lock` (the embedded default lock), `placement`
  (the managed-metadata line vocabulary install places and emit
  preserves — 0040).
- **judges** — evaluation over the resolved corpus: `engine` (clause
  evaluation), `graph` (edge resolution and the graph judges), `dial`,
  `coverage`, `coverage_note`, with `display` and `reporter` as the
  output faces.
- **provider** — the claude-code face: `builtin`, `builtin_kind` (the
  shipped kinds and their cited format facts).
- **verbs** — entry points and adoption: `main` (CLI dispatch — thin,
  per the auto-loaded rust rule), `install`, `bundle`, with
  `lib` exporting the library surface and `test_support` the shared
  fixture home (`engineering.md`, "One job, one home", test bullet).

## Invariants — stated as absences

- **foundation depends on nothing internal** — `check`, `tap`, `hash`,
  `address`, `json_splice` hold it; `extract` regains it when its
  accreted lock-row and manifest jobs land in their pipeline and
  formats homes (0040). A foundation module that grows an upward
  dependency has left the subsystem, not bent the rule.
- **formats never know the verbs or judges** — a format face is
  mechanics the pipeline selects by kind data, never a consumer of
  evaluation.
- **the provider face is data the engine loads, never a dependency of
  the model** — `kind`/`contract` compile without knowing Claude Code
  exists.
- **`main` carries dispatch only** — corpus assembly and judgment live
  in the library.

Three edges in today's tree contradicted this map; all three are ruled
to resolve toward it, code moving, map standing (0040): `drift →
install` dissolves into the `placement` module (the shared
managed-metadata vocabulary, imported downward by both); `frontmatter
→ builtin_kind` (test-only) dissolves when the adapter's fixtures
build synthetic kinds from `test_support`; `extract`'s upward imports
dissolve when its lock-row lifters land in `drift` and its
manifest-collection grammar (both faces) lands in `json_manifest`.
A fourth surfaced in the next sweep rotation and is ruled the same
way (2026-07-18): `normalize_path` is pure path vocabulary homed in
`graph` (judges) with all external callers in pipeline — it moves to
`address` (foundation), where its shape already lives. Until the
entries ship, the edges stand here as declared debt — the map is
intent, and intent loses to a better argument, never to drift.

## Codemap — the SDK (`sdk/src/`)

The authoring face mirrors the engine's layers, flat: `kind` /
`contract` / `needs` (the model face), `prose` (authored documents and
references), `assembly` + `declarations` (composition into the emit
payload), `emit` (payload production), `builtins` + `claude-code` (the
provider face), `dial`, `index` (the root export surface —
`engineering.md`, "An export earns its consumer"), `generated/` (the
ts-rs seam, machine-written).

## Growth rules

- **A new module declares its subsystem** in the entry that creates it,
  or files the fork proposing a new subsystem — a module with no
  declared home is residue against this page.
- **A new subsystem is a page amendment** — human-ratified, never
  derived; the sweep's roster follows this page, so the amendment is
  the integration.
- **Splits land flat**: a cohesion finding (`engineering.md`) extracts
  a new module into its host's subsystem; the codemap gains a name,
  the tree gains a file, no directories.
