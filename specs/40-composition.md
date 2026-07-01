# Composition — authoring the harness from temper's altitude

`temper` is the **composition write surface** (`20-surface.md`): the author
composes the harness here and `temper` projects it into the project. This file
owns the **author-declared contract** — how an author moves from "here is my
harness" to "here is the contract that gates it," and where that declaration
lives. It is `00-intent.md` law 2 (the author declares; built-ins are overridable
data) made into a workflow, and it closes the home-and-selection half of
`(harness-contract-provisioning)` — the `verified_by`-wired half is settled in
`10-contracts.md`.

## Built-ins are the floor, not the ceiling

`check` gates every harness against the **built-in contract for each artifact
kind** (`20-surface.md`, "contract selection is by artifact kind"). That is the
floor: it needs no author input and is what makes self-host green today. But a
built-in artifact contract is `temper`'s curated default, and a *harness* contract
— roles, rosters, the `verified_by` seam (`10-contracts.md`) — is **specific to one
harness and cannot be a built-in**. Until the author can declare on top of the
floor, law 2 is only half-real: the contracts are data, but the consumer cannot
adopt, fork, or extend them. The author-declared contract is what completes it.

## The author-declared contract — `temper.toml`

The author's declaration lives in a **`temper.toml`** at the project root, beside
the harness it governs. It is **optional** — absent, `check` runs the by-kind floor
unchanged. Present, it **layers over** that floor — and a gitignored
`temper-local.toml` layers over *it* for personal overrides (the committed-plus-
local split Lefthook proves); it never replaces the by-kind dispatch (which
`(contract-selection)` settled — no global active-contract, no `--contract` flag).
It does four things:

- **Adopt** — name a shipped template as a kind's contract explicitly (the
  default made visible), so no one writes a contract from scratch.
- **Extend / override** — add clauses to a kind's contract, flip a clause's
  severity (`required` ⟷ `advisory`), or fork the template for a kind.
- **Bind the harness** — declare roles: which artifact fills role R, by a decidable
  `match`, `required` or not, with its `verified_by` (`10-contracts.md`, "Roles and
  matching"). This is the interface/trait tier, the part no built-in can carry.
- **Define a custom kind** — declare a project-specific artifact kind (specs, ADRs,
  playbooks) in full — extraction + entities/relationships + contract — under
  `[kind.<name>]` (see *Declaring a custom kind* below). Built-in kinds are adopted;
  custom kinds are authored here.

The active contract — floor ⊕ `temper.toml` — is also the single source of the
editor schema `temper` emits (`50-distribution.md`): the same declaration that
gates the harness delivers the decidable contract as keystroke validation and its
guidance prose as hover docs. One source, the gate *and* the authoring aid.

### Decision: the author-declared contract lives in `temper.toml`, layered

**Chosen:** an optional `temper.toml` at the project root, layered over the by-kind
built-in floor, holds the author's adoptions, overrides, and harness roster.
**Rejected:** (a) a field in `author.toml` — that file is *generated* (the import
roll-up: hashes, provenance); authored intent in a regenerated file breaks
round-trip (law 5) and blurs authored-vs-derived. (b) the shipped templates
(embedded; `temper`'s `contracts/`) as the author's home — those are the std-lib
you adopt *from*, not where you declare. Three separate homes — authored
(`temper.toml`), generated (`author.toml`), shipped (templates) — keep provenance
honest. (Resolves the home/selection half of `(harness-contract-provisioning)`.)

## `temper.toml` is the surface's schema stratum

`temper.toml` is not config sitting *beside* the surface — it **is** the surface's
**schema stratum** (`20-surface.md`): the types, authored alongside the artifact
instances they govern. Hence its dual role — a member of the surface you author *and*
the contract the instances satisfy — is **stratification, not contradiction** (a
`trait` is authored *and* is the contract its `impl`s meet). Accordingly `temper.toml`
is checked for **admissibility** (well-formed against the algebras, never against
itself) while the artifacts are checked for **conformance**; and it layers floor ⊕
`temper.toml` (adopt the std-lib, declare your own). This reframes the topology: the
authoring surface is `temper.toml` + the artifacts as *one* thing, not a config file
plus a separate `.temper/` workspace (`20-surface.md`).

## Declaring a custom kind

A built-in kind is **adopted** — its extraction is temper's, you only layer its
contract (above). A **custom** kind is **fully declared** under `[kind.<name>]`, the
one home for a project's own artifact kind (its specs, ADRs, playbooks), composing
the algebras (`15-kinds.md`):

- **`governs`** — the file locus the kind reads (root + glob; file placement is
  itself an extraction primitive).
- **`[kind.<name>.extraction]`** — the composed extractors, each a primitive at a
  locus naming the feature it yields (a frontmatter field, an ATX heading, a
  `## Decision` block, a backtick-filename reference, a line count).
- **`[kind.<name>.entities]` / `.relationships`** (optional) — which features are
  entity homes and which references are edges, over the declared reference syntax;
  this is what yields the dependency graph the governance graph predicates act on
  (`45-governance.md`, `30-landscapes.md`).
- **`[[kind.<name>.clause]]`** — the contract, over the extracted features.

`import` discovers kinds from this declaration: it always scans the built-in harness
kinds, **plus every custom kind the active `temper.toml` declares** — absent a
declaration, built-ins only. temper reads its own `specs/` because its own
`temper.toml` declares the `spec` kind, not because anything is hardwired.

### Decision: a custom kind is declared in `temper.toml`, extraction and all

**Chosen:** the full kind definition — extraction, entities/relationships, contract —
is declared under `[kind.<name>]`, composed from the closed algebras (built-ins
adopted, customs authored here). This is the format `(model-declaration-format)` was
forwarded to but never carried: the spec landscape's *model* is just the `spec`
kind's declared entities + relationships, now with a real surface. **Rejected:** a
bespoke `model.toml` or per-file frontmatter markers (the old fork's candidates) — a
second declaration mechanism beside kinds, when a spec is a kind like any other; and
a per-kind engine-code extractor (`15-kinds.md` Decision) — the soundness escape
hatch. One mechanism declares every landscape.

## The authoring loop

Composition is one loop, all of it on the write surface:

1. **Adopt** a template for each kind (or take the floor).
2. **Extend / override** to fit the harness.
3. **Bind** roles and **wire** their verifiers.
4. **Check** — `conformance` *and* `admissibility` (`10-contracts.md`): the harness
   fills the contract, and the `temper.toml` itself stands up to the definition. An
   author-declared contract is checked before it is trusted to gate.
5. **Project** — `apply` writes the surface into the project (`20-surface.md`),
   structure format-preserving, prose byte-faithful.

Steps 1–3 are authoring; 4 is the gate over both checks; 5 is projection. The loop
is what law 7 ("compose everything; gate the decidable") looks like in use.

## Scope boundary

This file provisions the **harness contract** (roles / `verified_by` over the
Claude Code harness). The **spec-landscape model** is a **custom kind**
(`15-kinds.md`) — its entities are declared by the kind's extraction, its
relationships by declared edges (`45-governance.md`), and a custom kind is declared
in `temper.toml` (above). This resolves the old `(model-declaration-format)` fork:
there is no bespoke spec-model format — a spec is a kind like any other.
