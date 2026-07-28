## Surface

`requirements()` (src/read.rs:1418-1438) takes `name: Option<&str>` and
dispatches `Some(name) => requirement_detail(...)` / `None =>
roster_overview(...)`. `roster_overview` (src/read.rs:1442-1476) is the
forward roster view — every requirement, satisfier set, coverage state —
and the module's own header (src/read.rs:9-10) still narrates it as live:
"[`requirements`] walks it in reverse (the roster → each requirement's
satisfier set + coverage state, and with a name the blast radius…)".

`requirements()` has exactly one call site: `explain()`'s
`Species::Requirement(name) => requirements(..., Some(name))`
(src/read.rs:340-347) — always `Some`. `Species::Requirement` is only ever
constructed by `resolve()` (src/read.rs:210-248) from a target string that
already matched a requirement name in the roster, so it never carries an
absent name either. The CLI's only entry point, `Command::Explain { target:
String }` (src/main.rs:195-201), takes one mandatory positional — there is
no "no target" invocation. `roster_overview` is therefore unreachable in
production; only reachable via `rg`-verified zero external callers
(`rg -n "roster_overview" src/ tests/` — the definition and its one call
site, nothing else).

Two specs describing `explain`'s contract (`specs/model/contract.md`
"Read verbs", `specs/model/pipeline.md` "Read verbs") both describe it as
narrating one *named* target (member/requirement/kind/leaf) — neither
mentions a bare "list the whole roster" mode, so this doesn't read as a
spec-mandated capability with a missing wire-up.

This looks like residue from the pre-consolidation four-verb CLI (the
module header's own line 250-252 says the single `explain` verb replaced
`why`/`requirements`/`impact`/`context` as separate CLI spellings) — a
standalone `requirements` command likely supported a no-name "list all"
mode that never got a path back in after consolidation into `explain
<target>`.

The design call plan itself shouldn't make silently: delete
`roster_overview` + `Option<&str>` (and fix the stale header line 9-10) as
dead code, **or** the "roster → each requirement's...coverage state"
capability was meant to stay reachable and needs a CLI decision (e.g. does
`explain` grow a no-target / `requirements:` bare-listing spelling). Either
way, `roster_overview`'s own per-requirement `satisfiers_of` scan
(src/read.rs:1605-1641) re-walks the whole corpus once per requirement in
the roster — the same "Cost scale is hoisted" class the sibling entry
REQUIREMENT-DETAIL-MEMBER-INDEX-REHOIST (477895ab) just fixed elsewhere in
this file — but that's moot if the branch is deleted, so it rides whichever
way this resolves rather than filing as its own entry.

## Observed at

477895ab

## Suggested consolidation

Delete `roster_overview` and narrow `requirements()` to take `name: &str`
directly (drop the `Option`), and cut the stale "the roster → each
requirement's…" clause from the module header (src/read.rs:9-10) to match
— unless a human wants the no-target roster listing restored, in which case
it's a `Command::Explain` CLI-grammar decision (optional target, or a
`requirements:` bare spelling), not a plan call to make silently.
