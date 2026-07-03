+++
[satisfies.assembly]
rationale = "40-composition owns `assembly` — the layered binding declaration, authored as the face's config or hand-written at the floor, serialized as the manifest — and the authoring loop that produces it; the only home for `assembly`"

[provenance]
source_path = "./specs/architecture/40-composition.md"
import_hash = "78e65751b399ecf651fa7140c29e11de53f1d54b4fcfea4fa5756bd23fff3b19"
+++
# Composition — authoring the harness from temper's altitude

`temper` is the **composition write surface** (`20-surface.md`): the author
composes the harness here and `temper` projects it into the project. This file
owns the **assembly** — how an author moves from "here is my harness" to "here is
the assembly that gates it": which **package** binds each kind, what the roster
requires, and where custom kinds live. It is `00-intent.md` law 2 (the author
declares; built-ins are overridable data) made into a workflow, and it closes the
home-and-selection half of `(harness-contract-provisioning)` — the
`verified_by`-wired half is settled in `10-contracts.md`.

## Built-ins are the floor, not the ceiling

`check` gates every harness against the **built-in package for each artifact
kind** (`20-surface.md`, "package binding is by artifact kind"). That is the
floor: it needs no author input and is what makes self-host green today. But a
built-in package is `temper`'s curated default, and the *harness*-level layer —
requirements, rosters, the `verified_by` seam (`10-contracts.md`) — is **specific to
one harness and cannot be a built-in**. Until the author can declare on top of the
floor, law 2 is only half-real: the packages are data, but the consumer cannot
adopt, fork, or extend them. The **assembly** is what completes it.

## The assembly — authored on the face, serialized as the manifest

The author's declaration is the **assembly** — authored as the face's config
module (`defineHarness` in `temper.config.ts`) at the altitude, or hand-written
as **`temper.toml`** at the floor; either way it serializes to the manifest the
gate reads (`20-surface.md`). It is **optional** — absent, `check` runs the
by-kind floor unchanged. Present, it **layers over** that floor — and a
gitignored local layer over *it* for personal overrides (the
committed-plus-local split Lefthook proves); it never replaces the by-kind
dispatch (which `(contract-selection)` settled — no global active-contract, no
`--contract` flag). It is a **binding declaration**, not a store of
definitions: the heavy authored material (packages, custom kinds, member
modules) lives in the library (`20-surface.md`), and the assembly *references*
it. It does five things:

- **Select the members** — at the altitude the member list is the config's
  imports, so the roster is the import graph: an authored module nobody
  imports is visible shelf stock (the toolchain flags the unused import before
  the gate does); a selected module that fails to resolve never compiles. At
  the floor, membership is the kinds' `governs` discovery, unchanged.
- **Bind a package** — name the package a kind is checked against (its built-in by
  default, made visible; or a project-authored one), so no one
  writes a contract from scratch.
- **Extend / override** — add clauses, flip a clause's severity (`required` ⟷
  `advisory`), or fork a package for a kind — typed operations on the face,
  plain tables at the floor.
- **Declare requirements** — named obligations filled by a member's opt-in `satisfies`
  (never a name-`match` — the contract never guesses), `required` or not, optionally
  typed and `verified_by`-wired (`10-contracts.md`, "Requirements"), plus the
  **relationships-that-must-exist** and compositional constraints (count / unique /
  degree / acyclic, `45-governance.md`). This is the interface/graph tier — the
  extensional part no built-in can carry.
- **Register a custom kind** — point at a project-specific kind (specs, ADRs, playbooks)
  authored in the library and bind its package (see *Registering a custom
  kind* below). Built-in kinds are adopted; custom kinds are authored below and
  registered here.

Because the altitude's assembly is ordinary code *producing* declared data,
harness **families** are functions over it — a monorepo mapping its workspace
list to per-package rules, a scenario baseline instantiated with parameters —
capability that costs the model nothing: what ships is still one inert
manifest per harness (`00-intent.md`, the authoring-face Decision).

The assembly — floor ⊕ `temper.toml`, with the packages it binds — is also the single
source of the editor schema `temper` emits (`50-distribution.md`): the same declaration
that gates the harness delivers each package's decidable clauses as keystroke
validation and its guidance prose as hover docs. One source, the gate *and* the
authoring aid.

### Decision: the assembly lives at the project root, layered — one manifest shape

**Chosen:** an optional assembly at the project root — `temper.config.ts`
emitting `temper.toml` at the altitude, `temper.toml` hand-written at the
floor — layered over the by-kind built-in floor, holding the author's member
selection, package bindings, overrides, and harness roster, referencing the
library's authored material. **Rejected:** (a) authored intent in the
*generated* lock — breaks round-trip (law 5) and blurs authored-vs-derived;
(b) the shipped built-in packages as the author's home — those are the
std-lib you adopt *from*, not where you declare; (c) two manifest shapes for
two carriages — the floor's hand TOML and the face's emitted TOML are one
schema, or the gradient is a fork. Four provenance classes — **authored**
(the face + floor documents), **generated-canonical** (the emitted manifest),
**generated** (`lock.toml`, the projection), **shipped** (built-in packages) —
keep the surface honest. (Resolves the home/selection half of
`(harness-contract-provisioning)`; carriage per the 2026-07-03 reformulation.)

## `temper.toml` binds; packages check

`temper.toml` is the **assembly**: it does not itself carry the clauses that check a
member — it *binds* the **package** that does (`20-surface.md`). The contract-over-
contents relation still holds, but one level up: the assembly composes what governs
(bindings + roster), each bound **package** checks its members' shape, and the
**definition** grounds both. This is **stratification, not contradiction** — the
assembly is authored by you *and* governs the corpus, exactly as a `trait` is authored
*and* is the contract its `impl`s meet. So the assembly and each package are checked
for **admissibility** (well-formed against the algebras, never against themselves) and
the members for **conformance**; and it all layers floor ⊕ `temper.toml` (adopt the
std-lib packages, author your own). The root/`.temper/` split is the authored boundary
between *what binds* and *what is bound* — not a config file sitting incidentally beside
a workspace.

## Registering a custom kind

A built-in kind is **adopted** — its extraction is temper's, you only bind its package
(above). A **custom** kind is **authored in the library** — `defineKind({...})`
on the face, or a floor `KIND.md` document; one definition shape, two spellings
(`20-surface.md`, the kind-carriage Decision) — the one home for a project's
own artifact kind (its specs, ADRs,
playbooks) — and **registered** in the assembly. Its definition composes the algebras
(`15-kinds.md`):

- **`governs`** — the file locus the kind reads (root + glob; file placement is
  itself an extraction primitive).
- **extraction** — the composed extractors, each a primitive at a locus naming the
  feature it yields (a frontmatter field, an ATX heading, a `## Decision` block, a
  line count).
- **relationships** (optional) — which **declared structured fields** are edges
  (`45-governance.md`; never mined from prose bodies — law 8, `00-intent.md`).
  Each unit is itself a node (its id), and **entities** are declared in member
  headers (`15-kinds.md`) — so `relationships` is all a kind adds to yield the
  dependency graph the governance graph predicates act on (`45-governance.md`,
  `30-landscapes.md`).
- **its package** — the require-side, **always a bound package** under
  `.temper/packages/`, exactly as a built-in kind binds one. A custom kind is *purely
  declare-side*; it never carries clauses itself. `[kind.<name>] package = "<name>"` in
  the assembly is the whole require-side wiring — uniform with every built-in kind
  (see the Decision below).

Discovery reads kinds from the assembly's registrations: it always scans the built-in
harness kinds, **plus every custom kind the assembly registers** — absent a
registration, built-ins only. temper reads its own `specs/` because its own assembly
registers the spec kinds, not because anything is hardwired.

### Decision: a custom kind is an authored library artifact, registered in the assembly

**Chosen:** a custom kind's **declare-side** — extraction + entities/relationships — is
**authored in the library** (`defineKind` on the face; a floor `KIND.md`), composed from the closed
algebras and **registered** by the assembly; its **require-side is always a bound
package** in the library, *never inline* — identical to how a built-in kind
binds one. **Every kind refers to a declared package**; a kind is purely declare-side,
uniformly. This is the format `(model-declaration-format)` was forwarded to but never
carried: the spec landscape's *model* is just the `spec` kind's declared entities +
relationships, now with a real, `git mv`-able, drift-tracked surface. **Rejected:** (a)
inlining the whole definition under `[kind.<name>]` in `temper.toml` — reinflates the
assembly into a god-file; (b) letting a *custom* kind carry its contract **inline** in
its definition — the same god-file one level down, and worse than built-ins (which
can't), forking the cleave so a kind is sometimes declare-only and sometimes both; (c) a
bespoke `model.toml` or per-file frontmatter markers — a second declaration mechanism
when a spec is a kind like any other; (d) a per-kind engine-code extractor (`15-kinds.md`
Decision) — the soundness escape hatch. One mechanism authors every landscape; every
kind binds a package.

## The authoring loop

Composition is one loop, all of it on the write surface:

1. **Bind** a package to each kind (or take the floor).
2. **Extend / override** to fit the harness.
3. **Declare** requirements + relationships and **wire** their verifiers.
4. **Check** — `conformance` *and* `admissibility` (`10-contracts.md`): the members
   fill their bound packages, and the assembly + each package stand up to the
   definition. An author-declared package is checked before it is trusted to gate.
5. **Emit** — `emit` compiles the surface into the project and the manifest
   (`20-surface.md`), byte-reproducible and checked, prose content-faithful.

Steps 1–3 are authoring; 4 is the gate over both checks; 5 is the compile. The loop
is what law 7 ("compose everything; gate the decidable") looks like in use.

## Scope boundary

This file provisions the **assembly** (requirements / relationships / `verified_by`
over the Claude Code harness). The **spec-landscape model** is a **custom kind**
(`15-kinds.md`) — its entities are declared by the kind's extraction, its
relationships by declared edges (`45-governance.md`), and it is authored under
`.temper/kinds/` and registered in the assembly (above). This resolves the old
`(model-declaration-format)` fork: there is no bespoke spec-model format — a spec is a
kind like any other.
