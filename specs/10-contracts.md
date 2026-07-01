# Contracts — the two-layer model

A `Contract` is an author-declared, decidable description of structure that a
harness (or one artifact in it) must satisfy. `temper check` validates the
imported surface against the active contract and reports conformance. This is the
type checker; the contract is the types. (`00-intent.md` laws 2–3.)

`temper` runs **two checks**, not one. **Conformance** — the harness satisfies its
contract (the type checker above). **Admissibility** — the contract itself
satisfies *the definition*: the closed algebra below is both the vocabulary a
contract is written in *and* the contract a contract must satisfy. A type system
checks values against types *and* checks the types are well-formed; `temper`
checks harness-against-contract *and* contract-against-definition (`00-intent.md`
finish line — both greens). Admissibility is what lets an author *declare* a
contract (`40-composition.md`) without re-opening the heuristic swamp by the front
door (Decision below).

## The engine is generic; everything is an instance

There are not "two kinds of contract." There is one engine over the primitive
algebra (below), and every contract is an **instance** expressed in it. The
distinctions are compositional, not built-in:

| Instance | Declares | `temper` checks | Analogy |
| -------- | -------- | --------------- | ------- |
| **Artifact contract** | the shape of one artifact kind | each artifact conforms | a **type** |
| **Harness contract** | required requirements + relations + verifiers across a harness | each requirement is filled and wired | an **interface / trait** |
| **Spec contract** | the declared domain model + how prose binds to it (`30-landscapes.md`) | the model is coherent; prose binds; the graph resolves | a **schema / ontology** |

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
  character class, e.g. `[a-z0-9-]`); `max_len` / `min_len`; `enum`; `deny`
  (forbidden values); `forbidden_keys` (e.g. the Cursor `globs`/`alwaysApply` keys
  Claude Code ignores). The full `pattern` (arbitrary regex) clause is **held
  back** — see the `allowed_chars` Decision below.
- **structural** — `max_lines`; `require_sections` (named headings present);
  `must_define` (a field/marker exists, e.g. `disable-model-invocation`).
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
  `present`, `conforms-to` its typing, selected by opt-in `satisfies` or `match`
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
- **fill** — how the obligation is met. *Preferred:* an artifact **opts in** from its
  own representation with a `satisfies` link — the artifact declaring what it fills,
  not the contract reaching out to guess. *Alternative:* a contract-side `match`
  selector (`name` / glob) for artifacts that haven't opted in — the on-ramp for a
  freshly imported harness.
- **typing** — `kind` and/or `contract`: constrain *what* may fill it. Absent ⇒
  **kind-blind**: any artifact that opts in fills it.
- **multiplicity** — a single filler (≥1, the default) or a predicate over the filler
  *set* (`count` / `membership` / `unique`, `45-governance.md`).
- **`verified_by`** — wire an external judge for the behavioral remainder (below).

```toml
# opt-in fill, kind-blind — the intent path
# .temper/skills/dev-standards/  →  satisfies = ["dev-standards"]

# typed fill — the structural path (what a `role` was):
[requirement.linter]
kind     = "rule"                    # only a rule may fill it
match    = { name = "lint*" }        # contract-side selector, or a rule opts in via satisfies
required = true
```

### Coverage — the one referential check

`check` gates **coverage**: every `required` requirement resolves to its filler(s).
An **unfilled** requirement (nothing opts in or matches, or the multiplicity is
violated) and a **dangling** `satisfies` (naming no declared requirement) are precise
diagnostics. This is the **referential primitive** (above) — decidable, a true
positive every time. It is the same referential resolution the graph runs over its
edges (`30-landscapes.md`), one arity down: a requirement and its fillers.

So a requirement reads as *intent* ("the harness must maintain dev standards") while
the gate stays *weak* ("does that obligation have a resolving filler?"). The meaning
is human; coverage is checked; `temper` does **not** judge whether the filler *truly*
fulfils `means` — that is the author's attestation (the `satisfies` / `match`),
optionally backed by a wired `verified_by`.

### Decision: role and requirement are one concept

**Chosen:** a single **requirement** — a named obligation with optional `means`,
optional typing (`kind` / `contract`), a fill declaration (opt-in `satisfies`,
preferred, or `match`), optional multiplicity, and optional `verified_by`; `check`
gates **coverage** (every required requirement's filler resolves) — one referential,
decidable check. **Rejected:** the earlier split into a structural `role` and a
semantic `requirement` bridged by `filled_by`. That split was path-dependence — `role`
shipped first (the harness slice); `requirement` was added *beside* it in the intent
reframe rather than *absorbing* it — and every rule it forced (`filled_by`, "one fill
path per requirement", "kind-typing lives on the role") was ceremony patching a seam
between two halves of one idea. Unifying **deletes** those rules — the tell that the
split was artificial. Kind-typing is a facet (`kind` / `contract`), not a rival
concept; there is no `filled_by`, because there are not two things to bridge.
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

## Templates — best practices as data

The documented best practices (Anthropic skill mechanics, Pocock's invocation
axis, the cascade harness-economy model) ship as **contract templates** under
`contracts/` — declarations an author adopts, extends, forks, or ignores. They
are the std-lib types and the on-ramp so nobody writes a contract from scratch;
they are never hardcoded checks. A template admits a clause only if it is
decidable — so "name ≤ 64 chars, `[a-z0-9-]`" is in; "description triggers well"
and "no-op detection" are **out** (undecidable), and stay as prose guidance, not
checks. Emitted as a JSON Schema (`50-distribution.md`), a template delivers its
decidable clauses as keystroke validation and that prose guidance as hover docs —
best-practices-as-data reaching the editor without ever becoming a check.

## `verified_by` — where behavior goes

The undecidable remainder ("a tool that does something useful") is expressed by
wiring a verifier, exactly as a Rust trait declares signatures while tests prove
behavior:

```toml
[requirement.release-tool]
kind        = "command"
contract    = { required = ["description"], must_define = ["executable"] }
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
for the solved hard mechanics — `regex` for `pattern`, a graph crate for the
dependency graph, `toml`/`serde` for parsing. **Rejected:** wrapping a general
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

## Decision: a contract is identified by its path/binding, not an internal name

**Chosen:** a `Contract` carries **no required internal `name`**. Its identity is
*where it lives* — the file path a requirement binds (`contract = "contracts/skill.anthropic.toml"`)
or the inline block under a requirement. A display label for diagnostics derives from
the file stem (`skill.anthropic`). **Rejected:** a required top-level `name` field on
every contract. The contract examples above (Requirements; Templates)
identify contracts by path or inline binding and carry no internal name — a
required name is redundant with the path and forces ceremony into a data file
that is otherwise pure clauses. (This resolves the `(contract-name-field)` fork:
the curated `contracts/skill.anthropic.toml` rightly has no `name`; the model's
required-`name` was code drift — relax it to optional, derived from the stem.)

## Decision: the contract is itself checked — admissibility

**Chosen:** a contract is an artifact like any other, validated against **the
definition** (the closed algebra + the structural rules below) by the same engine,
*before* it is used to check a harness. This is **admissibility**. The author-
declared contract earns trust the way the harness does — by passing a check — not
by the author's say-so. **Rejected:** trusting an author-declared contract on
faith. The built-in templates are first-party and curated, but the moment an
author writes or forks a contract (`40-composition.md`), "the author declared it"
would become the heuristic escape hatch law 3 exists to close. Admissibility is
decidable, therefore sound:

- every clause names a predicate in the **closed vocabulary** (unknown ⇒ rejected);
- every referential clause **names its reference syntax** (the hole that made
  `companion-refs` unsound);
- every requirement's fill **resolves** — a `match` selector resolves, a typed
  `kind`/`contract` names a real kind/contract — and a `required` single-filler
  requirement is satisfiable;
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

- `.temper/` contents ⊨ `temper.toml` — **conformance**;
- `temper.toml` ⊨ **the definition** — **admissibility**.

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
