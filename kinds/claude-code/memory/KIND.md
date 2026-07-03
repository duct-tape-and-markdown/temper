+++
provider = "claude-code"
governs = { root = ".", glob = "CLAUDE.md" }
unit_shape = "file"
activation = { via = "always" }

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "sections"

[[extraction]]
primitive = "placement"
+++

# The memory kind — built-in

The declared definition of the Claude Code memory kind: a `CLAUDE.md` at the
project root — plain markdown, read into context at the start of every session
(https://code.claude.com/docs/en/memory, retrieved 2026-07-02). Identity is the
file stem. There is **no frontmatter**: the memory docs describe `CLAUDE.md` as
files you "write in plain text; Claude reads them at the start of every session,"
and document a `paths` key only for the *distinct* `.claude/rules/*.md`
mechanism, never for `CLAUDE.md` (same doc, retrieved 2026-07-02) — so this kind
declares no `format`. The sole `format` vocabulary value is `yaml-frontmatter`;
there is no "frontmatter-forbidden" member, and asserting `yaml-frontmatter` over
a frontmatterless file would misrepresent it. Omitting `format` is the accurate
statement, precedented by the project's own spec kinds, which declare none.

The extraction is markdown structure only — line budget, headings, sections,
placement — the same body-derived surface the `rule` kind carries, and correct
for a frontmatterless file (the whole file is the unit body, so a `field`
primitive would have nothing to project). What `governs` captures is the
**repo-root** `CLAUDE.md` alone (`root = "."`, `glob = "CLAUDE.md"`). Claude
Code's defining behavior is a *hierarchy* — it "reads `CLAUDE.md` files by
walking up the directory tree," concatenates every ancestor's file root-to-
working-directory, and lazy-loads descendant files when it reads under them
(same doc, retrieved 2026-07-02). That nested, nearest-last shape is not
expressible in one fixed-depth `governs` glob today; user scope
(`~/.claude/CLAUDE.md`), local scope (`CLAUDE.local.md`), and the `@path` import
graph (resolved relative to the importing file, absolute allowed, recursion
capped at four hops; same doc) are likewise out of this definition's reach —
`@path` lives in prose bodies, and the extraction algebra mines no references
from prose (`specs/architecture/15-kinds.md`, "Decision: no body-mined
references"; law 8, `specs/intent/00-intent.md`).

Kind identity is qualified `claude-code.memory` — the `provider` header names the
authority that defines the format, and placement mirrors identity
(`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider
axis"). It **deliberately shares the bare name `memory`** with `agents-md.memory`:
a bare `memory` reference no longer resolves to a unique kind, so an assembly
binding it collides with a load error naming both qualified candidates
(`KindError::AmbiguousKind`) — bind `claude-code.memory` explicitly. `unit_shape
= "file"` and `activation = { via = "always" }` are honest, typed metadata — the
governed unit is a lone file whose id is its stem, and the root `CLAUDE.md` loads
unconditionally at launch — but both are inert on the path a project-registered
kind actually takes today, carried for documentation value the way the built-in
kinds already carry activation ahead of its wiring (`src/kind.rs`, "Inert
until").

Built-in means **temper-sourced, not privileged** (`specs/architecture/15-kinds.md`):
this definition ships embedded exactly as a project's own `.temper/kinds/…`
definition would, differing only in source. `build.rs` embeds it and the engine
parses and composes its declared extraction through the generic built-in path
(`src/builtin_kind.rs`). What is **not** yet wired is discovery: `import`'s
built-in scan still names only `skill` and `rule` (`src/import.rs`), so a real
harness's `CLAUDE.md` is not imported off this `governs` locus until that scan is
generalized — shipping the `KIND.md` makes the kind parseable and resolvable, not
yet importable. Embedded *alongside* `agents-md.memory`, it also trips the eager
bare-name resolution in `builtin_kind::definitions()`; that collision is real
engine behavior, and making it pay only when an assembly references bare `memory`
is the open work this wave surfaces.
