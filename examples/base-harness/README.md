# base-harness

A base harness authored as a temper program, whole: the Claude Code
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

## Patterns plus customization

The program is `@dtmd/temper/patterns` hydrated, then grown.
`defaultPatterns()` supplies the mechanics of using temper well, which no project should
re-author: the typecheck `emit` runs before the program, the three gate
hooks (the session-start report and the guard's two edges), and the
harness reference at `.claude/skills/temper/`, whose topic files derive
from what the program declares. Its one required argument is the
project file the typecheck reads, because that path is this project's.
`.temper/harness.ts` spreads the result into `harness()` and adds its own
members beside the patterns':

```ts
const base = defaultPatterns({
  typecheck: { project: input(import.meta.url, "./tsconfig.json") },
  reference: { description: "How to operate the temper gate over this repository's …", satisfies: ["governance"] },
  playbooks: [playbook_corpus],
});
const program = harness({ ...base, require: { … }, members: [...base.members, memory_CLAUDE, …] });
```

Two shapes carry a worked customization. The reference's description is
hydrated with this project's own trigger, and it names the domain it
fills. Its body and topics are still derived, so declaring a kind or a
domain grows the reference without anyone writing to it.

The second is a playbook, the central pattern: one area of guidance
emitted across the surfaces Claude Code loads for it.
`.temper/playbooks/corpus.ts` hydrates it for the documentation corpus
and the code it governs:

```ts
export const playbook_corpus = playbook({
  name: "corpus",
  description: "Procedures for the documentation corpus and the code it governs: …",
  trigger: { paths: ["docs/**", "src/**", "TODO.md"] },
  satisfies: { rule: ["standards"], skill: ["operations"] },
  facts: span(`# The corpus is the program …`),
  topics: [
    { name: "verify-summary", when: "a change under `src/` or `TODO.md` needs confirming", prose: … },
    { name: "supersede-a-decision", when: "a standing ruling under `docs/decisions/` is replaced", prose: … },
  ],
});
```

The module supplies the content: the name, the description, the area's
paths, the domains it fills, its standing facts and one procedure per
topic. The shape supplies the structure. The facts become the path rule
`.claude/rules/corpus.md`, which closes on a typed pointer to the skill.
The skill `.claude/skills/corpus/SKILL.md` routes to one topic file per
procedure beside it, each link a typed mention. The skill stays listed
before any file is read, so a question asked first can still find it.
Everything else here is the project's content: the domains, the memory,
the conduct rule, the `grow-harness` skill, and the documentation corpus.

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
  playbook's verify topic interpolate the same constants, so one edit
  moves every projection and the surfaces cannot contradict each other.
  The gate hooks state no command at all: they come from the patterns, and
  the engine renders the command each runs into the settings manifest.
- **A procedure's claims rest on declared inputs.** The `verify-summary`
  topic links the scanner and renderer documents by typed mention, and
  each system declares the `src/` files implementing it as inputs the lock
  fingerprints. Delete `src/scan.js` and `temper emit` refuses:

  ```
  Error: temper::drift::dangling_import

    × member `system:scanner` references `src/scan.js`, resolving to `./src/
    │ scan.js`, which does not exist — a dangling reference
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
