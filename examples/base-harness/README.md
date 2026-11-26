# base-harness

A starter harness authored as a temper program, whole: the Claude Code
artifacts (memory, rules, skills, hooks) and a documentation corpus are
typed members of one program, their cross-references are edges the gate
resolves, and the files on disk are projections. The corpus is
spec-authoritative: documents are declared intent, and the code under
`src/` reconciles toward them (see `docs/decisions/authority-arrow.md`).

The governed code is a toy on purpose, a three-file checklist summarizer:

```sh
$ node src/main.js TODO.md
2/3 done
```

## Starter plus customization

The program is `@dtmd/temper/starter` hydrated, then grown. `starter()`
supplies the mechanics of using temper well, which no project should
re-author: the typecheck `emit` runs before the program, the three gate
hooks (the session-start report and the guard's two edges), and the
harness reference at `.claude/skills/temper/`, whose topic files derive
from what the program declares. Its one required argument is the
project file the typecheck reads, because that path is this project's.
`.temper/harness.ts` spreads the result into `harness()` and adds its own
members beside the starter's:

```ts
const base = starter({
  typecheck: { project: input(import.meta.url, "./tsconfig.json") },
  reference: { description: "How to operate the temper gate over this repository's …", satisfies: ["governance"] },
});
const program = harness({ ...base, require: { … }, members: [...base.members, memory_CLAUDE, …] });
```

The worked customization is the reference: its description is hydrated
with this project's own trigger, and it names the domain it fills. Its
body and topics are still derived, so declaring a kind or a domain grows
the reference without anyone writing to it. Everything else here is the
project's content: the domains, the memory, the rules, the skills, and
the documentation corpus.

What the harness demonstrates:

- **The harness is organized domain first, mechanism second.** Five
  domains (conduct, orientation, standards, operations, governance) are
  declared as requirements in `.temper/harness.ts` on day one, and every
  member names the domain it fills with a `satisfies` entry. Growth is
  additive: new knowledge joins an existing domain. Conduct, orientation,
  and governance are floored (`required: true`); remove the conduct rule
  and re-emit, and `temper check` fails:

  ```
  requirement.count

    x requirement `conduct` is unfilled: no artifact declares a `satisfies` link naming it
  ```

- **One fact, one home.** Commands and expected outputs that more than one
  surface states live once in `.temper/facts.ts`; the memory's map and the
  verify skill's procedure interpolate the same constants, so one edit
  moves every projection and the surfaces cannot contradict each other.
  The gate hooks state no command at all: they come from the starter, and
  the engine renders the command each runs into the settings manifest.
- **A skill can be path-gated, and its claims rest on declared inputs.**
  `verify-summary` declares `paths: ["src/**", "TODO.md"]`, so it
  registers only when the work touches what it knows how to verify, and it
  declares the entrypoint it tests, `src/main.js`, as an input the lock
  fingerprints. Delete `src/main.js` and `temper emit` refuses:

  ```
  Error: temper::drift::dangling_import

    × member `skill:verify-summary` references `src/main.js`, resolving to `./
    │ src/main.js`, which does not exist — a dangling reference
  ```
- **Claims about code go stale, and the gate says where to look.** Each
  system declares the `src/` files implementing it as inputs. Change
  `src/scan.js` without re-emitting and `temper check` names the member
  whose claims to re-verify (advisory; `--deny-advisories` fails on it):

  ```
  root.fresh

    ! declared input target `src/scan.js` (referenced by `system:scanner`) no longer matches the lock's fingerprint — the target changed and `emit` has not run; re-verify the member's claims against
    | the input, then re-emit
  ```

What the documentation corpus demonstrates:

- **Documents compose from declared members.** A system's invariants, a
  flow's steps, and a decision's rejected alternatives are embedded
  members (`.temper/kinds.ts`), each with its own markdown rendering;
  `emit` composes the documents under `docs/` from them. Editing a
  rendered document is drift; the discipline is construction, not
  convention.
- **Derived renderings replace authored duplicates.** A flow's steps each
  carry an edge to the system they happen in; the participants line in the
  projected document is rendered from the steps, so it can never disagree
  with them. There is no participants field to forget.
- **Lifecycle is positional and typed.** Superseding a decision is the
  `supersede()` operation: the replaced ruling's record lands in
  `docs/decisions/superseded/`, where the successor edge is required by
  the field's own type.
- **Both content faces, deliberately.** `docs/glossary.md` is the one
  layout source, the authored home for prose-first content, read under its
  declared layout, each term an addressable member.

## Run it

From this directory, with the `temper` binary installed and the SDK built
(`pnpm -C ../../sdk install && pnpm -C ../../sdk build` from here, once):

```sh
npm -C .temper install
temper emit
temper check
temper explain scanner
```

`explain scanner` narrates the member's place in the graph, including the
invariants its own body composes and the step's leaf that points at it:

```
Edges out (the resolved references it declares and the members its body composes, the exact set the gate ranges over):
  • it contains `system:scanner/invariant/done-is-exact` (invariant) — an embedded member its body composes
  • it contains `system:scanner/invariant/ignored-never-guessed` (invariant) — an embedded member its body composes
Edges in (the resolved references that point at it and the host whose body composes it):
  • leaf `flow:summarize/step/scan/in` points at it via its `mention` field
```

Standalone (outside this repository), replace the `file:../../../sdk`
dependency in `.temper/package.json` with the published `@dtmd/temper` and
use the installed `temper` binary.
