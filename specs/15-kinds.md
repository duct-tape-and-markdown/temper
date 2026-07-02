# Kinds — the extraction algebra and the kind system

A **kind** is a class of artifact temper can read and check — `skill`, `rule`,
`spec`. Where `10-contracts.md` is the engine's *predicate* half (what an artifact
must **satisfy**), this is the *extraction* half (what an artifact **is**, and how
it is read). Two closed algebras, two instance-layers:

> predicates : contracts  ::  extraction : kinds

A kind is **data, not engine code** — the completion of "one engine, every layer an
instance" (`00-intent.md`). The engine knows no kind by name; it extracts features
and validates clauses.

## The extraction algebra — the soundness boundary, as data

Every kind is read by an **extractor**: parse a unit into the structured features a
contract validates. Extraction is **the soundness boundary** — a clause is sound
only if its feature is *deterministically extractable*; garbage extraction would
forge false positives, so extractors admit only surface-decidable features.

Today extractors are engine code (`src/skill.rs`). The end state is that extraction
is **composed from a closed algebra of deterministic extraction primitives**, the
same way a contract is composed from the closed predicate vocabulary:

- **structured field** — a frontmatter / JSON / TOML value at a key-path (kind from
  the `type` lattice, `10-contracts.md`);
- **markdown structure** — ATX headings; named sections; a `## Decision` block
  (heading + its body); a fenced block;
- **text & file** — line count; file placement, naming, glob.

An author **composes** a kind from these; an author **writes no parsing**. The
closed vocabulary makes unsound extraction ("extract the meaning of paragraph 3")
**unsayable by construction** — the identical mechanism that keeps the predicate
algebra too weak to lie (`10-contracts.md`). Two closed algebras guard the two
boundaries: what you may *read*, and what you may *require*.

### Decision: extraction is a closed algebra, not author parsing

**Chosen:** a kind's extraction is composed from a fixed, engine-provided vocabulary
of deterministic extractors; the vocabulary is **harvested from the built-in kinds**
and extended deliberately. **Rejected:** letting a kind-definition carry arbitrary
extraction — a regex sweep, a script. Arbitrary extraction is the soundness
boundary's escape hatch: a kind that "extracts" a semantic property forges false
positives exactly as an unsound proxy predicate does. The author composes
extractors; the engine implements them; a genuinely missing primitive is a
deliberate vocabulary addition (`10-contracts.md`), never a per-kind hatch.

## Two categories of kind — ownership, not mechanism

A kind is defined the same way regardless of origin; what differs is **who owns the
definition**:

- **Built-in harness kinds** — the artifact kinds of *known harnesses* (Claude Code:
  `skill`, `rule`, `memory` — the `CLAUDE.md` family, with its own format facts:
  no frontmatter, `@`-imports, dual root locus — `agent`, `hook`, `command`, MCP,
  settings, plugin; Codex; …).
  **temper-maintained**, because the format is *external and evolving* — a skill's
  shape is the harness's truth, not the author's to invent. They are temper's
  **interface** to each harness, grouped per harness, shipped as the std-lib. The
  author **adopts** them.
- **Custom project kinds** — a project's *own* artifact kinds (its specs, ADRs,
  playbooks). **Author-defined at the project level**, composing the algebras.
  Project-specific; temper ships none of them.

`spec` is a **custom** kind — and temper's own first one, governing its `specs/`
(worked example below). It is *not* a harness artifact, which is exactly why it is
the author's to define, not temper's to ship.

### Decision: built-in vs custom is ownership, not a privileged mechanism

**Chosen:** both categories are kind-definitions over the same two algebras; the
line is *who maintains the definition* — temper tracks a harness format, or the
author models their own landscape. **Rejected:** a privileged built-in path with
custom kinds as a lesser bolt-on. This is "a new landscape is a new instance, never
new engine code" (`10-contracts.md`) made literal — including the extractor. Built-
ins are simply the kind-definitions temper ships and versions as harness adapters.
The identical non-privilege governs **packages** (the require-side): a built-in
package is just the one temper ships, and a project authors its own as a peer
(`10-contracts.md`, "a package is project-authorable, not vendor-privileged"). One
ownership axis cuts across both kinds and packages (`05-model.md`).

## A built-in kind is an adapter — two faces

A built-in kind mediates between its harness format and the surface language
(`20-surface.md`): it **parses** the external format into the member document —
the `import` on-ramp; drift detection and `re-add` reuse this face — and
**emits** the member document back out (`apply`'s projection, deterministic).
Extraction — the features clauses range over — reads the **member document**:
the surface is canonical, and `check` never ranges over generated output. A
custom kind typically has no external format to adapt — its members are born on
the surface, and its projection can be near-identity.

The emit face also owns the **locus**: a member's projection target derives from
the kind's locus plus the member's id — a built-in's locus is the harness's own
convention (a skill loads from `.claude/skills/<name>/SKILL.md` or not at all),
a custom kind's is its declared `governs` (`40-composition.md`), which thus
serves both faces (the read face's scan root, the emit face's target). A member
never sets its own destination — it declares *what it is*, and where it lands
follows from what it is; `provenance.source_path` is a record of where an
imported member came from, never a setting. Nor is import a lifecycle
prerequisite: a member born on the surface projects identically, its drift
baseline established by the lock at first `apply` rather than by an import
hash.

## Extending a built-in kind

A built-in's **extraction is temper's** — it mirrors the real harness format;
redefining it would check against a fiction. Its **require-side is a package the
author binds and layers** (`40-composition.md`): adopt the built-in package, add
custom clauses, flip a severity. The effective contract is **base ∪ custom**. And
because the IR preserves unknown frontmatter keys verbatim (`20-surface.md`), a
project convention on a known artifact — a `team:` key on skills — is *already
extracted*; the author only adds a clause over it. Permissive extraction, layerable
package: use the artifact your way, check it your way.

### Decision: base-contract clauses are marked fact or opinion

**Chosen:** a built-in contract marks each clause a **harness fact** (the keys Claude
Code ignores; `name-matches-dir`) or a **best-practice opinion** (body length).
Both are overridable — temper imposes nothing (`00-intent.md` law 4) — but
downgrading a *fact* silences the exact breakage temper exists to catch, so the
marking makes that a **deliberate, visible** act, never an accident. **Rejected:** a
flat clause list where a stray `severity = "advisory"` silently guts a
harness-correctness check.

## The entity graph is a kind capability

A kind may declare **entities** (a member's header names the concepts whose one
home it is) and **relationships** (edges over **declared structured fields** —
`45-governance.md`; never mined from prose bodies — law 8, `00-intent.md`). A
kind that does yields a **dependency graph of intent**: removing a load-bearing
entity surfaces its **blast radius** deterministically (`30-landscapes.md`). So
the graph is *not* a spec-special mechanism — it is what *any* kind gets by
declaring entities + relationships, an opt-in capability layered on the closed
extraction the kind already composes.

### Decision: no body-mined references — the `references` primitive is retired

**Chosen:** the extraction algebra carries **no primitive that derives references
from a member's prose body**. Relationships range over declared structured
fields only (law 8, `00-intent.md`; the reference Decision in `45-governance.md`).
The `references` primitive — backtick filename spans, shape-tested by engine
code, suffix-normalized (`strip_suffix`) — is retired from the engine and the
vocabulary: it grepped prose and called the result structure, violating the
standing `45-governance.md` Decision, and each refinement it invited (a declared
normalization, then a declared `match` shape glob) refined the violation rather
than the model. **Rejected:** keeping it as an opt-in primitive — a
sound-looking tier-1 gate over mined edges forges findings in both directions
(prose that mentions a file is not an edge; an edge is not always mentioned),
law 3's exact failure mode. Backtick file mentions are typography, permanently.
(Supersedes `(reference-id-normalization)`; the spec corpus's real edges are
declared in member headers — `90-spec-system.md`, "the corpus is classed".)

## A kind definition — one composed object

A kind is the **declare-side** object over the extraction algebra — with no code of
its own:

- **extraction** — extractor primitives (above), each applied at a locus, each
  naming the feature it yields;
- **entities & relationships** (optional) — the concepts a member's header may
  declare as having their home here, and which declared fields are edges (the
  graph capability above).

The **require-side** is not part of the kind object — it is a **package** bound to the
kind (`10-contracts.md`), carrying the clauses (`10-contracts.md`) over those features.
Where each half lives is the ownership line (above). A **built-in** kind is temper's,
shipped as a harness adapter: its *extraction* is temper's engine code (it mirrors an
external format the author cannot redefine), and it binds a *layerable built-in
package*. A **custom** kind is the **author's, authored under `.temper/kinds/<name>/`** as its
`KIND.md` (`20-surface.md`) — extraction and relationships composed from the algebra — and
binds a package — a project-authored one under `.temper/packages/` — by the same
by-name mechanism a built-in kind binds its embedded one (`10-contracts.md`). **Every kind
binds a package; none carries its contract inline** (`40-composition.md`).

### Decision: a custom kind is declared data, never engine code

**Chosen:** a custom kind's whole definition — extraction included — is composed
from the closed algebras and **authored under `.temper/kinds/<name>/`**, registered in
the assembly; the engine implements the primitives, the author only composes them.
**Rejected:** a custom kind carrying a bespoke extractor inside temper's crate — which
is exactly what temper's own `spec` kind is *today* (`src/spec.rs` + a hardwired
`import` scan + a would-be embedded contract), built before this mechanism existed.
That ships a custom kind as a built-in, breaking "temper ships none of them" (above): a
stranger installing temper would inherit temper's `spec` kind, and a project's own kind
would have nowhere to live but a patch to temper. Engine-code extraction is sanctioned
**only** for built-in harness adapters, whose format is external and evolving. temper's
own `spec` kind is authored under its own `.temper/kinds/` like any other custom kind;
the current engine-code scaffold is superseded.

## Worked example: temper's own spec corpus, custom kinds

temper governs its `specs/` with custom kinds — authored under temper's own
`.temper/kinds/` and registered in the assembly (`40-composition.md`) by the
mechanism above, not shipped in the crate. The corpus is **classed** — `intent`,
`architecture`, `process`, each class a kind governed by placement, paired by
declared demands and `satisfies` claims in member headers (`90-spec-system.md`,
which owns the class structure):

- **extraction:** ATX headings, `## Decision` blocks, line count, placement —
  markdown structure only, no body-mined references (Decision above). The
  corpus's edges are **declared** in member headers, never extracted from prose.
- **package (each class's require-side):** `max_lines` (advisory,
  `90-spec-system.md`'s ~150); decisions-name-alternatives (every `## Decision`
  carries a `Rejected` — a predicate over the decision-block extractor); and the
  class pairing — an `intent` member's declared entities must be satisfied by an
  `architecture` member (coverage over the fill edge, `10-contracts.md`).

Piloted over the corpus it confirms every Decision names its rejected
alternative and every declared entity has its architecture home, and flags the
over-length specs. This is the deepest dogfood: temper checking the corpus flume
derives from — self-hosting (`00-intent.md`) extended from `.claude/` to
`specs/`.
