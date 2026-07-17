# 0033 — a hold is named debt, and four come due

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(clause-vocabulary-holds)` fork: four shipped default-contract
absences are decidable, documented, and unexpressible — a member the
platform refuses outright passes `temper check` clean — and the corpus
sanctioned only one absence class (undecidable). The human's fence check
settled the frame: the closed vocabulary was built against *invention*
and *guessing* (invariant 2; decision 0011, which by name rejected
"minimal vocabulary, extended on demand"), never against documented
validation — the "strictest documented profile" stance is 0011's
descendant, and these four are the growth it mandates, arriving through
the ceremony it prescribes. The linting worry is guarded by a different
plank: validation enforces documented facts, linting enforces opinions,
and the author-facing pattern language stays refused. Session-argued,
human-ruled 2026-07-16; libraries verified live (serde_json_path, RFC
9535, CTS-tracked; crates.io retrieved 2026-07-16).

## Decision

**The absence taxonomy gains one word.** A documented, decidable rule
the vocabulary cannot yet spell is a **hold**: named in the contract's
header at the point of enforcement together with the widening that
would close it, keyed in the plan ledger, test-pinned where the gap is
load-bearing. A hold with no named closing widening is not a hold. One
sentence in `builtins.md`, not a doctrine.

**Four widenings close the four current holds:**

1. **`type` accepts a set of lattice kinds** — six documented
   `string|array(|object)` manifest fields become gateable; a
   one-element set is today's behavior exactly (0029's move).
2. **Field addressing widens to a declared RFC 9535 subset** — name
   segments and `[*]` only, riding `serde_json_path` (which joins the
   sanctioned crate set); `[*]` is the each-grain over elements. A path
   using anything beyond the subset — filters, slices, recursive
   descent — is rejected at admissibility: the RFC engine is hidden
   mechanics, exactly as `regex` hides under `allowed_chars` and
   `globset` under `glob-valid`. `owner.name` and `plugins[*].source`
   become spellable; extraction retains the nested values the paths
   evaluate over (the mechanism is the entry's — the row schema needs
   no delta, `field` is a string on both faces).
3. **`closed-keys`** — a clause declaring the kind's *already-declared*
   key set (the `required`/`optional` rows emit writes) exhaustive: an
   undeclared key is a finding. The allow-list is consumed, never
   authored twice; `plugin-manifest`'s `--strict` bar becomes spellable.
4. **`shape`** — a predicate over a **closed enum of engine-implemented,
   doc-cited shapes**, opening with the two documented ones: the skill
   name's hyphen rules (no leading, trailing, or consecutive) and
   no-XML-tags-in-description. This is the "later wave under their own
   design" 0022 reserved; `regex` stays the hidden mechanics, and no
   author ever writes a pattern.

**The platform's published JSON Schema becomes a drift oracle, never an
engine**: a fixture test diffs the shipped clause coverage against
json.schemastore.org/claude-code-plugin-manifest.json (the `$schema`
value in the platform's own docs, retrieved 2026-07-16), so the
vocabulary's lag behind the platform's validation bar is a number a
human reads, not an anxiety argued from scratch each time.

## Rejected

- **Zero tolerance** (a decidable documented rule must ship with its
  kind or the kind must not claim the profile): would have blocked the
  plugin kinds on four vocabulary decisions; the hold names the window
  instead of forbidding it.
- **A schema-validation predicate** (`jsonschema` crate over the
  published schema, closing widenings 1–3 in one word): one clause
  means one severity and no per-rule guidance for every violation
  inside it — a generic validator where the model wants a teaching
  gate; adopted as the oracle instead.
- **Author-facing JSONPath** (the full RFC as clause surface): filters
  and recursion are a pattern language — the open lint platform
  invariant 2 exists to prevent. Refused for the third time on the
  record (invariant 1; 0022; here).
- **RFC 6901 pointers** (already inside `serde_json`): no wildcard, so
  the each-grain over `plugins[]` — half the demand — is unsayable.
- **Un-flattening `FeatureValue` as the addressing fix**: touches the
  `type` predicate's kind-preservation contract and every consumer,
  where path evaluation over retained values needs no row change.
- **A hold framework heavier than one sentence**: the category
  question's "strange posture" was real — nobody argues documented
  rules shouldn't be checked; the residue is bookkeeping, and it gets
  one sentence.

## Consequences

`builtins.md` gains the hold sentence — same commit, this record. The
predicate vocabulary itself stays code-authoritative (equal
representation): the four widenings land as plan-derived entries with
fresh raw cites at encode time, `serde_json_path` enters `Cargo.toml`'s
sanctioned set (the CLAUDE.md stack list is a `.temper` projection —
its update is a `chore(harness)` act riding the entry), the oracle test
lands beside the plugin-manifest contract, and the four contract-header
hold notes discharge as each widening ships. The
`(clause-vocabulary-holds)` fork resolves whole; its record deletes.
