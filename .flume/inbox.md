<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->




## explain kind: "a `intent` member" — an article chosen before a kind name it cannot know — observed at 4ce5ee72

From cascade-integrations' probe of d935b088: the address-form strand
renders "a `intent` member". An English article picked ahead of a
backticked identifier is wrong for every vowel-initial kind name and
cannot be made right by choosing; phrase the strand without one (a member
of `intent`, or `intent` member) wherever `narrate_kind` composes an
article before a kind name. Cosmetic; ride the next entry that touches
`narrate_kind` or the posture sweep, whichever comes first.

## graph.acyclic ranges over declared field edges, and the spec scopes it to the import relation — observed at d9a34e39

`contract.md` "well-formedness" makes acyclicity a fixed check over the
**import relation** only: "the import relation is well-founded; a cyclic
graph makes evaluation itself ill-defined". The engine checks something
else. `gate.rs:553-556` passes `graph::resolved_edges(&edges, …)` (every
resolved **declared field** edge) to `graph::acyclic` (`graph.rs:254`).
`tests/graph.rs:306` `a_cyclic_reference_graph_fails_the_run` pins a
`routes_to` rule↔skill cycle as a failure. The scope is left over from
before the kernel: the check shipped on 06-30 (f973192d) against the old
wording, "the reference graph has no cycle". The 07-06 kernel re-found
(e842a32d) narrowed the spec and never reconciled the code. No decision
since has asked for field-edge acyclicity (0001 rejects acyclicity *as a
clause*, which is the same import-relation reason). Nothing downstream
needs it either. `degree` counts locally, route resolution works one edge
at a time, and `live_members` terminates on its visited set, so a
field-edge cycle leaves evaluation well-defined. Because the check is
fixed, it can't be dialed. A consumer modeling a request loop (view →
signal → path → state → view) was refused
(`graph.acyclic: … path p87 → state forgotpassword → view forgotpassword →
signal _resetpassword → path p87`) and had to demote the closing edge out
of the edge enumeration.

There is also the reverse half. The relation the spec does name, `@import`
directive edges (`classify_directives`), never reaches `acyclic`. They
feed only `reachable`. Whether a circular `@import` is ill-defined at
runtime is an external fact about Claude Code (UNVERIFIED; cite
code.claude.com/docs/en/memory before encoding). Route that half to an
open question if the docs are silent. Don't invent it. For the field half,
narrow `acyclic` to the import relation and rewrite the pinned test so a
field cycle passes. If someone later wants a DAG over field edges, the
spine rule makes that a clause (opt-in, dialable), never well-formedness.
Plan should not invent that clause.

**Evidence status: observed at 0.0.18** (`temper --version` →
`temper 0.0.18`, in a copy of the consumer's prototype). Restoring
`edgeFields: [{ field: "handled-by", to: ["path"] }]` on `signal` and
running `temper emit && temper check .` adds exactly one finding over the
as-is run: `graph.acyclic` "x the harness reference graph contains a cycle:
path `p87-reset-password` → state `forgotpassword` → view `forgotpassword`
→ signal `_resetpassword` → path `p87-reset-password`". The as-is run
already exits 1 on required clauses, so a minimal repro isolates the exit
code. Two custom kinds whose edge fields point at each other, with members
`one` ↔ `two`, give `check` exit 1 and "x the harness reference graph
contains a cycle: a `one` → b `two` → a `one`". Dropping one edge field
gives exit 0. The same repro carries a memory ↔ rule `@import` cycle
(`CLAUDE.md` imports `@.claude/rules/r.md`, which imports
`@../../CLAUDE.md`). With the field cycle removed, `check` exits 0 with
no `graph.*` finding and no unbacked-import finding. The import cycle
resolves and goes unjudged, as the code reads. What Claude Code does at
load time with that cycle stays UNVERIFIED.
**Dependency (human-ruled 2026-09-22):** the fix for the consumer's
`membership` workaround waits on this one. That fix splits entry paths into
their own kind and makes `handled-by` an edge to it, which closes the cycle
view → signal → entry path → state → view again, and today's `acyclic`
refuses it. Land this first. The consumer keeps its name-in-its-own-field
workaround until then, and its prototype is ready to test the fix.

## degree has no field filter; contract.md's by-incidence selector is spec'd and unbuilt — observed at d9a34e39

`contract.md` "selection" at HEAD defines **by incidence** as "the edges at a
member, filtered by field and direction". `Predicate::Degree`
(`src/contract.rs:328`) carries only `incoming`/`outgoing` bounds.
`graph::degree` (`graph.rs:307`) counts every resolved field edge plus
every mention edge, over deduped `(from, to)` arcs. So the field is
dropped before counting, and two fields between one pair count once. The
SDK's `degree()` (`sdk/src/contract.ts:253`) has no field parameter. The
filter half of the selector is missing at every layer. A consumer who
needed "incoming from `writes` edges only" had to choose edge directions so
each count meant one thing. Decision 0052 (accepted 09-08, in the working
tree and not yet committed at this sha) widens the filter to a **field
set** and names `Predicate::Degree gains a field-set filter` as a derived
entry. If 0052 has landed when this routes, merge into that entry rather
than filing a second one. This is its second driver. Arc dedupe must be
per filtered set, so a count over {a, b} counts an a-edge and a b-edge to
one target as two.

**Evidence status: observed at 0.0.18.** In the consumer's prototype as
is, `temper check .` (exit 1) with `channel` bound to
`degree({ incoming: { max: 1 } })` reports "kind `channel` bounds incoming
degree to [0, 1], but `ierror` has 3". The 3 is two `path.writes` edges
(`p16-lookup`, `p20-swap`) plus one `effect.clobbers` edge
(`e7-password-reset`), so both fields are counted. Strict `tsc` against
the published 0.0.18 types rejects
`degree({ incoming: { max: 1 }, field: "writes" })` with TS2353
"'field' does not exist in type '{ incoming?: …; outgoing?: … }'". The
arc dedupe also reproduces. One member reaching one target through two
fields (`to`, `also`) under `degree({ incoming: { max: 0 } })` gives
`check` exit 1 with "kind `b` bounds incoming degree to [0, 0], but `two`
has 1".

## text`` cannot take a computed string — observed at d9a34e39

Second occurrence. `examples/base-harness/.temper/kinds.ts:29` hand-builds
`{ kind: "text", template, mentions: [], includes: [] }` as `span()` for a
computed prose line. An external consumer on 0.0.18 did the same. The tag's
interpolations are references by design, so the gap is a supported
constructor for a plain prose span. Right now authors reach into the
`Text` shape, which is an SDK internal. Friction only; route as the loop
sees fit (it may want an `authoring.md` line first).
