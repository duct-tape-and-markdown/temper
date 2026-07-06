# Contracts — clauses, requirements, and the two walls

A **contract** is an author-declared, decidable description of structure that a
harness must satisfy. It is not a document and not a file format: it is **clause
data attached in the assembly** by two quantifiers — `expect` (universal: every
member of a kind owes these clauses) and `require` (existential: the harness
must contain a fill). `temper check` validates the committed harness against
that data and reports conformance. This is the type checker; the contract is
the types (`00-intent.md` laws 2–3; `05-model.md`, two algebras, one type).

`temper` runs **two checks**, not one. **Conformance** — the harness satisfies
the clauses attached to it (the type checker above). **Admissibility** — the
declarations themselves are well-formed: one stage, run before any conformance
pass (below). A type system checks values against types *and* checks the types
are well-formed; `temper` checks harness-against-clauses *and*
declarations-against-the-definition (`00-intent.md` finish line — both greens).

## The engine is generic; everything is an instance

The engine is **kind-blind and schema-blind** (`15-kinds.md`): it extracts
generic features, assembles one graph, and judges compiled predicates over
(features, graph) at three scopes — node, node-set, edge. It knows no domain
names. An artifact shape is an `expect` over a kind; a harness obligation is
a `require`; a spec corpus is just more kinds (`30-landscapes.md`). A new
landscape is new instance data, never new engine code — there is nowhere to
hardcode an opinion (`00-intent.md`: one engine, every layer an instance).

## The clause — the atom of a contract

```
Clause = predicate · severity · guidance · cite
```

A clause is an SDK (TypeScript) value that **erases to compiled data** at the
seam (`20-surface.md`): the author composes clauses as typed objects; the
engine consumes their compiled rows. There is no hand-authored clause grammar —
no clause document, no TOML clause table; the SDK's types are the only
authoring spelling, and the compiled row is the only thing checked.

- **predicate** — a member of the closed algebra (below). Nothing else is
  writable: the vocabulary is spelled as the SDK's types, so a clause outside
  it is a squiggle, not a runtime rejection.
- **severity** — `required` (gate-blocking) or `advisory` (reported,
  non-blocking). **The author marks each clause; `temper` does not decide what
  is an error.** `--deny-advisories` promotes advisories to blocking for a
  strict CI policy (the `-D warnings` analogue); the default gate blocks on
  `required` clauses only. The gate's **delivery posture** at each placement —
  hard block in CI, notify-and-approve at session start — is likewise the
  author's to declare (`50-distribution.md`), never the tool's to bake.
- **guidance** — the just-in-time teaching channel: the best-practice prose
  the predicate cannot encode (house style, rationale). It never gates — not
  by placement but by construction: the engine has **no path from prose to a
  predicate**. Its primary consumer is the **authoring agent** (`00-intent.md`,
  positioning), and delivery is at the moment of failure: a failing clause's
  diagnostic carries its guidance — the violation is the teaching moment — and
  the same text serves humans as hover docs (`50-distribution.md`). Guidance
  is a field *on* the clause value, so it cannot dangle from the clause it
  explains.
- **cite** — the external-fact source: a doc URL plus retrieved date, as
  **data, not a comment**. For the shipped floors it is the expected posture —
  their whole legitimacy is *sourced* opinion — and it is what makes a
  maintained floor auditable: when upstream docs move, the update ritual is
  *walk the clauses, re-check their citations*, never *re-derive from memory*.
  Facts drift when the harness's behavior changes, opinions when the docs
  change — different cadences, both traceable (`15-kinds.md`).

Every clause is decidable: given the surface, it is unambiguously true or
false. A violation is therefore always a true positive — which is what earns
the hard gate.

## The predicate algebra — closed, decidable, every member decides

The predicate vocabulary is **fixed and closed**. There is no arbitrary-code
clause; adding a predicate is a deliberate language change, never a
per-contract escape hatch (law 3), and a predicate is admitted only when its
consumer lands (the entry gate). **Every member decides** — a predicate that
can only answer *indeterminate* is not a weak member, it is inadmissible: an
indeterminate verdict is a silent no-op, and law 1 forbids silent passes.

Judged at the **node** scope (one member's features):

- **required** — a field or marker is present. Optionality is *not* a
  predicate: an optional field asserts nothing, so it has no clause — the
  SDK's interfaces carry optionality at the keystroke, and the compiler emits
  `required` predicates exactly where absence is a violation.
- **type** — the field's parsed scalar type (Decision below).
- **allowed_chars** — a declared character class, e.g. `[a-z0-9-]` (Decision
  below — no general `pattern`).
- **min_len / max_len**; **range** `{min, max}` over `integer`/`number` (a
  numeric bound is a fact); **enum**; **deny** (forbidden values);
  **forbidden_keys** (e.g. the Cursor `globs`/`alwaysApply` keys Claude Code
  ignores).
- **max_lines**; **require_sections** (named headings present);
  **section_contains** `{heading, marker}` (every section whose heading starts
  with the declared text carries the declared marker — the predicate behind
  the spec kind's decisions-name-alternatives); **name-matches-dir**.
- **reference-resolves** — a reference resolves, *only* over a declared
  structured field or a reference syntax the external format itself executes
  (a `CLAUDE.md` `@path` import) — never by grepping prose (law 8). If the
  author cannot name the reference syntax, the clause is inadmissible: without
  one the engine can only return indeterminate. (Exactly what made the slice-1
  `companion-refs` rule unsound — Decision below — and why no general
  "a dependency exists" predicate exists: it is a named, resolving reference
  or it is nothing.)

Judged at the **node-set** scope (the satisfier set of a requirement, or a
kind's population — quantification, not fuzziness):

- **count** — `|satisfiers| ∈ [min, max]` ("at most N planners," "exactly one
  release-tool"). A whole-kind population constraint quantifies over the kind;
  an intent subset quantifies over a requirement's opt-in satisfiers.
- **unique** — a field is unique across the set (names within a kind are the
  standing instance).
- **membership** — field F of every satisfier of R₁ is drawn from a feature
  over the satisfiers of R₂ ("every agent's `model` is one of the approved
  set"). Where the target set must itself be shaped, membership carries
  **inline clauses** qualifying it — the same clause values, attached in
  place, spreadable from a floor like any other.

Judged at the **edge** scope (the one relation graph — `45-governance.md` is
the detailed home):

- **degree** — in/out edge counts on a node. Two idioms are documented and
  supported: **self-registering** (`degree(incoming) = 0` — engages the world
  through its own registration and must not be pointed at) and **routed**
  (`degree(incoming) ≥ 1` — reached by being pointed at). General numeric
  bands wait at the entry gate for a consumer, like every extension.
- **acyclic** — the reference graph has no cycle (a circular import loads
  nothing).
- **reachable** — the member is in the world's reachability closure
  (registration and the world node, `45-governance.md`).

Held back, deliberately: **pattern** (Decision below) and **conditionals**
(`if field=X then require Y` — decidable, but where proxies re-enter;
`45-governance.md`). The fence is the entry gate: admit only when a concrete
sound need appears.

## Two quantifiers — `expect` and `require`

All clause attachment happens in the assembly through two quantifiers:

- **`expect(kind, clauses)`** — universal. Every member of the kind owes every
  clause in the array. This is the artifact-shape contract: what a skill *is*.
- **`require(name, …)`** — existential. The harness must contain a fill for a
  named obligation (Requirements, below). This is the harness contract: what
  the harness *contains*.

A shared clause set — a **floor** — is nothing but an **exported clause
array**. Adoption is an import; extension is a spread; overriding is array
surgery in the language the author already writes:

```ts
import { skill, skillFloor } from "@dtmd/temper/claude-code";

expect(skill, [
  ...skillFloor,                 // adoption is an import
  clause(maxLines(300), {
    severity: "advisory",
    guidance: "House style: past ~300 lines a skill body wants a reference file.",
  }),
]);
```

(Spelling illustrative; the SDK's types are normative.)

The documented best practices — Anthropic skill mechanics, Pocock's invocation
axis, the cascade harness-economy model — ship as floors exported from
`@dtmd/temper/claude-code`: first-party instances of the same type every project
authors, each clause cited to its source (the `cite` field, above). This is
law 2 with the machinery removed, and each guarantee got **stronger**:

- **adopted by choice** — adoption is a literal import statement the author
  writes; nothing binds by default.
- **overridable** — a spread plus a filter is the whole override mechanism; no
  layering rules, no precedence table, no shadowing semantics to learn.
- **project-authorable as peers** — a project's clause array is the *same
  type* as the shipped one; there is no vendor-privileged form to be a tenant
  of, and publishing your floor is exporting it (`50-distribution.md`).

Identity travels **by import, never by string** (`15-kinds.md`): `kind: skill`
references a value; two providers are two modules; collision is impossible and
there is no name-qualification scheme to administer.

### Decision: clause arrays and two quantifiers, not a package noun

**Chosen:** the reusable contract unit is an exported clause array, attached
by `expect`/`require` in the assembly; sharing, extension, and override are
the host language's own import and spread. **Rejected:** (a) a **package
noun** with layering machinery — a named carrier artifact with its own kind,
identity rules (binding-derived names, a `<kind>.<source>` naming scheme), a
filesystem home, and its own admissibility story; every piece of that
machinery existed to make a data file behave like a module, and a module does
it natively. (b) **Hand-authored grammar files** — a `PACKAGE.md` or TOML
clause table as the authoring surface: a permanent second spelling of the
algebra needing its own parser, format-preserving patching, and keystroke
channel, serving no author the SDK does not already serve (`00-intent.md`,
the SDK-is-the-surface Decision). (c) **Vendor-privileged floors** — built-ins
an author may merely fork; a contract is *opinion*, which law 2 makes the
author's, adopted from data. (d) **Guidance bound to clauses by heading
convention in separate files** — a heading binding is a reference, and
references dangle: rename a clause and its guidance silently orphans, the
exact silent-drift failure `temper` hunts; a field on the clause value
deletes the reference instead of checking it.

The engine-side residue of the package era retires with the noun, named so no
derived layer must guess (`90-spec-system.md`, Decisions): the `package` facet
on requirements and the resolver behind it (a requirement constrains by
`kind`; there is no second shape channel — Requirements, below), the
conformance pass validating satisfiers against a bound package's contract,
`membership` constrained by package conformance (`conforms_to` — membership's
target is a requirement's satisfier set, above), diagnostics that teach the
package vocabulary (`50-distribution.md`, no synonyms), and the curated
`PACKAGE.md`/`KIND.md` trees with their embeds (`15-kinds.md`, no kind file
format; the plugin delivers the gate, not clauses — `50-distribution.md`).
Their citation trail is not residue: each floor clause's source moves to the
`cite` field that already homes it, never dropped.

## Requirements — the harness's named obligations

A **requirement** is a named obligation on the harness: *something must be
present to fill it.* It is the existential quantifier's payload — the
interface/trait a concrete member implements. One concept carries every facet:

```
Requirement = means · kind · required · count? · unique? · membership? · degree? · verifiedBy?
```

```ts
require("dev-standards", {
  means: "the harness maintains development standards", // human — never judged
  kind: rule,        // identity by import — a value, not a string
  required: true,    // posture: absence blocks the gate
});
// a member opts in from its own declaration:  satisfies: ["dev-standards"]
```

- **`means`** — the authored *intent*, the why. **Carried, never
  interpreted** (no proxy; law 3): the surface organizes it, diagnostics
  quote it, no check reads it.
- **`kind`** — constrains *what* may fill the obligation, **by import**. A
  filler's shape obligations are its kind's `expect` clauses — the requirement
  never carries a second shape channel; it constrains which kind and how the
  satisfier set measures, not what a member of that kind is. Absent ⇒
  **kind-blind**: any member that opts in fills it.
- **`required`** — the **posture declaration**: what absence *means* —
  gate-blocking or advisory. Laws 1 and 4 in one field: the author declares
  whether an unfilled obligation blocks, and `temper` never invents an
  obligation the author didn't post. It is never cardinality (Decision below).
- **`count` / `unique` / `membership` / `degree`** — predicates over the
  **satisfier set** and its graph neighborhood (the node-set and edge scopes,
  above). Absent `count`, the fill is plainly existential: at least one
  satisfier. The governed set is defined by opt-in, kind-typed — never by a
  name selector.
- **`verifiedBy`** — wired delegation for the behavioral remainder: `temper`
  guarantees the obligation is filled and the judge is **present and wired**;
  the judge (a test, a CI job, an eval) guarantees the behavior, at runtime.
  Neither guesses; neither is `temper`. "Wired" is **referential**, not
  string-present: the named verifier must *resolve* — the test target, CI
  job, or path exists in the surface — or the declaration fails
  admissibility. A dangling verifier is a silent no-op, the very failure
  law 1 forbids. (Resolves the `verifiedBy` half of
  `(harness-contract-provisioning)`.)

**The fill is a join.** A requirement and a member's `satisfies` are the two
ends of one edge; the obligation is met when they join, and `check` resolves
the join (`45-governance.md`). An **unfilled** requirement (no satisfier, or
a set predicate violated) and a **dangling** `satisfies` (naming no declared
requirement) are the same diagnostic from opposite sides — precise,
decidable, a true positive every time. There is deliberately **no
contract-side name/glob match**: a name pattern is the contract *reaching out
to guess* (a rename silently breaks it) — the exact anti-pattern `temper`
exists to eliminate. So a requirement reads as intent while the gate stays
weak ("does that obligation have a resolving fill?"): the meaning is human,
the join is checked, and `temper` never judges whether the filler *truly*
fulfils `means` — that is the author's attestation, optionally backed by a
wired `verifiedBy`. The undecidable remainder ("a tool that does something
useful") is where `verifiedBy` earns its slot — a trait declares signatures
while tests prove behavior:

```ts
require("release-tool", {
  means: "a command that cuts a release",
  kind: command,
  required: true,
  count: { max: 1 },
  verifiedBy: "tests/release.test.ts", // temper checks it is wired; CI runs it
});
```

### Decision: `required` is posture, never cardinality

**Chosen:** `required` declares what absence means; `count` measures the
satisfier set. They are different kinds of thing — a policy declaration and a
measurement — and are never merged. **Rejected:** collapsing `required` into
cardinality (`required` ≡ `count ≥ 1`). The merge conflates the two axes at
exactly their useful joints: an *advisory* obligation with an exact count
("we'd like exactly one planner, warn otherwise") and a *blocking* obligation
with no counting at all are both real postures, and neither is sayable when
one field serves two masters. Posture is law-1/law-4 territory (who declared
that absence blocks); cardinality is a set predicate like any other.

### Decision: a requirement's publisher is any authored surface

**Chosen:** a requirement may be published by the **assembly** (a
project-level obligation: "this harness carries a rule for X") or by a
**member's own `requires`** (a concept-level obligation: "the entity this
member declares must be satisfied by an architecture member"). One namespace,
one fill mechanism, one gate: `satisfies` is the same opt-in claim whoever
published the demand, and a name collision across publishers is an
admissibility finding, never a shadowing rule. Inter-member structure is
authored **intentionally on each side** — the demand at the concept's home,
the fill at the satisfier — with no third mechanism to learn. **Rejected:** a
separate member-to-member pairing vocabulary beside requirements — two names
for one edge would fork the model `explain` teaches.

### Decision: satisfaction is total per named demand

**Chosen:** a `satisfies` claim is whole — a member fills a named requirement
or it does not. A demand only partly met is a **composite demand**: decompose
it into named sub-requirements and claim exactly the parts filled; the gate
then reports *which* named part is missing — decidable, actionable — never a
weight. **Rejected:** an authored partial claim (`partial: true`, a
percentage) — "how much of this obligation does my prose fill" is an
undecidable judgment (law 3), and a partial claim either counts toward the
fill (masking the unfilled remainder) or does not (decoration). Derived
partiality stays legitimate: `check` may *report* "3 of 5 demands filled" —
computed output, never an authored claim.

### Decision: role and requirement are one concept

**Chosen:** a single **requirement** carrying every facet above. **Rejected:**
an earlier split into a structural `role` and a semantic `requirement`
bridged by `filled_by`. That split was path-dependence — `role` shipped
first; `requirement` was added beside it rather than absorbing it — and every
rule it forced ("one fill path per requirement", "kind-typing lives on the
role") was ceremony patching a seam between two halves of one idea. Unifying
**deletes** those rules — the tell that the split was artificial.

## Admissibility — the contract is itself checked, one stage

Declarations earn trust the way the harness does — by passing a check, not by
the author's say-so. Admissibility is **one stage, run before any conformance
pass**; only when the declarations stand does the judge run its three scopes.

Most of it lives at the **keystroke**: clauses, kinds, and requirements are
typed SDK values, so a malformed declaration is a **squiggle** — an unknown
predicate is unwritable, a misspelled facet is a type error, a `kind`
reference is an import that either resolves or does not compile. tsc is doing
what a bespoke declaration-checker would otherwise do, earlier and in the
author's editor.

The engine does not re-trust the seam. The compiled data it consumes is
validated for shape before use:

- every clause row names a predicate in the **closed vocabulary** — unknown ⇒
  rejected loudly, never skipped;
- unknown keys in a compiled row are **rejected, never dropped** — a stray key
  that would silently disarm a gate is the exact failure `temper` exists to
  catch, so it must not commit it at its own seam (artifact frontmatter is
  different: law 5 byte-preserves an *artifact's* unknown keys verbatim;
  contract data is validated);
- every `enum` is non-empty; every `membership` names declared requirements at
  both ends; every referential clause names its reference syntax;
- every `verifiedBy` resolves to a declared verifier;
- every `kind` reference resolves to a declared kind row in the lock —
  whether an obligation is *filled* is conformance's question, never
  admissibility's.

**Chosen:** declarations validated before use, at both walls. **Rejected:**
trusting an author-declared contract on faith — the moment an author writes
or forks a floor, "the author declared it" would become the heuristic escape
hatch law 3 exists to close. Admissibility never *detects* an unsound proxy —
that would be the swamp again. The closed algebra makes the unsound proxy
**unsayable** (Decision below); admissibility only enforces that nothing
outside the algebra slipped in.

## The two walls

The closed vocabulary reaches the author twice — one algebra, two walls
(`20-surface.md`):

- **The keystroke wall — tsc.** The algebra is spelled as the SDK's types, so
  a clause outside the vocabulary cannot be typed; **field typing lives only
  in the SDK's plain interfaces** (the engine is schema-blind — a kind's
  runtime residue is five facts, `15-kinds.md`); guidance rides the hover.
  This wall is a courtesy: fast, local, editor-shaped.
- **The conformance wall — the gate.** The engine consumes committed
  artifacts and the lock, offline, with no language runtime; it validates
  compiled shape (admissibility) and judges conformance. This wall is the
  authority: every consumer meets it regardless of editor or authoring
  posture.

Erasure would strand the author's vocabulary on the wrong side of the seam —
so findings carry **compiled debug labels**, stamped at compile from the
declaration site (module, export, clause name): the gate names the failing
clause as the author spelled it and quotes its guidance, after every type has
erased. The walls cannot drift apart because the SDK **pins its engine**
(`20-surface.md`) — the vocabulary versions in lockstep on both sides of the
seam.

## The recursion bottoms out — the definition is not authorable

The two checks are one relation at successive rungs — contract over subject,
and the subject satisfies it:

- a member ⊨ the clauses attached to it — **conformance**;
- the declarations ⊨ **the definition** — **admissibility**.

The definition — the closed algebra plus the shape rules above — is
**engine-owned and fixed**. It is not on the authoring surface and must not
be: if the author could mint a predicate or loosen a shape rule, they could
mint an unsound proxy and law 3 collapses through that door. The SDK's host
language composes clauses — loops, spreads, conditionals at authoring time —
but **cannot define a predicate**: Turing-completeness is quarantined at
authoring; what crosses the seam is inert data (`00-intent.md`). That
un-authorability *is* the immune system. So the recursion is not infinite
turtles: two checked rungs grounded on a fixed axiom, which bottoms out in
the engine's own implementation, checked by its compiler and test suite —
the honest handoff off `temper`'s stack (`00-intent.md` honest bound; the
implementation language is deliberately non-normative — `(engine-language)`
resolved: Rust today, stated nowhere as contract).

## Decision: kill the heuristic rule registry

**Chosen:** checks come from author-declared clause data judged by a generic
engine. **Rejected:** the slice-1 hardcoded `all_rules()` registry. It put
the tool's opinions in `if` statements, and its two non-decidable members
produced error-tier false positives on real skills (`companion-refs` matched
prose filenames; `trigger` ignored `disable-model-invocation`, flagging
Pocock's own user-invoked reference skill). The decidable members (name
format, length, forbidden keys, required fields) survive — as clauses, not
code.

## Decision: a bespoke closed algebra, not a general policy engine

**Chosen:** a small, hand-built, **closed** vocabulary of decidable
predicates, with diagnostics owned by the engine, sitting on mature libraries
for the solved hard mechanics (regular-expression matching behind
`allowed_chars`, graph algorithms, parsers). **Rejected:** wrapping a general
policy/validation engine — OPA/Rego, CUE, or JSON Schema. Those engines are
mature and good, but they are *expressive enough to let an author write an
**unsound proxy*** — a deterministic rule standing in for a semantic judgment
it cannot decide (`word_count(description) < 10` as a "vagueness" check: it
runs, it is deterministic, and it is wrong constantly). That is precisely the
escape hatch law 3 closes. A deliberately weak, closed vocabulary makes the
unsound proxy **unsayable by construction** — there is no syntax for it — and
the SDK's host language does not reopen the hole: it decides *which* clauses
to attach, never what a clause can say. Secondary: generic engines emit poor
diagnostics, and diagnostics are the product. The benefit of building is not
more power — it is *less* power: a language too weak to lie.

## Decision: `allowed_chars`, not a general `pattern` clause

**Chosen:** the exposed field-charset predicate is `allowed_chars` — a
declared character class (`ranges` + `chars`, e.g. `[a-z0-9-]`): decidable,
and **too weak to encode a proxy** — it can say *which characters*, never
*which shape*. **Rejected:** a general `pattern = <regex>` clause. An
arbitrary regex is decidable (it matches or it does not), so it would pass
admissibility — but it is expressive enough to be an unsound proxy
(`(when to use|use this when)` as a has-a-trigger check; a regex standing in
for "third person"), the very hatch the bespoke-algebra Decision closed,
walking back in through the regex engine. Regular-expression matching stays a
sanctioned *mechanic inside the engine*; the author-facing vocabulary caps at
`allowed_chars`. If a genuine **format** need appears — a version or date
field a charset cannot shape — add a **narrow named predicate** for it, never
a general regex clause; the vocabulary stays too weak to lie. (Resolves
`(regex-crate)`: the live decision is to *not* expose an arbitrary-`pattern`
clause.)

## Decision: the `type` vocabulary is a closed scalar/container lattice

**Chosen:** the `type` predicate ranges over a fixed, closed set matching what
YAML frontmatter and JSON actually carry — `string`, `integer`, `number`,
`boolean`, `list`, `map`, `null` — taken from the source scalar's *parsed*
type. A sound `type` check requires extraction to **preserve the source
scalar's parsed type** (`20-surface.md`); the same lattice is what the SDK's
interfaces spell at the keystroke, so the two walls agree on what a field can
be. **Rejected:** a richer type language (formats, unions, nested schemas) —
that drifts toward JSON Schema, whose expressiveness is exactly the
unsound-proxy surface the bespoke-algebra Decision rejects.
`min_len`/`max_len`/`enum`/`allowed_chars`/`range` already refine *within* a
scalar type; `type` only fixes the kind. (Resolves `(field-type-lattice)`.)
