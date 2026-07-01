# Contracts — the two-layer model

A `Contract` is an author-declared, decidable description of structure that a
harness (or one artifact in it) must satisfy — the **require-side** of the cleave
(`05-model.md`), and *only* that: never a synonym for `temper.toml` (that is the
**assembly**) nor for the reusable bundle that carries it (that is a **package**). A
contract is what a **package** *contains*. `temper check` validates the imported
surface against the assembly and its bound packages, and reports conformance. This is
the type checker; the contract is the types. (`00-intent.md` laws 2–3.)

`temper` runs **two checks**, not one. **Conformance** — the harness satisfies its
assembly and the packages it binds (the type checker above). **Admissibility** — the
assembly and each package satisfy *the definition*: the closed algebra below is both
the vocabulary a contract is written in *and* the contract a contract must satisfy. A
type system checks values against types *and* checks the types are well-formed;
`temper` checks harness-against-package *and* package-against-definition (`00-intent.md`
finish line — both greens). Admissibility is what lets an author *declare* a package
(`40-composition.md`) without re-opening the heuristic swamp by the front door
(Decision below).

## The engine is generic; everything is an instance

There are not "two kinds of contract." There is one engine over the primitive
algebra (below), and every contract is an **instance** expressed in it. The
distinctions are compositional, not built-in:

| Instance | Declares | `temper` checks | Analogy | Home |
| -------- | -------- | --------------- | ------- | ---- |
| **Artifact contract** | the shape of one artifact kind | each artifact conforms | a **type** | a **package** |
| **Harness contract** | required requirements + relations + verifiers across a harness | each requirement is filled and wired | an **interface / trait** | the **assembly** |
| **Spec contract** | the declared domain model + how prose binds to it (`30-landscapes.md`) | the model is coherent; prose binds; the graph resolves | a **schema / ontology** | the `spec` kind's package + assembly |

The engine knows none of these names — it validates primitive clauses over
extracted features. A new landscape is a new instance, never new engine code.
This is the structural reason the project cannot rot into heuristics (`00-intent.md`
law: one engine, every layer an instance): there is nowhere to hardcode an
opinion.

## The primitive algebra (decidable only)

A contract is **pure declarative data** over a fixed, closed vocabulary of
decidable predicates. There is no arbitrary-code clause. Adding a predicate to
the vocabulary is a deliberate language change, never a per-contract escape
hatch (law 3). The primitives:

- **field** — `required` / `optional`; `type`; `allowed_chars` (a declared
  character class, e.g. `[a-z0-9-]`); `max_len` / `min_len`; `range` `{min, max}`
  over `integer`/`number` (a numeric bound is a fact — motivated in
  `45-governance.md`); `enum`; `deny` (forbidden values); `forbidden_keys` (e.g.
  the Cursor `globs`/`alwaysApply` keys Claude Code ignores). The full `pattern`
  (arbitrary regex) clause is **held back** — see the `allowed_chars` Decision
  below.
- **structural** — `max_lines`; `require_sections` (named headings present);
  `must_define` (a field/marker exists, e.g. `disable-model-invocation`);
  `section_contains` `{heading, marker}` (every section whose heading starts with
  the declared text carries the declared marker — the predicate behind the spec
  kind's decisions-name-alternatives, `15-kinds.md`).
- **referential** — a reference resolves — *only over a precisely declared
  reference syntax* (e.g. markdown links, an explicit `@path`), never by grepping
  prose. If the author can't name the reference syntax, the clause is not
  admissible (this is exactly what made the slice-1 `companion-refs` rule unsound;
  see Decision below).
- **cross-artifact** — names unique within a kind; `name-matches-dir`. (A general
  "a declared dependency exists" predicate is **held back** — like `pattern`, below —
  until it names a decidable reference syntax and an extractor to resolve it. Without
  one the engine can only return *indeterminate* — a silent no-op law 1 forbids — so
  it is inadmissible, not a working clause.)
- **requirement** (harness layer) — a named obligation is *filled*: a filler is
  `present` and `conforms-to` its typing, bound by the artifact's opt-in `satisfies`
  (see Requirements).
- **verified_by** (the delegation seam) — names an external verifier for the
  behavioral part. `temper` checks the verifier is *declared and wired*; it does
  **not** run it or judge the behavior (law 3, honest bound).

Every primitive is decidable: given the surface, each clause is unambiguously
true or false. A violation is therefore always a true positive — which is what
earns the hard gate.

## Requirements — the harness's named obligations

A **requirement** is a named obligation on the harness: *something must be present to
fill it.* It is the harness-layer (set-arity) concept — the interface/trait a concrete
artifact implements. **One** concept carries every facet. An earlier draft split it
into a structural `role` and a semantic `requirement` bridged by `filled_by`; that was
path-dependence, and the Decision below retires it. There is one requirement.

```toml
[requirement.dev-standards]
means    = "the harness maintains development standards"  # the why — human, never judged
required = true                                           # gate-blocking vs advisory
```

A requirement declares — all facets optional except its name:

- **`means`** — the authored *intent*, the why. `temper` **never interprets it** (no
  proxy; law 3); the surface carries and organizes it (`20-surface.md`).
- **fill** — a requirement and a member's `satisfies` are the **two ends of one edge**;
  the obligation is met when they join. Neither end is primary: the requirement is
  declared in the assembly, the `satisfies` on the member's own representation, and
  `check` **resolves the join** (an unfilled requirement and a dangling `satisfies` are
  the same diagnostic from opposite sides — Coverage, below). This is the *only* fill:
  there is deliberately **no contract-side name/glob `match`** — a name pattern is the
  contract *reaching out to guess* (a rename silently breaks it; intent inferred, not
  declared), the exact anti-pattern temper exists to eliminate.
- **typing** — `kind` and/or `package`: constrain *what* may fill it. The filler must
  be of the named `kind` and/or **conform to the named package** — never inline clauses
  (clauses live only in packages, below). A filler is thus checked against its own
  kind's bound package (conformance) *and* any package a requirement names — packages
  **compose**, as a type implements several traits. Both are *exact, declared*
  constraints — not a name-guess — which is why they stay where `match` was refused.
  Absent ⇒ **kind-blind**: any artifact that opts in fills it.
- **multiplicity** — a single filler (≥1, the default) or a predicate over the
  **satisfier set** — the artifacts opting into the requirement (`count` / `membership`
  / `unique`, `45-governance.md`). The governed set is defined by opt-in, kind-typed —
  never by a name selector.
- **`verified_by`** — wire an external judge for the behavioral remainder (below).

```toml
# opt-in fill, kind-blind — the intent path
# .temper/skills/dev-standards/  →  satisfies = ["dev-standards"]

# typed fill — kind-constrained opt-in (what a `role` was, minus the name guess):
[requirement.linter]
kind     = "rule"                    # only a rule may fill it
package  = "lint-standards"          # …that also conforms to this package (composes)
# a rule opts in from its representation:  satisfies = ["linter"]
required = true
```

### Coverage — the one referential check

`check` gates **coverage**: every `required` requirement resolves to its filler(s).
An **unfilled** requirement (nothing opts in, or the multiplicity is violated) and a
**dangling** `satisfies` (naming no declared requirement) are precise diagnostics.
This is the **referential primitive** (above) — decidable, a true positive every
time. It is the same referential resolution the graph runs over its
edges (`30-landscapes.md`), one arity down: a requirement and its fillers.

So a requirement reads as *intent* ("the harness must maintain dev standards") while
the gate stays *weak* ("does that obligation have a resolving filler?"). The meaning
is human; coverage is checked; `temper` does **not** judge whether the filler *truly*
fulfils `means` — that is the author's attestation (the `satisfies` opt-in),
optionally backed by a wired `verified_by`.

### Decision: role and requirement are one concept

**Chosen:** a single **requirement** — a named obligation with optional `means`,
optional typing (`kind` / `package`), fill by the artifact's opt-in `satisfies` (the
sole binding — no name-`match`, which would be the contract guessing), optional
multiplicity, and optional `verified_by`; `check` gates **coverage** (every required
requirement's filler resolves) — one referential, decidable check. **Rejected:** the earlier split into a structural `role` and a
semantic `requirement` bridged by `filled_by`. That split was path-dependence — `role`
shipped first (the harness slice); `requirement` was added *beside* it in the intent
reframe rather than *absorbing* it — and every rule it forced (`filled_by`, "one fill
path per requirement", "kind-typing lives on the role") was ceremony patching a seam
between two halves of one idea. Unifying **deletes** those rules — the tell that the
split was artificial. Typing is a facet (`kind` / `package`), not a rival concept;
there is no `filled_by`, because there are not two things to bridge.
**Rejected also:** temper assessing whether a filler *truly* fulfils `means` — that is
undecidable; the judged tier (`00-intent.md` tier 2) is advisory and delegated, and
behavioral truth goes to a wired `verified_by`. Meaningful obligation, weak gate.
(`requirement.` is its own namespace — distinct from the `rule` artifact kind.)

## Severity is declared, not baked

`temper` does not decide what is an error vs a warning. **The contract author
marks each clause `required` (gate-blocking) or `advisory` (reported, non-
blocking).** This replaces the tool-baked error/warn split: `--deny-advisories`
promotes advisories to blocking for a strict CI policy (the `-D warnings`
analogue). The default gate blocks on `required` clauses only.

The gate's **delivery posture** is declared the same way (`50-distribution.md`):
how firmly a failing contract is enforced at each placement — a hard block in CI,
an advisory notify-and-approve at session start — is the author's to set, not the
tool's to bake.

## Packages — the reusable, bindable unit of a contract

A **package** is the named unit that carries a kind's contract: a bundle of clauses,
bound to a kind by the assembly (`40-composition.md`). It is the reusable form of the
require-side — the std-lib type and the on-ramp so nobody writes a contract from
scratch. The documented best practices (Anthropic skill mechanics, Pocock's invocation
axis, the cascade harness-economy model) ship as **built-in packages**, adopted,
extended, forked, or ignored — never hardcoded checks.

A package is authored in the same medium as any member (`20-surface.md`): **one
document** — `PACKAGE.md` — whose structured header carries the clauses and whose
prose is the guidance. It carries **two semantically separate channels**, and the
split is load-bearing (`00-intent.md` law 2; `50-distribution.md`) — enforced by
the algebra, not the filesystem (Decision below):

- **clauses** — decidable predicates only, in the header; a package admits a
  clause *iff* it is decidable, so "name ≤ 64 chars, `[a-z0-9-]`" is in and
  "description triggers well" / "no-op detection" are **out** (undecidable).
  These gate. A clause may carry a `guidance` key — the hover-sized why,
  colocated with the clause it explains so it can never dangle from it.
- **guidance** — the best-practice prose the clauses cannot encode (house style,
  rationale): per-clause `guidance` text plus the document body. It never
  gates — not because of where it sits, but because the engine has **no path
  from prose to a predicate**: the closed vocabulary cannot say "check the
  guidance." Its primary consumer is the **authoring agent** (`00-intent.md`,
  positioning), and delivery is just-in-time: a failing clause's diagnostic
  carries its colocated `guidance` — the violation is the teaching moment — and
  the emitted schema serves the same text to humans as hover docs
  (`50-distribution.md`).

Emitted as a JSON Schema (`50-distribution.md`), a package delivers its clauses as
keystroke validation and its guidance as hover docs — best-practices-as-data reaching
the editor without either channel bleeding into the other. (JSON Schema itself
colocates `description` beside its constraints: colocated data, separated
semantics — the exact pattern.) Taste cannot become a squiggle because the
algebra has no syntax for it.

### Decision: guidance is colocated; the channel split is semantic

**Chosen:** clauses and guidance share one package document — per-clause
guidance as a key on the clause, package-level guidance as the document body —
and the guarantee "guidance never gates" is the **algebra's**: prose has no path
into a predicate. **Rejected:** physically separate files with guidance bound to
clauses by a heading convention. A heading binding is a *reference*, and
references dangle: rename a clause and its guidance silently orphans — the exact
silent-drift failure `temper` hunts, reintroduced into its own package format
and needing its own referential check to stay sound. Physical separation was a
physical proxy for a semantic invariant the algebra already enforces; colocation
deletes the reference instead of checking it.

### Decision: a built-in package is named for its source, and cited to it

**Chosen:** a built-in package's name is `<kind>.<source>` — the suffix is
**provenance of taste**, naming whose documented opinions the clauses encode:
`skill.anthropic`, and `rule` is **renamed `rule.anthropic`** (its clauses are
equally Anthropic-sourced — the `paths` scoping key, the lean-rule guidance;
the bare name was slice-order accident, not design). Bare names belong to
project-authored packages; the suffix scales to other sources and other
harnesses (`rule.cursor` would be Cursor's opinions about Cursor's format) and
keeps multiple packages per kind legible (`skill.anthropic` beside a project's
`skill.yourteam`). And the sourcing is **data, not a file comment**: a clause
may carry a `source` citation (URL + retrieved date) beside its `guidance`, and
for a *built-in* package's clauses it is the expected posture — their whole
legitimacy is *sourced* opinion. This is what makes "temper-maintained"
auditable: when the upstream docs move, the update ritual is *walk the clauses,
re-check their citations*, never *re-derive the package from memory*. It
composes with the fact/opinion marking (`15-kinds.md`): facts drift when the
harness's **behavior** changes, opinions when the **docs** change — different
cadences, both now traceable. The rename lands when the built-in packages are
authored on temper's own surface (the `contracts/` retirement, Decision below);
the embedded `rule` floor persists only until then. **Rejected:** bare kind
names for built-ins (hides whose taste it is, and collides with the
project-package namespace); source attribution as file-header prose (what
`contracts/*.toml` carries today — unauditable, and it drifts exactly like any
uncited claim).

### Decision: a package is project-authorable, not vendor-privileged — and is itself a kind

**Chosen:** a package is authored the same way regardless of origin; the only
difference is **who owns it** — a **built-in** package temper ships as a harness
adapter, or a **custom** package the author writes for their own project — and both
bind identically. A package is *itself* an artifact of the **`package` kind**, living
under `.temper/packages/<name>/` (`20-surface.md`) and checked by **the definition**
(admissibility, below): `package : the definition :: skill : its package`. **Rejected:**
packages as embedded, vendor-only files an author may merely fork. That is the exact
two-tier privilege `15-kinds.md` retired for *kinds* ("built-in vs custom is ownership,
not a privileged mechanism"), and it has no soundness basis here — a contract is
*opinion*, which law 2 makes the author's, adopted from data. Vendor-only packages
would make the author a tenant of temper's opinions rather than a composer of their
own. temper's built-in packages become just the *first-party instances* of a unit
every project authors and — via `bundle` (`50-distribution.md`) — publishes through
the same channel. **The `contracts/` embedded std-lib retires:** a built-in package's
authoritative home is `.temper/packages/<name>/` in temper's *own* repo (authored on
temper's own surface — the deepest dogfood), and the build **embeds** those authored
sources into the binary as the shipped std-lib. A consumer never carries a copy; the
assembly binds the built-in *by name* and it resolves from the embedded set (a version
pin will govern which format version — harness-version pinning is a held-back loose
end, `45-governance.md`). So the *same* package is
**authored** in temper's repo and **shipped** to a consumer — one artifact, two
provenance roles, no duplication, no `contracts/` mirror.

## `verified_by` — where behavior goes

The undecidable remainder ("a tool that does something useful") is expressed by
wiring a verifier, exactly as a Rust trait declares signatures while tests prove
behavior:

```toml
[requirement.release-tool]
kind        = "command"
package     = "release-command"    # a command that also conforms to this package
verified_by = "tests/release.rs"   # author checks this is wired; CI runs it
```

`temper` guarantees the requirement is filled and the judge is present and wired. The
judge (a test, a CI job, an eval) guarantees the behavior, at runtime. Neither
guesses; neither is `temper`.

"Wired" is a **referential** clause (above), not a string-present check: the named
verifier must *resolve* — the test target, CI job, or path exists in the surface —
or the `verified_by` fails admissibility. A dangling verifier is a silent no-op,
the very failure law 1 forbids. (Resolves the `verified_by` half of
`(harness-contract-provisioning)`.)

## Decision: kill the heuristic rule registry

**Chosen:** rules come from an author-declared contract validated by a generic
engine. **Rejected:** the slice-1 hardcoded `all_rules()` registry. It put the
tool's opinions in `if` statements, and its two non-decidable members produced
error-tier false positives on real skills (`companion-refs` matched prose
filenames; `trigger` ignored `disable-model-invocation`, flagging Pocock's own
user-invoked reference skill). The decidable members (name format, length,
forbidden keys, required fields) survive — as contract *clauses*, not code. The
slice-1 pipeline (import → IR → check → diagnostics → gate) stays; only the
source of rules changes.

## Decision: a bespoke closed algebra, not a general policy engine

**Chosen:** a small, hand-built, **closed** vocabulary of decidable primitives
(above), with diagnostics owned in-crate (`miette`), sitting on mature libraries
for the solved hard mechanics — `regex` for the charset matching behind
`allowed_chars` (the general `pattern` clause is held back — Decision below), a
graph crate for the dependency graph, `toml`/`serde` for parsing. **Rejected:** wrapping a general
policy/validation engine — OPA/Rego, CUE, or JSON Schema.

Those engines are mature and good, but they are *expressive enough to let an
author write an **unsound proxy*** — a deterministic rule that stands in for a
semantic judgment it cannot actually decide (e.g. `word_count(description) < 10`
as a "vagueness" check: it runs, it is deterministic, and it is wrong constantly).
That is precisely the heuristic escape hatch `00-intent.md` law 3 exists to close.
A deliberately weak, closed vocabulary makes the unsound proxy **unsayable by
construction** — there is no syntax for it. Secondary: generic engines emit poor
diagnostics, and diagnostics are the product (`00-intent.md`). The benefit of
building is not more power — it is *less power*: a language too weak to lie.
Adopt libraries for the solved mechanics; build the vocabulary, the diagnostics,
and the gate.

## Decision: `allowed_chars`, not a general `pattern` clause

**Chosen:** the exposed field-charset predicate is `allowed_chars` — a declared
character class (`ranges` + `chars`, e.g. `[a-z0-9-]`): decidable, and **too weak
to encode a proxy** — it can say *which characters*, never *which shape*.
**Rejected:** a general `pattern = "<regex>"` clause. An arbitrary regex is
decidable (it matches or it does not), so it would pass admissibility — but it is
expressive enough to be an **unsound proxy** (`pattern = "(when to use|use this
when)"` as a has-a-trigger check; a regex standing in for "third person"), the very
escape hatch the bespoke-algebra Decision closed against OPA/CUE/JSON-Schema,
walking back in through `regex`. `regex` stays sanctioned for *solved mechanics*,
but the author-facing vocabulary caps at `allowed_chars`. If a genuine **format**
need appears — a version or date field a charset cannot shape — add a **narrow
named predicate** for it, never a general regex clause; the vocabulary stays too
weak to lie. (Resolves `(regex-crate)`: regex was already sanctioned — the live
decision is to *not* expose an arbitrary-`pattern` clause.)

## Decision: a package is identified by its binding, not an internal name

**Chosen:** a package carries **no required internal `name`**. Its identity is *where
it lives* — its `.temper/packages/<name>/` home, which every binding names (`package =
"skill.anthropic"`, whether from a kind or a requirement). A display label for
diagnostics derives from the name/stem (`skill.anthropic`).
**Rejected:** a required top-level `name` field on every package. The examples above
(Requirements; Packages) identify packages by binding and carry no internal name — a
required name is redundant with the home and forces ceremony into a data file that is
otherwise clauses + guidance. (This resolves the `(contract-name-field)` fork: the
curated `skill.anthropic` package rightly has no `name`; the model's required-`name`
was code drift — relax it to optional, derived from the stem.)

## Decision: the contract is itself checked — admissibility

**Chosen:** a contract is an artifact like any other, validated against **the
definition** (the closed algebra + the structural rules below) by the same engine,
*before* it is used to check a harness. This is **admissibility**. The author-
declared contract earns trust the way the harness does — by passing a check — not
by the author's say-so. **Rejected:** trusting an author-declared contract on
faith. The built-in packages are first-party and curated, but the moment an
author writes or forks a package (`40-composition.md`), "the author declared it"
would become the heuristic escape hatch law 3 exists to close. Admissibility is
decidable, therefore sound:

- every clause names a predicate in the **closed vocabulary** (unknown ⇒ rejected);
- every referential clause **names its reference syntax** (the hole that made
  `companion-refs` unsound);
- every requirement's typed `kind`/`package` **names a real kind/package** —
  whether the requirement is *filled* is conformance's question (coverage), never
  admissibility's;
- every regex-backed clause **compiles** (none today — `pattern` is held, above);
  every `enum` is non-empty;
- every `verified_by` **resolves** to a declared verifier (above).

Admissibility never *detects* an unsound proxy — that would be the swamp again.
The closed algebra makes the unsound proxy **unsayable** (the "language too weak to
lie" Decision); admissibility only enforces that nothing outside the algebra
slipped in. It bottoms out at the hand-built algebra — the axiom, validated by
code, not by a further contract. No regress.

### Decision: unknown keys are rejected, not ignored

**Chosen:** every parsed table in `temper.toml` — `[requirement.*]`, `[kind.*]`, a
predicate clause — **rejects an unrecognized key** at parse rather than silently
dropping it. A misspelled `requird = true` must fail loudly, not degrade to
`required = false` and quietly disable the gate it was meant to arm. **Rejected:**
lenient parsing that ignores stray keys (the prior behavior across the requirement /
predicate parsers). A typo that weakens a contract is
exactly the failure temper exists to catch — so temper must not commit it in its own
parser; this is the anti-silent-gap non-negotiable (`collaboration` rule) applied to
the config surface itself. Unknown *contract* keys join the closed vocabulary as an
admissibility violation — the same posture as an unknown predicate (above), one rung
out to keys. This is distinct from an *artifact's* unknown frontmatter, which law 5
byte-preserves verbatim in `extra` (`20-surface.md`): artifact content is carried;
contract-surface keys are validated.

## The recursion bottoms out — the definition is not in `temper.toml`

The two checks are one relation at successive rungs — **contract over subject, and
the subject satisfies it:**

- a member artifact ⊨ its bound **package**'s contract — **conformance**;
- the **assembly** (`temper.toml`) and each **package** ⊨ **the definition** —
  **admissibility** (a package is a `.temper/` artifact, so it is checked here like any
  other, then trusted to check its members).

The definition — the closed algebra above plus the structural rules — is **engine-
owned and fixed; it is not in `temper.toml`, and must not be.** If the author could
add a primitive or loosen a structural rule from the surface, they could mint an
unsound proxy and law 3 collapses through that door — so the algebra is deliberately
**un-authorable**. That un-authorability *is* the immune system. So the recursion is
not infinite turtles: it is exactly two *checked* rungs grounded on a *fixed axiom*.
The relation is uniform (contract over subject at every rung), but the topmost
contract is engine-provided, never authored — and it bottoms out in **Rust, checked
by `rustc`**: the honest handoff off temper's own stack (`00-intent.md` honest bound).
`temper` governs harnesses and specs down to the algebra; it does not check its own
primitive vocabulary — that is the axiom it stands on.

## Decision: the `type` vocabulary is a closed scalar/container lattice

**Chosen:** the `type` primitive ranges over a fixed, closed set matching what
YAML frontmatter and JSON actually carry — `string`, `integer`, `number`,
`boolean`, `list`, `map`, `null` — taken from the source scalar's *parsed* type.
A sound `type` check therefore requires the extractor to **preserve the source
scalar type** in projection (`20-surface.md`); stringifying every scalar (the
slice-1 shortcut, `extract.rs`) makes `type` undecidable and is corrected before
the primitive ships. **Rejected:** a richer type language (formats, unions, nested
schemas, numeric ranges) — that drifts toward JSON-Schema, whose expressiveness is
exactly the unsound-proxy surface the "bespoke closed algebra" Decision rejects.
`min_len`/`max_len`/`enum`/`allowed_chars` already refine *within* a scalar type; `type`
only fixes the kind. (Resolves `(field-type-lattice)`.)
