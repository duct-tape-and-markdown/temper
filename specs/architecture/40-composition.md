# Composition — the assembly

The **assembly** is where a harness becomes one thing. `harness()` takes the
whole — the members, the demands over them, the residual settings, the
reachability dial — as **one typed value**, authored in the SDK
(`20-surface.md`). Like every SDK type it erases at the seam: the engine never
sees the constructor, only the plain data it compiles to, riding the lock as
rows. This spec owns that value and how the parts compose.

```
Harness = members · expect · require · settings · reachability
```

There is no second authoring surface for the assembly — no configuration
dialect, no merge precedence between files. Composing two partial harnesses is
ordinary code (a spread, a concat, a function), typed at the keystroke. Every
assembly fact has exactly one home; what ships is still one inert set of
committed artifacts plus the lock (`00-intent.md`, the SDK Decision). Because
the assembly is code *producing* declared data, harness **families** cost the
model nothing: a monorepo maps its workspace list to per-package members; a
scenario baseline is a function instantiated with parameters.

Absent any authored assembly, the engine's embedded default program — the
compiled built-in floor (`50-distribution.md`) — still gates the harness kinds,
so SDK-less checking never runs ungated.

## `members` — the roster is the import graph

The member list is the assembly's imports. An authored member module nobody
lists is visible shelf stock — the toolchain flags the unused import before
the gate does; a listed member that fails to resolve never compiles. Members
themselves (their fields, their prose) are `20-surface.md`'s; kinds and
genres, the values that construct them, and the authoring postures are
`15-kinds.md`'s. The assembly only *contains* them.

## `expect` and `require` — the two demand forms

All demands are stated in one vocabulary — `Clause` and `Requirement`,
owned by `10-contracts.md` — under two quantifiers:

- **`expect` is universal**: every member of a kind owes these clauses. It is
  keyed by the kind **value** (an import, never a string — identity travels by
  import), and it maps each kind to a clause array.
- **`require` is existential**: the harness must **contain** a fill. Each
  entry is a `Requirement` (means · kind · required · clauses? · verifiedBy?
  — the shape, the set-scope clauses, and the posture-vs-measurement split
  are `10-contracts.md`'s), keyed by a string the fill's `satisfies` names.

A genre is a kind at the block locus (`15-kinds.md`), so both forms reach
block-grain members unchanged: `expect` can key a genre; a `require` entry can
be filled by a block inside a host document.

The pair in use:

```ts
import { skill, skillFloor } from "@dtmd/temper/claude-code";

export default harness({
  members: [...skills, ...rules, deployHook],
  expect: {
    [skill.key]: [...skillFloor, descriptionUnder(1024)],
  },
  require: {
    "deploy-checklist": requirement({
      means: "a release is never cut without the checklist skill",
      kind: skill,
      required: true,
      verifiedBy: "ci:deploy-checklist-eval",
    }),
  },
});
```

`expect` here says *every* skill meets the floor plus one local clause;
`require` says *some* skill must exist whose `satisfies` includes
`"deploy-checklist"`, absence blocks the gate, and the behavioral half is
delegated to a wired verifier. Universal shape, existential presence — the two
halves of `00-intent.md` law 2's "the author declares a contract."

## Binding is implicit — a floor is a clause array

A **floor** is nothing but an exported clause array. `@dtmd/temper/claude-code`
exports one per built-in kind; a project exports its own the same way.
Adoption is the import; the spread inside `expect` is the entire binding;
overriding is composing the array — append a clause, filter one out, wrap one
with a different severity. There is no binding record, no package-to-kind
table: the question "which contract governs this kind?" is answered by
reading the `expect` entry, and nowhere else.

## Decision: `satisfies` keys are strings, resolved at emit

**Chosen:** a member's `satisfies` names a requirement by string key; the keys
are resolved when the harness is assembled, and the graph flags a dangling key
(a `satisfies` no requirement declares, a requirement no member fills) as a
finding. This is the one place in the model where identity travels by name
rather than by import — deliberately: the assembly stays **downstream of
members**. A member module is a leaf other harnesses can also list.
**Rejected:** requirement values imported into members — it inverts the
dependency direction, making every member module depend on the assembly that
lists it, so no member could be shared across harnesses and the member/assembly
layering collapses into a cycle. The string key costs one dangling-reference
check the graph already knows how to make.

## `settings` — the residual

`settings` holds the harness-level fields that have no member home (e.g.
`autoMemoryEnabled`). Emit projects them into the settings artifact alongside
the folded member registrations (`20-surface.md`; emit is total and members
are the only other source). The list only shrinks: as members absorb their
registrations, fields leave it. Ruled 2026-07-04, narrowing
`(settings-residual)`: emit owns the whole **project-scope** settings
artifact; personal preferences belong to the local/user scopes the format
authority already layers, with permission rules merging across scopes
(managed > CLI > local > project > user —
code.claude.com/docs/en/settings, retrieved 2026-07-04). The residual's
exact typed field list stays open with the settings member design.

## `reachability` — the dead-registration dial

Registration edges, the world node, and reachability-from-the-world are
`45-governance.md`'s predicate. The assembly holds the **opt-in dial**: whether
a member unreachable from the world — registered by nothing, embedded by
nothing — is a finding at all, and at what severity. That is the assembly's
call, never the engine's taste (`00-intent.md` law 2). Absent the dial,
dead registrations produce no findings.

## Decision: one authored assembly, no configuration dialect

**Chosen:** the assembly is authored SDK-side as one typed value; its facts
ride the lock as rows; the same declaration is the single source of the editor
schema `temper` emits (`50-distribution.md`). **Rejected:** (a) a hand-authored
`temper.toml` — or its decomposition into `roster.toml` / `bindings.toml` — a
permanent second authoring surface restating in tables what the SDK states in
types, each dialect demanding its own docs, keystroke channel, and
format-preserving patcher; (b) kind-binding tables (`KindBinding` records,
package-to-kind maps) — binding is already the spread in `expect`, and a table
is a second spelling of an import; (c) a package noun with layering machinery —
defaults ⊕ authored ⊕ local precedence rules re-derive at read time what the
author can state directly: composition is code, and precedence is the
evaluation order the author writes.

OPEN: the committed-plus-gitignored personal-override split the dialect era
carried has no stated spelling in the one-value model — proposed slug
`(local-overrides)`.

## The authoring loop

1. **Author** members (`20-surface.md`) and list them.
2. **Adopt** floors in `expect`; compose local clauses over them.
3. **Declare** `require` entries and wire their verifiers.
4. **Check** — tsc at the keystroke; the engine's judges at the gate
   (`45-governance.md`).
5. **Emit** — the compile into the committed artifacts and the lock
   (`20-surface.md`), byte-reproducible, content-faithful.

The loop is `00-intent.md` law 7 — compose everything; gate the decidable —
in use, and law 6's fearless refactoring is why it stays cheap to re-run.

## Scope boundary

This spec owns the assembly value and its five fields. The clause and
requirement vocabulary is `10-contracts.md`'s; kinds, genres, loci, and the
authoring postures are `15-kinds.md`'s; members, emit, and the lock are
`20-surface.md`'s;
the graph, the judges, and the registration predicate are `45-governance.md`'s.
A landscape is just more kinds (`30-landscapes.md`) — nothing here is
harness-specific except the built-in floor the example imports.
