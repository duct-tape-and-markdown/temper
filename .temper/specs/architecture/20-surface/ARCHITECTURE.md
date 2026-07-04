+++
[satisfies.member]
rationale = "20-surface owns member carriage — module-carried, document-carried, in-place — and the Decisions that fix one feature shape across all three; the only home for how a member is authored"

[satisfies.projection]
rationale = "20-surface owns the deterministic, content-faithful projection mechanics (law 5) from the surface to the on-disk landscape — the sole architecture home for the emit law"

[satisfies.drift-engine]
rationale = "20-surface owns the drift model — emit determinism (config.stale), projection freshness against the lock, hand-edits as findings routed to the authored source — and the verbs that read it"

[satisfies.authoring-face]
rationale = "20-surface owns the authoring face — the typed module library, its emit contract, and the adoption gradient that keeps the manifest floor first-class"

[satisfies.manifest]
rationale = "20-surface owns the manifest — the inert `temper.toml` + lock serialization, its hand-writable floor form, and the emit determinism that binds face to manifest"

[provenance]
source_path = "./specs/architecture/20-surface.md"
source_hash = "76eedc456b409139f3afdeb46fcde24933da6659d30ed4048dea80718d93483d"
+++
# The config surface — author, emit, project, drift

The surface is `temper`'s **composition write surface**: the medium the harness
*lives in*. At the altitude it is the **authoring face** — a **typed module
library** in which members, kinds, packages, and the assembly are typed values,
composition is an `import`, and **emit** compiles the library into the
projection plus one inert **manifest** the gate reads (`00-intent.md`, "the
authoring face is a typed library"). At the floor it is declared data with no
language runtime: a hand-written manifest over members that live **in place**.
`init` is the one-time on-ramp (Decision below); `emit` compiles the face out;
a hand-edit to generated output is a drift finding routed to the authored
source. The source and its output are **different media** on purpose — a module
compiling to an artifact reads as src→dist, where the retired same-medium
mirror read as a copy that isn't (`00-intent.md`, the retirement Decision;
state at retirement: the `mirror-era` tag).

## The surface: the assembly over its contents

The surface has two homes (`05-model.md`), and only two:

- **The assembly** — authored as the face's config module (`temper.config.ts`,
  `defineHarness`) at the altitude, or hand-written as `temper.toml` at the
  floor; either way serialized as the **manifest**. It *binds* a package to
  each kind, declares the **requirements** (the roster) and the
  **relationships-that-must-exist**, **selects the members** (at the altitude,
  the member list is the config's imports — an authored module nobody imports
  is shelf stock the toolchain itself flags), and layers over the built-in
  floor. It is the extensional layer: what the environment contains and how it
  connects.
- **The library** (`.temper/`) holds the **authored-and-checked sources**: the
  member modules, kind definitions, and packages — typed values at the
  altitude, surface-language documents where a floor posture or a
  not-yet-migrated corpus carries them (member carriage, below). In-place
  members have no library entry at all: the landscape file is the member.
  `package` is a peer kind, not a privileged path (`15-kinds.md`); there is no
  `member/` bucket — "member" is the role a non-governing artifact plays, and
  its kind name already says so.

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
provenance + emit fingerprints — the baseline `diff` and the freshness checks
stand on, written by the tool, never hand-composed.

## Topology: source, contract, output

The surface is a **workspace tree** whose authored unit is the member (carriage
below). The three provenance classes never blur: **authored** (the face + any
floor documents), **generated-canonical** (the manifest — committed, gate-read,
hand-writable at the floor), **generated** (the lock, the projection).

```
temper.config.ts              # the ASSEMBLY authored: defineHarness — bindings,
                              #   requirements, member selection (imports)
.temper/                      # the LIBRARY: authored sources
  lock.toml                   # generated state-of-record (provenance + emit fingerprints)
  rules/collaboration.ts      # a MEMBER MODULE: typed fields, satisfies, body
  skills/operate-the-gate/    # a member module + its prose asset
    skill.ts                  #   fromFile('./SKILL.md') — long prose keeps its medium
    SKILL.md
  kinds/<name>.ts             # a custom KIND: defineKind (floor form: KIND.md)
  packages/<name>.ts          # a PACKAGE: typed clauses + guidance (floor: PACKAGE.md)
temper.toml                   # the MANIFEST: emitted (or hand-written at the floor);
                              #   the only thing the gate reads
.claude/**                    # the PROJECTION: emitted, guarded, never hand-edited
```

- **The manifest is inert data.** Every declaration the gate, CI, and the read
  verbs consume — bindings, requirements, joins, member features, mention
  edges — serializes here. No placement ever executes authoring code
  (`00-intent.md`).
- **The lock** — the generated **state-of-record**: per-member provenance and
  the emit fingerprints (`source_hash` + `emit_hash`) that `config.stale` and
  projection freshness compare against. The tool writes it; you never compose
  it.
- **A hand-written manifest is patched format-preserving** (`toml_edit`)
  whenever the tool touches it; an emitted manifest is re-emitted whole
  (nothing of the human's in it to lose).

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
**authored source**: a document-carried or in-place member is extracted; a
module-carried member arrives **pre-extracted** — its features are its declared
typed fields, bounded by the same closed vocabulary via the manifest schema
(`15-kinds.md`, "two sources, one shape"). `check` never ranges over the
projection except to verify its freshness against the lock.

## Member carriage — module, document, in-place

A member's **representation in the harness model** is one shape everywhere:
its typed fields (the clause values its package checks), its `satisfies`
claims with rationale, any demands it publishes, and its content-faithful
body. What varies is the **carriage** — how that shape is authored — and every
carriage serializes identically into the manifest:

- **Module-carried** (the altitude): the member is a typed value in the
  library — `rule({ satisfies: {...}, body: md\`...\` })`. Fields are typed
  object keys (a `satisfies` naming an undeclared requirement is a squiggle
  before the gate ever runs — the authoring wall, `10-contracts.md`); short
  prose rides an inline dedenting template; long prose stays a markdown asset
  the module declares (`fromFile`), keeping its tooling. Composition is an
  `import`: a shared fragment, a generated family of members, a target list —
  ordinary code, quarantined at authoring time.
- **Document-carried** (the floor, and any not-yet-migrated corpus): the
  member is one surface-language document — a TOML-fenced header of clause
  modules (`[clause.<field>]`, `[satisfies.<name>]`, `[requirement.<name>]`)
  over the body, patched format-preserving (`toml_edit`). The header carries
  exactly what the module's typed fields carry; the two carriages are one
  data shape in two spellings.
- **In-place** (the floor's harness members): the landscape file itself is the
  member — a `.claude/rules/x.md` byte-identical to one outside temper.
  Features are **extracted** (`15-kinds.md`); joins it participates in are
  declared in the assembly (the demand and the fill both name it there),
  because the harness format is not temper's to annotate. No projection, no
  provenance, no drift: the file is its own source.

A clause still has two sides under one name (`10-contracts.md`): the
**predicate** lives in the kind's package; the member carries that clause's
**value** — as a typed key, a header table, or an extracted feature. The
package defines the check; the member shows its value for it. `satisfies` and
published `requirement`s remain the graph's whole source: member-to-member
coupling is a **join** (`45-governance.md`), and at the altitude the join is
*also* an import — the module graph and the join graph carry the same intent,
one checked by the toolchain at the keystroke, one by the gate everywhere.
Conformance status stays **derived** — a `check` output, never persisted into
any carriage.

### Decision: three carriages, one feature shape

**Chosen:** a member is authored module-carried, document-carried, or
in-place; the manifest serializes all three to one shape, and every consumer
(gate, read verbs, graph, drift) is carriage-blind. Kinds declare which
carriages their members may use (`15-kinds.md`); migration between carriages
is mechanical (`init` lifts in-place → module; the document form is the same
data hand-spelled). **Rejected:** (a) the module as the *only* carriage —
mandatory Node for every consumer, against the gradient's floor
(`00-intent.md`); (b) the document as the only carriage — the retired mirror:
same-medium source/output illegibility, composition without imports, and the
five-surface scatter the reformulation exists to collapse; (c) per-carriage
feature shapes — a consumer that knows the carriage re-opens the
per-kind-adapter drift `15-kinds.md` closed.

### Decision: the member module's prose is a field, never a wrapper

**Chosen:** a module-carried member's body is **data the module declares** —
an inline dedenting template for short prose, a declared markdown asset
(`fromFile`) for long prose — rendered byte-deterministically at emit; the
words are the author's untouched (law 5). Interpolations in that prose are
**mentions** (below), authored per word. **Rejected:** (a) prose-only-inline —
template literals are hostile to long documents (no markdown tooling,
escaping noise), and the tax lands on exactly the artifacts (skills,
CLAUDE.md) authored most; (b) prose-only-sidecar — two files for a
three-line rule; the author picks per member; (c) a template *language* over
prose (loops, conditionals rendering different text per emit) — emitted
prose must be one authored text, byte-stable; generation of member
*families* happens at the value level (many members), never inside one
member's words.

### Decision: the data dialect is TOML; the authoring face is TypeScript

**Chosen:** everywhere the surface is **declared data** — the manifest, the
floor's document headers, a vendored package — the dialect is TOML: it parses
unambiguously (no implicit-typing traps — a type checker whose own medium has
ambiguous scalars would be self-satire), diffs line-by-line where contracts
are actually reviewed, has the Rust ecosystem's only mature format-preserving
editor (`toml_edit`) for the hand-written floor, and Taplo delivers the
emitted schema (`50-distribution.md`) as keystroke validation there. The
**authoring face's** dialect is TypeScript — chosen not for expressiveness in
the *data* (the emitted TOML is exactly as inert either way) but for what the
toolchain does at authoring time: the closed vocabularies as types, TSDoc as
the hover-guidance channel, imports as composition and as the join's
keystroke-checked half. The earlier TOML-header Decision's own escape clause
predicted this: "the language's identity is the clause-module structure, not
its spelling — swapping dialects later is a deterministic rewrite." The
structure survived; the spelling moved to where the tooling is. **Rejected:**
YAML (no format-preserving Rust editor; ambiguous scalars); JSON/JSONC
(comment-hostile); the declarative programmables (CUE/Dhall/Nickel/Pkl —
expressive *data* dialects are the unsound-proxy door; TypeScript avoids it
precisely by being fully general **and fully quarantined**: it can only ever
*produce* the inert dialect, never *be* it); a bespoke dialect (a parser and
editor ecosystem owned forever).

### Decision: a kind definition rides the same carriages as a member

**Chosen:** a custom kind is authored **module-carried** (`defineKind({...})`
— `governs`, the composed extraction, entities/relationships as typed keys,
its class-description prose as the definition's doc text, delivered as hover
guidance where a member of the kind is authored) or **document-carried** at
the floor (`.temper/kinds/<name>/KIND.md`, the TOML-fenced header over the
class prose — the form `(kind-artifact-format)` resolved, which remains the
no-Node spelling). One definition shape, two spellings — exactly the member
carriage rule, applied to the kind that governs them. **Rejected:** (a) a
bare `kind.toml` — a second file convention that strands the kind's prose;
(b) a kind-definition carriage *distinct from* member carriage — the
definitions are artifacts too (`10-contracts.md`, a package is itself a
kind), and a special spelling for governing artifacts re-opens the privilege
`15-kinds.md` closed.

## Mentions — prose that declares its own references

A module-carried member's prose may **interpolate declared values** — an
entity, a member, a requirement, a section anchor: `` prose`A ${kind} is
declared, never mined…` ``. Each interpolation is a **mention**: it renders
deterministically (one display rule corpus-wide, the entity's form and link)
and registers in the manifest as a declared **one-way edge** — the readmitted
annotation class of `45-governance.md`'s join taxonomy: resolution-checked
(a mention cannot dangle), obligation-free (the obligation graph ignores it;
`impact` reports mentions as citations, never fallout). The surrounding words
are the author's, untouched (law 5); renaming a declared value flows through
every mention — fearless refactoring reaching inside paragraphs. The
vocabulary Decision of `50-distribution.md` ("no synonyms") gains its
mechanism: guidance, diagnostics, hover docs, and spec prose interpolate the
*same values*, so a synonym is visibly a word bound to nothing.

### Decision: a mention is opt-in per word — no completeness check, ever

**Chosen:** the author marks which words are references by interpolating
them; every other word is just a word. Plain prose with zero mentions is a
fully legal member of every kind, and **no package, assembly, or engine check
may quantify over mention completeness** ("this paragraph should have modeled
its nouns") — that check class is inadmissible by definition, law 8's
never-climb bound (`00-intent.md`). **Rejected:** (a) mention-completeness or
mention-density clauses — the mining swamp rebuilt from the declaration side:
a demand that prose *be modeled* is temper adjudicating what the human should
have declared (law 4 at its finest grain); (b) auto-linking recognized names
in prose — mining with a friendlier name; recognition is authorship, never
inference.

## Genre values — prose that declares its own anatomy

A kind may declare **genres** (`15-kinds.md`): typed shapes for its members'
recurring prose forms. A **genre value** is authored module-carried
(`decision({...})`) or document-carried (the genre fence, Decision below);
its meaning-carrying fields are **prose leaves** — authored strings, law-5
protected, mention-capable at the altitude (floor leaves carry no mentions:
interpolation stays an altitude feature until a floor mention syntax is
separately ratified). Every genre value serializes into the manifest in one
shape and every consumer is carriage-blind — the carriage Decision, extended
inside the member. A leaf is **addressed** — member, genre key, field path —
and a citation may target a leaf (`45-governance.md`); `impact` reports at
leaf grain. Adoption is opt-in per block (`00-intent.md`, the genre
Decision): an unfenced block is plain prose, fully legal, and no check may
quantify over genre completeness.

### Decision: leaf addresses are structural and keyed, never positional

**Chosen:** a leaf is addressed by its position in the typed shape — member,
genre key, field path — stable under content edits, so drift, `impact`, and
citations survive rewording. At the altitude, renames flow like any refactor
— the module graph carries them; at the floor, a key rename is a bare text
edit, and its citations break to the resolution check — the citer is told,
which is the mention rule's net, accepted. Sibling collections are **keyed**
(`rejected.baked-projection.because`), never positional (`rejected[0]`) —
positional addresses die on insertion and reorder, which is exactly when
impact must survive. **Rejected:** (a) content-hash identity — dies on every
edit, defeating drift routing; (b) mandatory manual IDs on every leaf — a
toll; the address rides structure the author already writes.

### Decision: the floor spelling is a genre fence

**Chosen:** at the floor, a genre value is a **TOML genre fence** in the
document body: a fenced block whose info string names the genre and key
(info string `genre.decision surface-authority`), whose interior is TOML —
leaf fields as multi-line strings, sibling collections as keyed tables
(`[rejected.baked-projection]`). The fence is the document carriage's
declared-data medium (the header's dialect) admitted at body position:
block-grained, position-bearing, rendered in place by the display rule,
patched format-preserving (`toml_edit`), keystroke-validated where the
schema ships (`50-distribution.md`). It is declared syntax in the one
landscape where temper is itself the format authority — the surface
language documents the fence as executed (emit renders it; the gate reads
its serialization) — the body-side analogue of a structured field
(`15-kinds.md`, directives). Extraction composes the algebra's fenced-block
primitive with a TOML parse into the one manifest shape. **Rejected:**
(a) promoting the heading/bold convention (`### Decision:` + `**Chosen:**`)
to model structure — deriving graph leaves from prose typography is law 8's
mining ban; the rung-2 decision-block extraction (`15-kinds.md`, worked
example) keeps exactly its current standing: clause features, never leaves;
(b) a bespoke marker grammar — the dialect Decision already rejected owning
a parser ecosystem forever, and inside a fence there is no markdown tooling
to save; (c) header-carried genres with body anchors — splits a decision's
structure from its words.

## Artifact kinds & package binding

The kind *system* — the extraction algebra and the built-in/custom split — is
`15-kinds.md`; these are the **built-in harness kinds** the surface ships and how
`check` dispatches them. Each kind has an extractor and a **package** bound to it (its
built-in package by default). Slice 1 shipped **skill**; the next kind is **rule**
(`.claude/rules/*.md`): frontmatter `paths` (optional — the real Claude Code scoping
key) plus a content-faithful body. Its package's clauses forbid the Cursor keys Claude
Code ignores (`description`, `globs`, `alwaysApply`) — the exact mistake that motivated
the project (a rule authored with `.mdc` frontmatter loads nothing). Discovery (`init`, and
the floor's `governs` walks) scans every built-in kind at its harness locus —
**one `harness_path`, the project root, captures every kind**:
`.claude/skills/*/SKILL.md`, `.claude/rules/*.md`
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

## Content-faithful, deterministically emitted (law 5)

- **Content-faithful:** `temper` never rewords, synthesizes, or drops authored
  prose — the words are the human's, whether composed on the face, spelled in a
  floor document, or lifted in by `init`. The invariant is *authored, never
  synthesized*, not *structure only*.
- **`init` lifts once.** `init` scans an existing harness and generates the
  config skeleton over members **in place** — no file moves, no reformatting;
  lifting a member into the library (module carriage) is a per-member
  migration that normalizes framing, never content. The fixpoint lives on the
  surface: emitting, then initing the emitted output, yields the surface back.
- **Emit is byte-reproducible, and checked.** `emit` compiles the face —
  members to their harness formats, the assembly to the manifest — same
  library in, same bytes out, verified by double-emit comparison at every run:
  nondeterminism in authoring code (a timestamp, an unordered map) is a
  loud emit failure, never a silent churn. The lock records each member's
  `source_hash` + `emit_hash`; a committed output that matches neither is the
  `config.stale` finding (drift, below). Prose lands byte-identical to its
  authored text; companions are copied byte-for-byte.
- Provenance: in-place members carry none (the file is its own source);
  module- and document-carried members are anchored by the lock's
  fingerprints.
- A hand-written manifest or floor document is patched format-preserving
  (`toml_edit`); a lossy serialize-from-scratch on anything a human authors is
  forbidden. Emitted files are re-emitted whole — nothing of the human's in
  them to lose.
- **The display rule owns connective tissue.** A genre value is rendered by
  one corpus-wide **display rule** per genre — emit-owned,
  byte-deterministic. Its connective tissue (headings, the Chosen/Rejected
  labels, ordering, anchors) is projection formatting, the markdown analogue
  of the manifest's TOML syntax — never synthesized prose: every
  meaning-carrying word in the projection traces to an authored leaf or a
  declared value's rendered form (the mention rule, reused). A hand-edit to a
  rendered genre block is a drift finding routed to the authored source, same
  as any member.

## Drift — one direction, two freshness facts (the hard core)

The drift engine is what "fearless refactoring" (law 6) rests on, and the
carriage re-cut simplifies it to **one authored direction and two decidable
freshness facts**:

- **`config.stale`** — the committed manifest/projection does not match the
  lock's `source_hash`/`emit_hash` pair: the face changed and emit hasn't run,
  or emitted output was hand-altered. One finding, pointing at whichever side
  moved.
- **Projection hand-edits** are drift findings **routed to the authored
  source** — the diagnostic names the member module (or floor document) that
  owns the bytes, and the guard artifacts (the authority posture below) make
  the edit loud at write time, not just at check time. There is no reverse
  parse: you don't patch `dist/`.
- **In-place members cannot drift** — there is no projection to diverge from;
  their only freshness fact is the repository's own.

`emit` is idempotent and dry-runnable; `diff` renders what a run would change.

### Decision: `re-add` is retired — hand-edits route to the source

**Chosen:** there is no reverse round-trip from projection to surface. A
direct edit to emitted output is **drift** — surfaced by the guard at the
write boundary and by `config.stale` at the gate — and the remedy is named in
the finding: edit the owning module/document and re-emit, or (for a harness
adopting the change) lift it deliberately with `init`'s per-member migration.
**Rejected:** `re-add`'s parse-the-projection-back — load-bearing when the
mirror made source and output the same medium (an edit was ambiguous between
them), it is machinery without a case once the media differ: patching
generated output is patching `dist/`, and a tool that *merges* dist-edits
back teaches its authors the wrong home. Retiring it deletes the three-state
merge model — desired / last-applied / real — whose whole purpose was
disambiguating edits the src→dist split now prevents. (Supersedes the
three-state Decision's mechanics; the shipped surface-authority lock is what
made the retirement safe.)

### Decision: the surface is the source of truth

**Chosen:** the composition surface is canonical; what it emits (`.claude/`,
`specs/`, the manifest) is a **projection**, and a direct edit to a projection
is drift routed to the authored source (Decision above). Direct-on-disk
authoring stays first-class through **carriage**, not through reverse-parsing:
an in-place member's file *is* the source. **Rejected:** the surface as a
read-only *lens* over canonical on-disk files. The lens framing contradicts
law 7 — you cannot *compose* a harness you only mirror — and strands fearless
refactoring (law 6), which needs a surface the author authors. (Resolves
`(surface-authority)`; the reconcile-back verb the original resolution named
is retired above.)

### Decision: surface authority is a declared posture, never a baked stance

**Chosen:** how firmly the surface owns its projections is the author's
declaration — an assembly `authority` posture: **`shared`** (the default)
keeps direct on-disk authoring first-class (in-place carriage is the norm;
guards inform and route on the files that *are* emitted); **`surface`** opts
into enforcement — the altitude's posture, where projections are `dist/` and
the guard blocks. The enforcement artifacts are
**install-wired, enumerated, self-audited, reversible** (the `gate-installed`
pattern): a managed-by **note** on projections whose format tolerates
cost-free metadata (never stamped by `emit` — law 5 keeps the projection
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
(b) `emit`-stamped notes — the projection is the surface body, and a
stamping projector breaks law 5 for every downstream byte-comparison; (c)
framing the hook as a wall — multi-consumer loci (`docs/market-formats.md`)
make that a false promise. (Ratified 2026-07-03; graduates
`(surface-authority-lock)` from `docs/horizons.md` — the drift re-cut noted
there still rides behind the shipped lock.)

### Decision: discovery respects ignore rules; the backing set reads raw disk

**Chosen:** member **discovery** — the `governs` walks `init` and the scans
run — always excludes `.git/` and honors the repository's ignore rules
(`.gitignore`): a member is **authored content**, and an ignored file is by
declaration not authored here (an any-depth memory glob must not import a
dependency tree's `AGENTS.md` files as the project's own members). The
**directive-backing file set** is the opposite case and stays **raw disk**:
whether an `@import` target is backed is a fact about the filesystem the
harness reads, and law 3 fixes the safe direction — an extra file in the
backing set can only *suppress* a finding, pruning it can *forge* one. Two
sets, two rules, never merged. **Rejected:** (a) raw-disk discovery — strangers'
files as members; (b) ignore-filtered backing — forged unbacked findings on
targets that exist; (c) a temper-specific ignore file — the repository already
declares what it considers authored, and a second vocabulary would drift.

### Decision: the workspace is per-project

**Chosen:** the surface targets a **per-project** harness — the `.claude/` and
co-located artifacts of one project, located by the explicit path `init` / `check`
already take. **Rejected (for now):** managing a mirror of the global `~/.claude`,
or both at once. The per-project harness is the unit a contract gates and a session
loads; the global config is a later extension the same engine handles as another
landscape root (`30-landscapes.md`), not a redesign. (Resolves `(workspace-scope)`.)

### Decision: the projection is re-emitted; the authored floor is patched

**Chosen:** `emit` **re-emits the projection deterministically** from the
authored source — full-file, byte-stable, idempotent; the *hand-authored*
structured text (a floor manifest, a document header) is patched
format-preserving (`toml_edit`) when the tool writes it. **Supersedes** the
earlier "patch only changed fields, never re-emit" rule (`(yaml-writeback)`):
that rule was load-bearing when `.claude/` was a peer surface humans
hand-curated — with the projection generated, there is nothing of the human's
in it to lose, and determinism replaces preservation as the guarantee (YAML
now exists only on the generated side). **Rejected:** surgically patching the
projection to preserve hand edits — that blurs authored-vs-derived; a direct
edit to the projection is *drift*, and drift routes to the authored source
(the `re-add` retirement above), never to `emit`'s tiptoeing.

### Decision: `init` is the on-ramp, and adoption is a gradient

**Chosen:** `init` scans an existing harness and generates the **config
skeleton over members in place** — zero file moves, zero reformatting: a
40-artifact harness lands governed by the floor on day one, byte-identical to
the harness it was the day before. Members arrive **unrecognized** — fully
functional — and **recognition** (the intent-encoding: `satisfies` +
rationale, joins) accrues member-by-member; the pressure to recognize comes
from the author's own declared requirements failing coverage — the right
instrument — never from on-ramp ceremony. Adoption then climbs a **gradient**,
per member, all three rungs one engine and one manifest shape:

1. **gate-only** — hand-written manifest, in-place members, no Node anywhere;
2. **`init`** — the generated skeleton, still in place;
3. **altitude** — members lifted into the library as modules
   (a per-member migration: framing normalizes, content never), the assembly
   authored as the config, `emit` in the loop.

**Rejected:** (a) migration-into-a-mirror as the on-ramp (the retired
`import`) — a toll booth that copies the whole harness into a second tree
before the first finding; the on-ramp cost is what the wedge lives or dies
on; (b) an all-or-nothing altitude — the gradient is what keeps the typed
face an *earned* posture instead of a prerequisite; (c) recognition demanded
up front — unchanged from the original resolution.

## CLI surface

- `temper init [<harness-path>]` — the on-ramp: scan → config skeleton over
  members in place (Decision above); per-member `init --lift` migrates a
  member up a carriage rung.
- `temper emit [--frozen] [--dry-run]` — compile the authoring face: members
  to their harness formats, the assembly to the manifest, fingerprints to the
  lock; double-emit verified, `--frozen` refuses network (CI posture,
  `50-distribution.md`).
- `temper check [<workspace>]` — the gate: validate **conformance** (each member
  against the package its kind is bound to, `10-contracts.md`),
  **admissibility** (the assembly and each package against the definition),
  and **freshness** (`config.stale` — the committed manifest/projection
  against the lock); exit non-zero on a `required`-clause violation
  (`--deny-advisories` to also block on advisory). The gate reads only the
  manifest + lock + authored sources — never a language runtime.
  `check --harness <path>` is the **one-shot mode**: scan-internally over a raw
  harness, no workspace touched — the session-start placement's verb
  (`50-distribution.md`).
- `temper diff` — render what an `emit` would change (the drift engine's read
  face).
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

The family gains leaf grain: `impact` accepts a leaf address and reports
citations separately from fallout (`45-governance.md`); `context <address>`
emits a member's or leaf's declared neighborhood — genre slot, siblings,
citers, satisfied requirements — the pre-edit context bundle for the primary
author. Both consume only the manifest: offline, tier-1, no runtime. And
both **disclose coverage**: under the gradient a mixed-rung corpus is the
standing state, not an edge case, so every leaf-grain answer names what it
cannot see ("N documents below rung 3 — not represented at leaf grain"). An
incomplete answer wearing complete clothes erodes the read verbs exactly as
a false gate-block erodes the gate (law 1); the disclosure ships with the
verbs, never after.

Logic lives in the library; `main` is a thin `clap` dispatch that maps results to
an exit code (`.claude/rules/rust.md`).
