# Kinds — typed constructors and their runtime residue

A **kind** is a class of member — `skill`, `rule`, `hook`, `spec`. It is
declared in the SDK and consumed by the engine as data: **a typed constructor
plus five facts of runtime residue**. `10-contracts.md` owns what a member must
*satisfy* (clauses, judges); this spec owns what a member *is* — how a kind is
declared, where its members live (the two loci), how the engine reads them
without knowing them (generic extraction), what a genre is, and the three
postures an author may write in.

## A kind is a constructor plus five facts

In the SDK a kind is a plain typed surface — an interface and a constructor:

```ts
interface Skill extends Member {
  description: string;   // scalar — data
  body: Prose;           // Prose — law-5-protected words
}
export const skill = kind<Skill>({ /* the five facts */ });
```

Field typing lives **only** here, in the SDK's plain interfaces. `tsc` is the
keystroke wall: a misspelled field, a wrong type, a missing required field is a
red squiggle in the author's editor before anything reaches the engine. There
is one field concept with two species: a **scalar** field is data; a **Prose**
field is authored words law 5 protects (`00-intent.md`; `20-surface.md` owns
the Prose constructors).

Every type erases at the seam (`20-surface.md`): the engine never sees an
interface. What a kind leaves behind at runtime — its residue, riding the lock
as rows — is **five facts**:

1. **label** — the compiled debug label findings speak, so the gate uses the
   author's vocabulary ("skill `deploy`", never an anonymous node id).
2. **locus** — where members live: `at(path)` or `genre(within hosts)` (below).
3. **layout** — the shape of the on-disk artifact the member projects to and is
   read from (a lone file, identity from the stem; a directory with an entry
   file, identity from the directory name; frontmatter over a body).
4. **registration** — the declared edge between a member and the world (below).
5. **edge fields** — which of the kind's declared fields are references to
   other members, and so become graph edges (below).

Everything else the interface declares — field names, field types,
requiredness — is compile-time only. The engine is **schema-blind**: it never
checks a value against a type. What the gate checks are clauses
(`10-contracts.md`), and clauses are compiled predicates, not schemas.

### Decision: field typing lives in the SDK — there is no kind file format

**Chosen:** a kind's field schema is a plain SDK interface, checked by `tsc`
at the keystroke and erased at the seam; the runtime residue is exactly the
five facts, as data. There is no `KIND.md`, no header grammar of
`format`/`unit_shape`/`activation` keys, no hand-parsed kind file at all.
**Rejected:** a hand-authored kind grammar the SDK never read — the
half-consolidated ladder: the same schema living twice, once as the SDK's
types and once as a parsed grammar, drifting; plus the parser, the docs
channel, and the keystroke story that second home demands, all serving a
format only the engine would ever consume.

## Two loci

Every kind's members live at one of two loci:

- **`at(path)`** — members live at path globs, standard glob semantics, one
  dialect. The glob plus the layout fact locate and shape the artifact. For
  the built-in Claude Code kinds these are the harness's own conventions,
  external facts cited at the point of claim: a skill is a directory whose
  entry file is `SKILL.md` with YAML frontmatter (agentskills.io/specification,
  retrieved 2026-07-02), living at `.claude/skills/<name>/SKILL.md`
  (code.claude.com/docs/en/skills, retrieved 2026-07-02); the `CLAUDE.md`
  memory family has its own format facts — no frontmatter, `@`-imports, dual
  root locus (code.claude.com/docs/en/memory, retrieved 2026-07-02). A member
  never sets its own destination: where it lands follows from what it is.
- **`genre(within hosts)`** — members live as typed fenced blocks inside host
  documents of the named kinds. A kind at this locus is a **genre** (below).

## The engine is kind-blind — extraction is generic

The engine pipeline is extract → assemble → judge (`10-contracts.md`). The
extraction stage is **generic**: frontmatter, sections, fenced blocks, and
directives — four mechanics implemented once, driven by the five facts, never
by kind-specific code. The engine knows no kind by name; it reads artifacts
into features and hands them to compiled judges.

The extraction vocabulary — structured field, heading, section, decision
block, fenced block, line count, placement — survives as the SDK's field and
feature constructors, not as a grammar: the author composes typed fields, the
compiler lowers them onto the generic mechanics.

Extraction remains **the soundness boundary**: a clause is sound only if its
feature is deterministically extractable, so the constructors admit only
surface-decidable reads. Unsound extraction ("extract the meaning of paragraph
3") is unsayable by construction — the same closure that keeps the predicate
vocabulary too weak to lie (law 3, `00-intent.md`; `10-contracts.md`).

### Decision: kinds are declared data over generic extraction, never engine code

**Chosen:** every kind — built-in or custom — is an SDK value compiling to the
same five-fact rows, read by the same four generic mechanics; the engine
implements the mechanics once and knows no kind by name. **Rejected:** (a)
per-kind extractor modules in the engine (the `src/skill.rs` precedent) — the
field list already lives in the kind's interface, and a second home as engine
code drifts from the first; (b) a kind definition carrying arbitrary
extraction — a regex sweep, a script — the soundness boundary's escape hatch:
a kind that "extracts" a semantic property forges false positives exactly as
an unsound predicate does.

## Registration — the kind's world fact

**Registration** is the declared edge between a member and the world: the
mechanic by which the harness itself reaches the member. It is one fact with
per-kind spellings, each an external fact cited at the point of claim:

- a **skill** registers a description trigger — the named field is always in
  context; the body loads on invocation: "skill descriptions are loaded into
  context so Claude knows what's available, but full skill content only loads
  when invoked" (code.claude.com/docs/en/skills, retrieved 2026-07-02);
- a **rule** registers a path scope: "path-scoped rules trigger when Claude
  reads files matching the pattern, not on every tool use"
  (code.claude.com/docs/en/memory, retrieved 2026-07-02). An absent or blank
  `paths` field is **not** a dead edge — "rules without a `paths` field are
  loaded unconditionally and apply to all files" (same retrieval); only a
  *present* field whose globs match nothing is provably dead;
- a **memory** file registers unconditionally — `CLAUDE.md` is loaded in full
  at launch (code.claude.com/docs/en/memory, retrieved 2026-07-02);
- a **hook** registers for a lifecycle event: "hooks execute as shell commands
  at fixed lifecycle events" (code.claude.com/docs/en/memory, retrieved
  2026-07-02);
- an **MCP server** registers a connection.

The world is a node (`45-governance.md`), and it is the other endpoint of
every registration. **Reachability is graph reachability from the world**; a
member whose registration edge is dead — a blank skill description, a `paths`
list matching zero files — is decidably unreachable, the harness-shaped
analogue of dead code. Blast radius is the same closure inverted. Registration
is per-kind mechanics over per-member data: that rules register by path scope
is the kind's fact; the glob values are the member's ordinary field. Genre
members inherit registration through their host.

### Decision: registration generalizes activation

**Chosen:** one world-edge fact covering every way the world holds a member —
loading triggers, event subscriptions, connections — so reachability is plain
graph reachability from the world node, one closure for every kind.
**Rejected:** a loading-only "activation" vocabulary as its own subsystem —
it could not say what holds an MCP server or a hook in the world, so
reachability needed a bespoke checker per mechanism, and the world edge fell
out of the one graph everything else lives in.

## Edge fields

A kind names which of its declared fields are references to other members.
The assembled graph has **one edge enumeration, many sources**: declared
references (these edge fields), `satisfies`, embeds and mentions
(`20-surface.md`), and registrations. Every edge is declared, never mined
(law 8, `00-intent.md`; `45-governance.md` decides the reference case).

## Directives — format-executed body syntax

A **directive** is body-carried syntax the format authority documents as
**executed** by the harness — grammar that *does*, not prose that mentions.
The admission test is execution; an undocumented mention stays typography
forever (law 8).

- **`at-import`** — an `@path/to/file` occurrence imports the target into
  context: documented for Claude Code memory files, recursion-capped, resolved
  relative to the importing file (code.claude.com/docs/en/memory, retrieved
  2026-07-02); Gemini documents the same grammar for its memory files
  (geminicli.com/docs/cli/gemini-md, retrieved 2026-07-02). It is the harness
  spelling of the **embed** edge — `${embed(x)}` in Prose (`20-surface.md`) —
  and is never assumed for a kind whose docs are silent (a skill body's `@` is
  unmodeled until cited).

A directive yields edges as facts, with three verdicts: a target resolving to
a **member** is a member→member edge (reachability propagates over it); a
target resolving to an ungoverned **repo file** is a backed boundary edge; a
target resolving to **nothing** is an **unbacked pointer** — the importing
member's finding, silent context loss made author-time.

### Decision: execution is the admission test for body syntax

**Chosen:** extraction admits body-carried syntax iff the format authority
documents it as executed by the harness — per-syntax, per-kind, cited.
Directive edges are observed, never authored: temper reads the format's
one-way behavior as fact and demands nothing of the target. **Rejected:** (a)
treating directives as prose — a broken `@path` import is silent context loss
at runtime, the exact failure class law 1 moves to author-time; (b) a generic
reference/link primitive over prose — mined edges forge findings in both
directions (a mention is not an edge; an edge is not always mentioned), law
3's false-positive machine by another door.

## A genre is a kind at the block locus

A **genre** is not a lesser thing beside kinds — it *is* a kind whose locus is
`genre(within hosts)`: its members live as typed fenced blocks inside host
documents instead of at their own paths. The fence is the declaration — an
authored, opt-in act, so law 8 holds: nothing is recognized in plain prose.

Genre members **extend `Member`** like any other: they have names, carry
`satisfies` and `requires`, enter the one graph, and participate in
requirements **cross-kind** — a decision block inside a spec can fill a
requirement a file-grain kind declares, and vice versa. Their leaves are the
one field concept: scalar fields are data; Prose fields are
law-5-protected words, addressable one by one. Registration inherits through
the host: a block is held in the world exactly as far as its host document is.

Adoption is opt-in per block, forever — plain prose is a fully legal member of
every genre-bearing kind, and no check may quantify over genre completeness
(law 8's opt-in bound; the genre Decision in `00-intent.md`).

### Decision: a genre is a full kind, and genre checks are data, never engine

**Chosen:** genres ride the kind machinery whole — same constructor, same five
facts, one different locus — and every check over them is a clause some module
ships (law 2). **Rejected:** (a) hardcoded genre checks — a built-in ontology
of argument is the tool's taste, in the engine where it is hardest to see; a
corpus that argues differently declares its own genres with the same
machinery; (b) genres as annotations outside the member model — typed shapes
whose instances are not members, with no `satisfies`, no requirements, no
graph presence — which strands cross-kind coverage and reduces blocks to
decoration the gate cannot reason about.

## Three authoring postures

A genre-bearing corpus can be authored at three postures, and **the postures
are equal — the system is not opinionated about where you author**:

1. **Plain prose** — `file()`: the document is an authored artifact referenced
   whole. Forever legal.
2. **Embedded genres** — typed fenced blocks authored inline in markdown; the
   fence is the declaration (law 8 holds). The document stays the medium; the
   blocks enter the graph.
3. **Fully composed** — typed collections in TS; the document is pure render.

Postures 2 and 3 emit **identical bytes**: the fence format is both the
authoring template and the render target, so moving a block between them is a
mechanical, byte-stable round-trip. That shared fence format is deliberately
undesigned until its first consumer — the genre-adoption pilot — lands
(`(genre-fence-format)`, deferred 2026-07-04; the earlier TOML-fence ruling
was manifest-era residue and does not bind the pilot); its acceptance test is
exactly the byte-stable round-trip. A mention in posture 2 spells exactly as
in posture 3 — `${address}` (`20-surface.md`, the `(mention-marker)`
resolution).

No upgrade advisory, no adoption metric, no lint counting fences. Movement
between postures is author-initiated and free in both directions. `install`'s
lift is this move at scale — and like every lift it is one-time, free to
normalize framing, never to alter content (law 5).

### Decision: the postures are equal — no adoption ladder

**Chosen:** three first-class postures over one on-disk format; the tool
never ranks them. **Rejected:** an adoption ladder — postures ordered as
maturity stages, an advisory nudging prose into fences and fences into TS, a
metric counting conversions. That turns a representation choice into the
tool's taste (laws 2 and 4) and punishes the plain-prose author law 8 exists
to protect.

## Built-in and custom kinds — ownership by module

A kind is defined the same way regardless of origin; what differs is who owns
the module:

- **Built-in kinds** are the artifact kinds of a known harness — for Claude
  Code: skill, rule, memory, agent, hook, command, MCP, settings — shipped as
  a published SDK module, **`@dtmd/temper/claude-code`**, plus a compiled default
  program embedded in the engine binary so SDK-less checking works out of the
  box. temper maintains them because the formats are external and evolving: a
  skill's shape is the harness's truth, not the author's to invent. The author
  adopts them by import.
- **Custom kinds** are a project's own — its specs, ADRs, playbooks — declared
  in the project's SDK code with the same constructor and the same five facts.
  temper ships none of them. Modeling a new landscape is declaring more kinds
  (`30-landscapes.md`).

**Identity travels by import, never by string.** `kind: skill` in any
declaration references the imported value; two providers are two npm modules,
so collision is impossible and nobody pays a qualification tax (the
`(kind-harness-axis)` fork resolves here: a provider *is* a module).

Extending a built-in kind takes no new machinery. Its members' checks are
clauses attached in the assembly — `expect` and `require`
(`40-composition.md`); the shared floor is an exported clause array in
`@dtmd/temper/claude-code`, adopted by import, overridden by a spread
(`10-contracts.md`). And because extraction is generic, a project convention
on a known artifact — a `team:` key on skills — is already extracted; the
author only adds a clause over it.

### Decision: built-ins are a module, and identity is an import

**Chosen:** built-in kinds are ordinary SDK values in a published module (with
a compiled default program in the engine binary for SDK-less checking —
derived from the module's own emit, never a hand-written second spelling:
`50-distribution.md`, the derived-lock Decision); kind identity is the
imported value. **Rejected:** (a) a privileged built-in path
with custom kinds as a lesser bolt-on — the engine implements no kind either
way, so privilege would be pure accident of packaging; (b) string-name
identity with provider qualification — bare vs qualified names
(`<provider>.<name>`), collision resolution rules, registration shadowing — a
name-resolution machine rebuilt inside temper to solve the collision the
module system already makes impossible.

## Worked example: the spec corpus, swallowed

temper governs its own `specs/` with custom kinds — `intent`, `architecture`,
`process`, each class a kind at `at(specs/<class>/*.md)`
(`90-spec-system.md` owns the class structure). Generic extraction reads the
headings, decision blocks, line counts, and placement; the clauses ride the
project's own module: `max_lines` advisory at ~150; every `## Decision`
carries a `Rejected`; and the class pairing is a requirement — an `intent`
member's declared demand filled by an `architecture` member, coverage over an
ordinary cross-kind edge.

The corpus's genre target: `decision`, `law`, and `bound` genres declared on
the spec kinds, harvested from the corpus's own conventions. A decision
authored as a block is a member — it can satisfy, be required, and light up in
a blast radius — while the same demand over posture-1 prose stays a
decision-block shape check. Migration between postures is per document, author-
initiated, never demanded. This is the deepest dogfood: self-hosting
(`00-intent.md`) extended from `.claude/` to `specs/`.
