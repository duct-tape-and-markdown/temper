# Contracts — the two-layer model

A `Contract` is an author-declared, decidable description of structure that a
harness (or one artifact in it) must satisfy. `temper check` validates the
imported surface against the active contract and reports conformance. This is the
type checker; the contract is the types. (`00-intent.md` laws 2–3.)

## The engine is generic; everything is an instance

There are not "two kinds of contract." There is one engine over the primitive
algebra (below), and every contract is an **instance** expressed in it. The
distinctions are compositional, not built-in:

| Instance | Declares | `temper` checks | Analogy |
| -------- | -------- | --------------- | ------- |
| **Artifact contract** | the shape of one artifact kind | each artifact conforms | a **type** |
| **Harness contract** | required roles + relations + verifiers across a harness | the roster is filled and wired | an **interface / trait** |
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

- **field** — `required` / `optional`; `type`; `pattern` (regex); `max_len` /
  `min_len`; `enum`; `deny` (forbidden values); `forbidden_keys` (e.g. the
  Cursor `globs`/`alwaysApply` keys Claude Code ignores).
- **structural** — `max_lines`; `require_sections` (named headings present);
  `must_define` (a field/marker exists, e.g. `disable-model-invocation`).
- **referential** — a reference resolves — *only over a precisely declared
  reference syntax* (e.g. markdown links, an explicit `@path`), never by grepping
  prose. If the author can't name the reference syntax, the clause is not
  admissible (this is exactly what made the slice-1 `companion-refs` rule unsound;
  see Decision below).
- **cross-artifact** — names unique within a kind; a declared dependency exists;
  `name-matches-dir`.
- **role** (harness layer) — a component filling role R is `present`,
  `conforms-to` contract C, and is selected by `match` (see Roles).
- **verified_by** (the delegation seam) — names an external verifier for the
  behavioral part. `temper` checks the verifier is *declared and wired*; it does
  **not** run it or judge the behavior (law 3, honest bound).

Every primitive is decidable: given the surface, each clause is unambiguously
true or false. A violation is therefore always a true positive — which is what
earns the hard gate.

## Roles and matching

A harness contract binds an abstract **role** to whichever concrete artifact
fills it. The open question is *which* artifact: matching is itself a decidable
selector, never a guess.

```toml
[role.task-planning]
artifact = "skill"
contract = "contracts/skill.anthropic.toml"   # adopt a template, or inline clauses
match    = { name = "plan*" }                  # decidable selector picks the filler
required = true                                 # absent filler ⇒ contract violation
```

Matching options (all decidable): by `name`/glob, by an explicit `role:` marker
the artifact declares, or — preferred for clarity — by the artifact *opting in*
(declaring the role it fills) rather than the contract reaching out to guess.
When zero or many artifacts match a `required` single-filler role, that is a
conformance error, reported precisely.

## Severity is declared, not baked

`temper` does not decide what is an error vs a warning. **The contract author
marks each clause `required` (gate-blocking) or `advisory` (reported, non-
blocking).** This replaces the tool-baked error/warn split: `--deny-advisories`
promotes advisories to blocking for a strict CI policy (the `-D warnings`
analogue). The default gate blocks on `required` clauses only.

## Templates — best practices as data

The documented best practices (Anthropic skill mechanics, Pocock's invocation
axis, the cascade harness-economy model) ship as **contract templates** under
`contracts/` — declarations an author adopts, extends, forks, or ignores. They
are the std-lib types and the on-ramp so nobody writes a contract from scratch;
they are never hardcoded checks. A template admits a clause only if it is
decidable — so "name ≤ 64 chars, `[a-z0-9-]`" is in; "description triggers well"
and "no-op detection" are **out** (undecidable), and stay as prose guidance, not
checks.

## `verified_by` — where behavior goes

The undecidable remainder ("a tool that does something useful") is expressed by
wiring a verifier, exactly as a Rust trait declares signatures while tests prove
behavior:

```toml
[role.release-tool]
artifact    = "command"
contract    = { required = ["description"], must_define = ["executable"] }
verified_by = "tests/release.rs"   # author checks this is wired; CI runs it
```

`temper` guarantees the slot is filled and the judge is present and wired. The
judge (a test, a CI job, an eval) guarantees the behavior, at runtime. Neither
guesses; neither is `temper`.

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

## Decision: a contract is identified by its path/role, not an internal name

**Chosen:** a `Contract` carries **no required internal `name`**. Its identity is
*where it lives* — the file path a role binds (`contract = "contracts/skill.anthropic.toml"`)
or the inline block under a role. A display label for diagnostics derives from the
file stem (`skill.anthropic`). **Rejected:** a required top-level `name` field on
every contract. The contract examples above (Roles and matching; Templates)
identify contracts by path or inline binding and carry no internal name — a
required name is redundant with the path and forces ceremony into a data file
that is otherwise pure clauses. (This resolves the `(contract-name-field)` fork:
the curated `contracts/skill.anthropic.toml` rightly has no `name`; the model's
required-`name` was code drift — relax it to optional, derived from the stem.)
