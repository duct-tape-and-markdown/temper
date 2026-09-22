<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->





## A clause bound where no judge reaches it is accepted and never judged: set predicates in a `when` body, member predicates in a requirement — observed at f8bb2e07

**Observed** on current main (f8bb2e07), in a scratch integration test that
reuses `tests/graph.rs`'s own fixtures (rule `style` routes_to skill
`standards`; nothing points at `style`):

| clause bound on | result |
|---|---|
| kind `rule`: `when(type(routes_to) = string)` → `required(zz_missing_field)` | fires (the guard holds) |
| kind `rule`: same guard → `degree(incoming ≥ 1)` | **silent, `check` exits 0** |
| kind `rule`: `degree(incoming ≥ 1)` unguarded | fires |
| requirement `gate` (style opts in): `required(zz_missing_field)` bare | **silent, exits 0** |
| requirement `gate`: `when(…)` → `required(zz_missing_field)` | **silent, exits 0** |

The first cell is also field evidence: a consumer on 0.0.18 wrote
`when(enumOf("status",["live"]), [clause(degree({incoming:{min:1}})),
clause(membership("handled-by","request-entry"))])`. It passed while tamper
tests violated both. It worked around the problem by splitting the kind
(`retired-signal`).

**Read** (why): a `when` body goes through the per-member `decide` table
(engine.rs:1237 → `evaluate`). For `degree`/`membership`/`count`/`unique`/
`kind`/`extent{whole}`, that table returns `Indeterminate`, and `evaluate`
turns that into silence (engine.rs:845). Its comment says the arm is
"unreachable on an admissible run: admissibility fences every producer", but
`when_restrictions` (engine.rs:170) fences guards and nesting, never set
predicates in a body. The set judges (`engine::judge`, `graph::degree`,
`graph::mention_reachable`) read only a selection's top-level clauses. And
`engine::validate` runs only over kind contracts (gate.rs:44), so an opt-in
selection's member-grain clauses reach no judge at all.

**What the spec already decides** (no new intent needed for either):
- Set predicates in a `when` body: **refuse at admissibility.** 0041 binds a
  body at the guard's *element* ("each element the guard's path locates is
  judged independently"). A selection predicate has no element to range over.
  Evaluating it under the guard would narrow a selection by a field value,
  which `contract.md` "selection" forbids: "narrowing is an each-grain
  clause over a selection, never a second selector". The consumer's need is
  lifecycle as kind partition, the settled `(lifecycle-encoding)` precedent
  (`docs/horizons.md`, `(base-harness)`). Guarded selection is a vocabulary
  question for a decision, not something to build.
- Member-grain predicates bound through a requirement: **judge them.**
  `contract.md` "selection": "There is no separate universal/existential
  machinery: the quantifier is the clause's grain." A `required` means the
  same bound by kind or by opt-in.
- Either way, `decide`'s set-predicate arm should fail loud (an internal
  error naming the predicate), never return silence. A floor that claims
  unreachability is how both holes shipped.

**Label collision: a defect against 0049, spelling ruled.** Two `when`
guards over one field collide on `clause.label-collision`: `clause_label`
(contract.rs:84, stamped at drift.rs:1594) and the SDK's `clauseField`
compile `owner.predicate.field`, and for a `when` the field is the guard's.
So `when(status ∈ {live})` and `when(status ∈ {retired})` both label as
`<kind>.when.status`. 0049 already decides the rule: "A compiled label is a
clause's address and carries every argument that distinguishes the clause",
and it folded `section_contains`'s heading and marker in on the same ground.
The spelling is ruled by John, 09-22: a `when` label carries its guard's
value set, sorted and `+`-joined (the `require_sections` precedent), after
`=`: `signal.when.status=live`, `signal.when.status=live+retired`,
`skill.when.routes_to=string`. It is deterministic, legible, and still a dial
address. Rust and the SDK must compile the same string; the
builtin-lock frozen test holds them together. Every existing `when` label
changes spelling, so a dial entry naming an old `…when.<field>` label stops
matching, and the release note must say so.

**Relation to DEGREE-FIELD-SET-FILTER:** no shared fix. The fixes do share
files (`src/engine.rs`, `src/contract.rs`), so order them rather than run
them in parallel.
