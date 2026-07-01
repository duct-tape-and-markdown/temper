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
  (heading + its body); a markdown link or a declared reference; a fenced block;
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
  `skill`, `rule`, `agent`, `hook`, `command`, MCP, settings, plugin; Codex; …).
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

## Extending a built-in kind

A built-in's **extraction is temper's** — it mirrors the real harness format;
redefining it would check against a fiction. Its **contract is a template the
author layers on** (`40-composition.md`): adopt the base, add custom standards,
flip a severity. The effective contract is **base ∪ custom**. And because the IR
preserves unknown frontmatter keys verbatim (`20-surface.md`), a project convention
on a known artifact — a `team:` key on skills — is *already extracted*; the author
only adds a clause over it. Permissive extraction, layerable contract: use the
artifact your way, check it your way.

### Decision: base-contract clauses are marked fact or opinion

**Chosen:** a built-in contract marks each clause a **harness fact** (the keys Claude
Code ignores; `name-matches-dir`) or a **best-practice opinion** (body length).
Both are overridable — temper imposes nothing (`00-intent.md` law 4) — but
downgrading a *fact* silences the exact breakage temper exists to catch, so the
marking makes that a **deliberate, visible** act, never an accident. **Rejected:** a
flat clause list where a stray `severity = "advisory"` silently guts a
harness-correctness check.

## The entity graph is a kind capability

A kind may declare which extracted features are **entities** (a marked heading is an
entity's one home) and which references are **relationships** (over the kind's
declared reference syntax). A kind that does yields a **dependency graph of
intent**: removing a load-bearing entity surfaces its **blast radius**
deterministically (`30-landscapes.md`). So the graph is *not* a spec-special
mechanism — it is what *any* kind gets by declaring entities + relationships, an
opt-in capability layered on the closed extraction the kind already composes.

## A kind definition — one composed object

A kind is a single declared object over the two algebras — with no code of its own:

- **extraction** — extractor primitives (above), each applied at a locus, each
  naming the feature it yields;
- **entities & relationships** (optional) — which named features are entity homes
  and which references are edges (the graph capability above), over the kind's
  declared reference syntax;
- **contract** — clauses (`10-contracts.md`) over those features.

Where the definition lives is the ownership line (above). A **built-in** kind is
temper's, shipped as a harness adapter: its *extraction* is temper's engine code
(it mirrors an external format the author cannot redefine), its *contract* a
layerable template. A **custom** kind is the **author's, declared in `temper.toml`**
(`40-composition.md`) — extraction included, composed from the algebra.

### Decision: a custom kind is declared data, never engine code

**Chosen:** a custom kind's whole definition — extraction included — is composed
from the closed algebras and declared in `temper.toml`; the engine implements the
primitives, the author only composes them. **Rejected:** a custom kind carrying a
bespoke extractor inside temper's crate — which is exactly what temper's own `spec`
kind is *today* (`src/spec.rs` + a hardwired `import` scan + a would-be embedded
contract), built before this mechanism existed. That ships a custom kind as a
built-in, breaking "temper ships none of them" (above): a stranger installing
temper would inherit temper's `spec` kind, and a project's own kind would have
nowhere to live but a patch to temper. Engine-code extraction is sanctioned **only**
for built-in harness adapters, whose format is external and evolving. temper's own
`spec` kind is declared in temper's own `temper.toml` like any other custom kind;
the current engine-code scaffold is superseded.

## Worked example: `spec`, temper's own custom kind

temper governs its `specs/` with a custom `spec` kind — declared in temper's own
`temper.toml` (`40-composition.md`) by the mechanism above, not shipped in the
crate:

- **extraction:** ATX headings, `## Decision` blocks, and backtick-filename
  references (`` `NN-name.md` `` — the corpus's declared reference syntax).
- **contract:** `max_lines` (advisory, `90-spec-system.md`'s ~150);
  decisions-name-alternatives (every `## Decision` carries a `Rejected` — a
  predicate over the decision-block extractor); references-resolve (over that
  declared syntax).

Piloted over the corpus it confirms every Decision names its rejected alternative
and every cross-reference resolves, and flags the over-length specs. This is the
deepest dogfood: temper checking the corpus flume derives from — self-hosting
(`00-intent.md`) extended from `.claude/` to `specs/`.
